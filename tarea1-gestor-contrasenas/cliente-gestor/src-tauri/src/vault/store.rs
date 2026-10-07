//! Archivo de bóveda. Por dentro es SQLite, pero en disco el archivo entero va
//! cifrado (cabecera `BOV2` + XChaCha20-Poly1305) con la clave Argon2id de la
//! maestra. Una bóveda vieja, en SQLite sin cifrar, se reescribe al abrirla.
//! Este módulo no escribe secretos en logs.

use std::fs;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection, DatabaseName, OptionalExtension};

use crate::crypto::cipher;
use crate::crypto::kdf::{self, ParametrosKdf};

const VERIFICADOR: &[u8] = b"boveda-v1";
const MAGIC_BOV2: &[u8] = b"BOV2";
const CABECERA_BOV2: usize = 32;

const ESQUEMA: &str = "
CREATE TABLE meta (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    salt BLOB NOT NULL,
    memoria_kib INTEGER NOT NULL,
    iteraciones INTEGER NOT NULL,
    paralelismo INTEGER NOT NULL,
    verificador BLOB NOT NULL
);
CREATE TABLE credenciales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sistema TEXT NOT NULL,
    usuario TEXT NOT NULL DEFAULT '',
    secreto BLOB NOT NULL,
    notas BLOB NOT NULL,
    categoria TEXT NOT NULL DEFAULT '',
    creado_en INTEGER NOT NULL,
    actualizado_en INTEGER NOT NULL,
    vence_en INTEGER,
    favorito INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE politicas (
    sistema TEXT PRIMARY KEY,
    modo TEXT NOT NULL,
    longitud INTEGER NOT NULL,
    minusculas INTEGER NOT NULL,
    mayusculas INTEGER NOT NULL,
    digitos INTEGER NOT NULL,
    simbolos INTEGER NOT NULL,
    regex TEXT NOT NULL DEFAULT '',
    historial INTEGER NOT NULL DEFAULT 0,
    dias_validez INTEGER NOT NULL DEFAULT 0,
    categoria TEXT NOT NULL DEFAULT ''
);
CREATE TABLE historial (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    credencial_id INTEGER NOT NULL,
    secreto BLOB NOT NULL,
    creado_en INTEGER NOT NULL
);
PRAGMA user_version = 3;
";

#[derive(Debug)]
pub enum ErrorBoveda {
    YaExiste,
    NoExiste,
    MaestraIncorrecta,
    TotpRequerido,
    TotpInvalido,
    NoEncontrada,
    DatoInvalido(&'static str),
    ArchivoInvalido,
    Sqlite(rusqlite::Error),
    Cripto,
    Kdf,
    Io(std::io::Error),
}

impl std::fmt::Display for ErrorBoveda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::YaExiste => write!(f, "Ya hay una bóveda en esa ruta. Ábrela con la contraseña maestra."),
            Self::NoExiste => write!(f, "No hay una bóveda en esa ruta."),
            Self::MaestraIncorrecta => {
                write!(f, "Contraseña maestra incorrecta, o el archivo fue alterado.")
            }
            Self::TotpRequerido => write!(f, "Esta bóveda pide el código TOTP."),
            Self::TotpInvalido => write!(f, "Código TOTP inválido."),
            Self::NoEncontrada => write!(f, "Esa credencial no está en la bóveda."),
            Self::DatoInvalido(msg) => write!(f, "{msg}"),
            Self::ArchivoInvalido => write!(f, "El archivo no tiene el formato de una bóveda."),
            Self::Sqlite(e) => write!(f, "No se pudo usar el archivo de la bóveda: {e}"),
            Self::Cripto => write!(f, "No se pudo cifrar o leer el secreto."),
            Self::Kdf => write!(f, "No se pudo derivar la clave de la bóveda."),
            Self::Io(e) => write!(f, "No se pudo preparar la carpeta de la bóveda: {e}"),
        }
    }
}

impl From<rusqlite::Error> for ErrorBoveda {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sqlite(value)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CredencialResumen {
    pub id: i64,
    pub sistema: String,
    pub usuario: String,
    pub categoria: String,
    pub vence_en: Option<i64>,
    pub favorito: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Politica {
    pub sistema: String,
    pub modo: String,
    pub longitud: u32,
    pub minusculas: bool,
    pub mayusculas: bool,
    pub digitos: bool,
    pub simbolos: bool,
    pub regex: String,
    pub historial: u32,
    pub dias_validez: u32,
    pub categoria: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CredencialDetalle {
    pub id: i64,
    pub sistema: String,
    pub usuario: String,
    pub secreto: String,
    pub notas: String,
    pub categoria: String,
}

pub struct Boveda {
    conn: Connection,
    clave: [u8; 32],
    ruta: PathBuf,
    guardar: bool,
}

impl Drop for Boveda {
    fn drop(&mut self) {
        if self.guardar {
            let _ = self.persistir();
        }
        self.clave.fill(0);
    }
}

impl Boveda {
    pub fn crear(ruta: &Path, maestra: &str) -> Result<Self, ErrorBoveda> {
        Self::crear_con(ruta, maestra, ParametrosKdf::boveda())
    }

    pub fn abrir(ruta: &Path, maestra: &str) -> Result<Self, ErrorBoveda> {
        Self::abrir_con(ruta, maestra, None)
    }

    pub fn abrir_con(ruta: &Path, maestra: &str, codigo: Option<&str>) -> Result<Self, ErrorBoveda> {
        validar_maestra(maestra)?;
        if !ruta.exists() {
            return Err(ErrorBoveda::NoExiste);
        }
        let archivo = fs::read(ruta).map_err(ErrorBoveda::Io)?;
        if archivo.starts_with(b"SQLite format 3") {
            return Self::abrir_sqlite_legado(ruta, maestra, codigo);
        }
        if archivo.starts_with(MAGIC_BOV2) {
            return Self::abrir_archivo_cifrado(ruta, &archivo, maestra, codigo);
        }
        Err(ErrorBoveda::ArchivoInvalido)
    }

    fn abrir_sqlite_legado(ruta: &Path, maestra: &str, codigo: Option<&str>) -> Result<Self, ErrorBoveda> {
        let (mut clave, plano) = {
            let conn = Connection::open(ruta)?;
            let (salt, parametros, verificador) = leer_meta(&conn)?;
            let mut clave = derivar(maestra, &salt, parametros)?;
            match cipher::descifrar(&clave, &verificador) {
                Ok(plano) if plano == VERIFICADOR => {
                    asegurar_esquema(&conn)?;
                    if let Err(error) = exigir_totp(&conn, &clave, codigo) {
                        clave.fill(0);
                        return Err(error);
                    }
                    let data = conn.serialize(DatabaseName::Main).map_err(ErrorBoveda::from)?;
                    (clave, data.to_vec())
                }
                _ => {
                    clave.fill(0);
                    return Err(ErrorBoveda::MaestraIncorrecta);
                }
            }
        };
        let conn = match conexion_desde(&plano) {
            Ok(conn) => conn,
            Err(error) => {
                clave.fill(0);
                return Err(error);
            }
        };
        let boveda = Self {
            conn,
            clave,
            ruta: ruta.to_path_buf(),
            guardar: true,
        };
        boveda.persistir()?;
        Ok(boveda)
    }

    fn abrir_archivo_cifrado(
        ruta: &Path,
        archivo: &[u8],
        maestra: &str,
        codigo: Option<&str>,
    ) -> Result<Self, ErrorBoveda> {
        let (parametros, salt, blob) = leer_cabecera(archivo)?;
        let mut clave = derivar(maestra, &salt, parametros)?;
        let plano = match cipher::descifrar(&clave, blob) {
            Ok(plano) => plano,
            Err(_) => {
                clave.fill(0);
                return Err(ErrorBoveda::MaestraIncorrecta);
            }
        };
        let conn = match conexion_desde(&plano) {
            Ok(conn) => conn,
            Err(error) => {
                clave.fill(0);
                return Err(error);
            }
        };
        let (_salt, _parametros, verificador) = match leer_meta(&conn) {
            Ok(meta) => meta,
            Err(error) => {
                clave.fill(0);
                return Err(error);
            }
        };
        match cipher::descifrar(&clave, &verificador) {
            Ok(plano) if plano == VERIFICADOR => {}
            _ => {
                clave.fill(0);
                return Err(ErrorBoveda::MaestraIncorrecta);
            }
        }
        if let Err(error) = asegurar_esquema(&conn) {
            clave.fill(0);
            return Err(error);
        }
        if let Err(error) = exigir_totp(&conn, &clave, codigo) {
            clave.fill(0);
            return Err(error);
        }
        Ok(Self {
            conn,
            clave,
            ruta: ruta.to_path_buf(),
            guardar: true,
        })
    }

    pub fn ruta(&self) -> &Path {
        &self.ruta
    }

    pub fn alta(
        &self,
        sistema: &str,
        usuario: &str,
        secreto: &str,
        notas: &str,
        categoria: &str,
    ) -> Result<CredencialResumen, ErrorBoveda> {
        validar_credencial(sistema, secreto)?;
        let sistema = sistema.trim();
        let usuario = usuario.trim();
        let mut categoria = categoria.trim().to_string();
        let politica = self.obtener_politica(sistema)?;
        if let Some(ref p) = politica {
            validar_secreto(p, secreto, &[])?;
            if categoria.is_empty() {
                categoria = p.categoria.clone();
            }
        }
        let cifrado = cifrar_texto(&self.clave, secreto)?;
        let notas_cifradas = cifrar_texto(&self.clave, notas)?;
        let ahora = ahora();
        let vence_en = politica.as_ref().and_then(calcular_vence);
        self.conn.execute(
            "INSERT INTO credenciales (sistema, usuario, secreto, notas, categoria, creado_en, actualizado_en, vence_en)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![sistema, usuario, cifrado, notas_cifradas, categoria, ahora, ahora, vence_en],
        )?;
        self.persistir()?;
        Ok(CredencialResumen {
            id: self.conn.last_insert_rowid(),
            sistema: sistema.to_string(),
            usuario: usuario.to_string(),
            categoria,
            vence_en,
            favorito: false,
        })
    }

    pub fn listar(&self) -> Result<Vec<CredencialResumen>, ErrorBoveda> {
        let mut stmt = self.conn.prepare(
            "SELECT id, sistema, usuario, categoria, vence_en, favorito
             FROM credenciales ORDER BY favorito DESC, sistema COLLATE NOCASE, id",
        )?;
        let filas = stmt.query_map([], |row| {
            Ok(CredencialResumen {
                id: row.get(0)?,
                sistema: row.get(1)?,
                usuario: row.get(2)?,
                categoria: row.get(3)?,
                vence_en: row.get(4)?,
                favorito: row.get::<_, i64>(5)? != 0,
            })
        })?;
        filas.collect::<Result<Vec<_>, _>>().map_err(ErrorBoveda::from)
    }

    /// Sistemas con al menos una credencial ya vencida. RF-17.
    pub fn sistemas_vencidos(&self) -> Result<Vec<String>, ErrorBoveda> {
        let ahora_ts = ahora();
        let mut sistemas = Vec::new();
        for item in self.listar()? {
            let Some(vence) = item.vence_en else {
                continue;
            };
            if vence >= ahora_ts || sistemas.iter().any(|sistema| sistema == &item.sistema) {
                continue;
            }
            sistemas.push(item.sistema);
        }
        Ok(sistemas)
    }

    pub fn obtener(&self, id: i64) -> Result<CredencialDetalle, ErrorBoveda> {
        let fila = self
            .conn
            .query_row(
                "SELECT id, sistema, usuario, secreto, notas, categoria FROM credenciales WHERE id = ?1",
                [id],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, Vec<u8>>(3)?,
                        row.get::<_, Vec<u8>>(4)?,
                        row.get::<_, String>(5)?,
                    ))
                },
            )
            .optional()?;
        let Some((id, sistema, usuario, blob, notas_blob, categoria)) = fila else {
            return Err(ErrorBoveda::NoEncontrada);
        };
        let secreto = descifrar_texto(&self.clave, &blob)?;
        let notas = descifrar_texto(&self.clave, &notas_blob)?;
        Ok(CredencialDetalle {
            id,
            sistema,
            usuario,
            secreto,
            notas,
            categoria,
        })
    }

    pub fn modificar(
        &self,
        id: i64,
        sistema: &str,
        usuario: &str,
        secreto: &str,
        notas: &str,
        categoria: &str,
    ) -> Result<(), ErrorBoveda> {
        validar_credencial(sistema, secreto)?;
        let sistema = sistema.trim();
        let actual = self.obtener(id)?;
        let politica = self.obtener_politica(sistema)?;
        if let Some(ref p) = politica {
            let mut anteriores = Vec::new();
            if secreto != actual.secreto && p.historial > 0 {
                anteriores = self.secretos_historial(id, p.historial)?;
            }
            validar_secreto(p, secreto, &anteriores)?;
        }
        if secreto != actual.secreto {
            let blob: Vec<u8> = self.conn.query_row(
                "SELECT secreto FROM credenciales WHERE id = ?1",
                [id],
                |row| row.get(0),
            )?;
            self.conn.execute(
                "INSERT INTO historial (credencial_id, secreto, creado_en) VALUES (?1, ?2, ?3)",
                params![id, blob, ahora()],
            )?;
            if let Some(ref p) = politica {
                if p.historial > 0 {
                    self.conn.execute(
                        "DELETE FROM historial WHERE credencial_id = ?1 AND id NOT IN (
                            SELECT id FROM historial WHERE credencial_id = ?1 ORDER BY id DESC LIMIT ?2
                        )",
                        params![id, p.historial],
                    )?;
                }
            }
        }
        let vence_en = if secreto != actual.secreto {
            match &politica {
                Some(p) => calcular_vence(p),
                None => self.vence_de(id)?,
            }
        } else {
            self.vence_de(id)?
        };
        let cifrado = cifrar_texto(&self.clave, secreto)?;
        let notas_cifradas = cifrar_texto(&self.clave, notas)?;
        let cambiadas = self.conn.execute(
            "UPDATE credenciales
             SET sistema = ?1, usuario = ?2, secreto = ?3, notas = ?4, categoria = ?5, actualizado_en = ?6, vence_en = ?7
             WHERE id = ?8",
            params![
                sistema,
                usuario.trim(),
                cifrado,
                notas_cifradas,
                categoria.trim(),
                ahora(),
                vence_en,
                id
            ],
        )?;
        if cambiadas == 0 {
            Err(ErrorBoveda::NoEncontrada)
        } else {
            self.persistir()?;
            Ok(())
        }
    }

    pub fn marcar_favorito(&self, id: i64, favorito: bool) -> Result<(), ErrorBoveda> {
        let marcadas = self.conn.execute(
            "UPDATE credenciales SET favorito = ?1 WHERE id = ?2",
            params![i64::from(favorito), id],
        )?;
        if marcadas == 0 {
            Err(ErrorBoveda::NoEncontrada)
        } else {
            self.persistir()?;
            Ok(())
        }
    }

    /// Copia cifrada con contraseña de transporte. El archivo no lleva la maestra ni secretos en claro.
    pub fn exportar(&self, destino: &Path, transporte: &str) -> Result<(), ErrorBoveda> {
        self.exportar_con(destino, transporte, ParametrosKdf::boveda())
    }

    fn exportar_con(&self, destino: &Path, transporte: &str, params: ParametrosKdf) -> Result<(), ErrorBoveda> {
        if transporte.is_empty() {
            return Err(ErrorBoveda::DatoInvalido("La contraseña de transporte no puede estar vacía."));
        }
        let filas = self.filas_export()?;
        let json = serde_json::to_vec(&filas).map_err(|_| ErrorBoveda::Cripto)?;
        let salt = kdf::generar_salt();
        let clave = derivar(transporte, &salt, params)?;
        let blob = cipher::cifrar(&clave, &json).map_err(|_| ErrorBoveda::Cripto)?;
        let mut archivo = Vec::with_capacity(32 + blob.len());
        archivo.extend_from_slice(b"GEX1");
        archivo.extend_from_slice(&params.memoria_kib.to_le_bytes());
        archivo.extend_from_slice(&params.iteraciones.to_le_bytes());
        archivo.extend_from_slice(&params.paralelismo.to_le_bytes());
        archivo.extend_from_slice(&salt);
        archivo.extend_from_slice(&blob);
        if let Some(dir) = destino.parent() {
            if !dir.as_os_str().is_empty() {
                fs::create_dir_all(dir).map_err(ErrorBoveda::Io)?;
            }
        }
        fs::write(destino, archivo).map_err(ErrorBoveda::Io)?;
        Ok(())
    }

    pub fn importar(&self, origen: &Path, transporte: &str) -> Result<usize, ErrorBoveda> {
        if transporte.is_empty() {
            return Err(ErrorBoveda::DatoInvalido("La contraseña de transporte no puede estar vacía."));
        }
        let archivo = fs::read(origen).map_err(ErrorBoveda::Io)?;
        if archivo.len() < 32 || &archivo[..4] != b"GEX1" {
            return Err(ErrorBoveda::ArchivoInvalido);
        }
        let memoria = u32::from_le_bytes(archivo[4..8].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
        let iteraciones = u32::from_le_bytes(archivo[8..12].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
        let paralelismo = u32::from_le_bytes(archivo[12..16].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&archivo[16..32]);
        let clave = derivar(
            transporte,
            &salt,
            ParametrosKdf {
                memoria_kib: memoria,
                iteraciones,
                paralelismo,
            },
        )?;
        let json = cipher::descifrar(&clave, &archivo[32..]).map_err(|_| {
            ErrorBoveda::DatoInvalido("Contraseña de transporte incorrecta, o el archivo fue alterado.")
        })?;
        let filas: Vec<FilaExport> = serde_json::from_slice(&json).map_err(|_| ErrorBoveda::ArchivoInvalido)?;
        let ahora = ahora();
        let tx = self.conn.unchecked_transaction()?;
        for fila in &filas {
            if fila.sistema.trim().is_empty() || fila.secreto.is_empty() {
                return Err(ErrorBoveda::DatoInvalido("La copia tiene una credencial sin sistema o sin secreto."));
            }
            let secreto = cifrar_texto(&self.clave, &fila.secreto)?;
            let notas = cifrar_texto(&self.clave, &fila.notas)?;
            tx.execute(
                "INSERT INTO credenciales (sistema, usuario, secreto, notas, categoria, creado_en, actualizado_en, vence_en, favorito)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6, ?7, ?8)",
                params![
                    fila.sistema.trim(),
                    fila.usuario.trim(),
                    secreto,
                    notas,
                    fila.categoria.trim(),
                    ahora,
                    fila.vence_en,
                    i64::from(fila.favorito),
                ],
            )?;
        }
        tx.commit()?;
        self.persistir()?;
        Ok(filas.len())
    }

    fn filas_export(&self) -> Result<Vec<FilaExport>, ErrorBoveda> {
        let mut stmt = self.conn.prepare(
            "SELECT sistema, usuario, secreto, notas, categoria, favorito, vence_en FROM credenciales ORDER BY id",
        )?;
        let filas = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
                row.get::<_, Vec<u8>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
                row.get::<_, Option<i64>>(6)?,
            ))
        })?;
        let mut salida = Vec::new();
        for fila in filas {
            let (sistema, usuario, secreto, notas, categoria, favorito, vence_en) = fila?;
            salida.push(FilaExport {
                sistema,
                usuario,
                secreto: descifrar_texto(&self.clave, &secreto)?,
                notas: descifrar_texto(&self.clave, &notas)?,
                categoria,
                favorito: favorito != 0,
                vence_en,
            });
        }
        Ok(salida)
    }

    pub fn borrar(&self, id: i64) -> Result<(), ErrorBoveda> {
        let borradas = self
            .conn
            .execute("DELETE FROM credenciales WHERE id = ?1", [id])?;
        if borradas == 0 {
            Err(ErrorBoveda::NoEncontrada)
        } else {
            let _ = self
                .conn
                .execute("DELETE FROM historial WHERE credencial_id = ?1", [id]);
            self.persistir()?;
            Ok(())
        }
    }

    /// Deja el secreto cifrado y el TOTP inactivo hasta `confirmar_totp`. RF-02.
    pub fn enrolar_totp(&self) -> Result<(String, String), ErrorBoveda> {
        let (secreto, uri) = crate::auth::totp::generar().map_err(|_| ErrorBoveda::Cripto)?;
        let blob = cifrar_texto(&self.clave, &secreto)?;
        self.conn.execute(
            "UPDATE meta SET totp_secreto = ?1, totp_activo = 0 WHERE id = 1",
            params![blob],
        )?;
        self.persistir()?;
        Ok((secreto, uri))
    }

    pub fn confirmar_totp(&self, codigo: &str) -> Result<(), ErrorBoveda> {
        let blob: Option<Vec<u8>> = self
            .conn
            .query_row("SELECT totp_secreto FROM meta WHERE id = 1", [], |row| row.get(0))?;
        let Some(blob) = blob else {
            return Err(ErrorBoveda::DatoInvalido("No hay un TOTP pendiente."));
        };
        let secreto = descifrar_texto(&self.clave, &blob)?;
        if !crate::auth::totp::verificar(&secreto, codigo) {
            return Err(ErrorBoveda::TotpInvalido);
        }
        self.conn
            .execute("UPDATE meta SET totp_activo = 1 WHERE id = 1", [])?;
        self.persistir()?;
        Ok(())
    }

    pub fn cambiar_maestra(&mut self, actual: &str, nueva: &str) -> Result<(), ErrorBoveda> {
        validar_maestra(nueva)?;
        if nueva == actual {
            return Err(ErrorBoveda::DatoInvalido(
                "La nueva contraseña maestra tiene que ser distinta.",
            ));
        }
        let (salt, parametros, _) = leer_meta(&self.conn)?;
        let derivada = derivar(actual, &salt, parametros)?;
        if !claves_iguales(&derivada, &self.clave) {
            return Err(ErrorBoveda::MaestraIncorrecta);
        }
        let salt_nueva = kdf::generar_salt();
        let clave_nueva = derivar(nueva, &salt_nueva, parametros)?;
        let verificador = cipher::cifrar(&clave_nueva, VERIFICADOR).map_err(|_| ErrorBoveda::Cripto)?;
        let credenciales = self.blobs_credenciales()?;
        let mut credenciales_nuevas = Vec::with_capacity(credenciales.len());
        for (id_cred, secreto, notas) in credenciales {
            let secreto_plano = descifrar_texto(&self.clave, &secreto)?;
            let notas_plano = descifrar_texto(&self.clave, &notas)?;
            credenciales_nuevas.push((
                id_cred,
                cifrar_texto(&clave_nueva, &secreto_plano)?,
                cifrar_texto(&clave_nueva, &notas_plano)?,
            ));
        }
        let historial = self.blobs_historial()?;
        let mut historial_nuevo = Vec::with_capacity(historial.len());
        for (id_hist, secreto) in historial {
            let plano = descifrar_texto(&self.clave, &secreto)?;
            historial_nuevo.push((id_hist, cifrar_texto(&clave_nueva, &plano)?));
        }
        let totp_actual: Option<Vec<u8>> = self
            .conn
            .query_row("SELECT totp_secreto FROM meta WHERE id = 1", [], |row| row.get(0))?;
        let totp_nuevo = match totp_actual {
            Some(blob) => Some(cifrar_texto(
                &clave_nueva,
                &descifrar_texto(&self.clave, &blob)?,
            )?),
            None => None,
        };
        let tx = self.conn.transaction()?;
        for (id_cred, secreto, notas) in &credenciales_nuevas {
            tx.execute(
                "UPDATE credenciales SET secreto = ?1, notas = ?2 WHERE id = ?3",
                params![secreto, notas, id_cred],
            )?;
        }
        for (id_hist, secreto) in &historial_nuevo {
            tx.execute(
                "UPDATE historial SET secreto = ?1 WHERE id = ?2",
                params![secreto, id_hist],
            )?;
        }
        tx.execute(
            "UPDATE meta SET salt = ?1, verificador = ?2 WHERE id = 1",
            params![salt_nueva.as_slice(), verificador],
        )?;
        if let Some(blob) = &totp_nuevo {
            tx.execute(
                "UPDATE meta SET totp_secreto = ?1 WHERE id = 1",
                params![blob],
            )?;
        }
        tx.commit()?;
        self.clave = clave_nueva;
        self.persistir()?;
        Ok(())
    }

    fn blobs_credenciales(&self) -> Result<Vec<(i64, Vec<u8>, Vec<u8>)>, ErrorBoveda> {
        let mut stmt = self.conn.prepare("SELECT id, secreto, notas FROM credenciales")?;
        let filas = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;
        filas.collect::<Result<Vec<_>, _>>().map_err(ErrorBoveda::from)
    }

    fn blobs_historial(&self) -> Result<Vec<(i64, Vec<u8>)>, ErrorBoveda> {
        let mut stmt = self.conn.prepare("SELECT id, secreto FROM historial")?;
        let filas = stmt.query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?)))?;
        filas.collect::<Result<Vec<_>, _>>().map_err(ErrorBoveda::from)
    }

    pub fn guardar_politica(&self, politica: &Politica) -> Result<(), ErrorBoveda> {
        let sistema = politica.sistema.trim();
        if sistema.is_empty() {
            return Err(ErrorBoveda::DatoInvalido("El sistema de la política no puede estar vacío."));
        }
        if politica.modo != "aleatoria" && politica.modo != "passphrase" {
            return Err(ErrorBoveda::DatoInvalido("El modo tiene que ser aleatoria o passphrase."));
        }
        if politica.longitud == 0 {
            return Err(ErrorBoveda::DatoInvalido("La longitud de la política tiene que ser mayor que cero."));
        }
        if !politica.regex.is_empty() {
            crate::generator::regex_policy::cumple(&politica.regex, "")
                .map_err(|_| ErrorBoveda::DatoInvalido("La expresión regular no es válida."))?;
        }
        self.conn.execute(
            "INSERT INTO politicas (
                sistema, modo, longitud, minusculas, mayusculas, digitos, simbolos, regex, historial, dias_validez, categoria
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(sistema) DO UPDATE SET
                modo = excluded.modo,
                longitud = excluded.longitud,
                minusculas = excluded.minusculas,
                mayusculas = excluded.mayusculas,
                digitos = excluded.digitos,
                simbolos = excluded.simbolos,
                regex = excluded.regex,
                historial = excluded.historial,
                dias_validez = excluded.dias_validez,
                categoria = excluded.categoria",
            params![
                sistema,
                politica.modo,
                politica.longitud,
                politica.minusculas as i64,
                politica.mayusculas as i64,
                politica.digitos as i64,
                politica.simbolos as i64,
                politica.regex,
                politica.historial,
                politica.dias_validez,
                politica.categoria.trim()
            ],
        )?;
        self.persistir()?;
        Ok(())
    }

    pub fn obtener_politica(&self, sistema: &str) -> Result<Option<Politica>, ErrorBoveda> {
        let fila = self
            .conn
            .query_row(
                "SELECT sistema, modo, longitud, minusculas, mayusculas, digitos, simbolos, regex, historial, dias_validez, categoria
                 FROM politicas WHERE sistema = ?1",
                [sistema.trim()],
                |row| {
                    Ok(Politica {
                        sistema: row.get(0)?,
                        modo: row.get(1)?,
                        longitud: row.get::<_, i64>(2)? as u32,
                        minusculas: row.get::<_, i64>(3)? != 0,
                        mayusculas: row.get::<_, i64>(4)? != 0,
                        digitos: row.get::<_, i64>(5)? != 0,
                        simbolos: row.get::<_, i64>(6)? != 0,
                        regex: row.get(7)?,
                        historial: row.get::<_, i64>(8)? as u32,
                        dias_validez: row.get::<_, i64>(9)? as u32,
                        categoria: row.get(10)?,
                    })
                },
            )
            .optional()?;
        Ok(fila)
    }

    fn secretos_historial(&self, id: i64, n: u32) -> Result<Vec<String>, ErrorBoveda> {
        let mut stmt = self.conn.prepare(
            "SELECT secreto FROM historial WHERE credencial_id = ?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let filas = stmt.query_map(params![id, n], |row| row.get::<_, Vec<u8>>(0))?;
        let mut secretos = Vec::new();
        for blob in filas {
            secretos.push(descifrar_texto(&self.clave, &blob?)?);
        }
        Ok(secretos)
    }

    fn vence_de(&self, id: i64) -> Result<Option<i64>, ErrorBoveda> {
        self.conn
            .query_row("SELECT vence_en FROM credenciales WHERE id = ?1", [id], |row| row.get(0))
            .map_err(ErrorBoveda::from)
    }

    fn crear_con(ruta: &Path, maestra: &str, parametros: ParametrosKdf) -> Result<Self, ErrorBoveda> {
        validar_maestra(maestra)?;
        if ruta.exists() {
            return Err(ErrorBoveda::YaExiste);
        }
        if let Some(padre) = ruta.parent() {
            if !padre.as_os_str().is_empty() {
                fs::create_dir_all(padre).map_err(ErrorBoveda::Io)?;
            }
        }
        let conn = Connection::open_in_memory()?;
        let clave = match inicializar(&conn, maestra, parametros) {
            Ok(clave) => clave,
            Err(error) => return Err(error),
        };
        let mut boveda = Self {
            conn,
            clave,
            ruta: ruta.to_path_buf(),
            guardar: true,
        };
        if let Err(error) = boveda.persistir() {
            boveda.guardar = false;
            let _ = fs::remove_file(ruta);
            return Err(error);
        }
        Ok(boveda)
    }

    fn persistir(&self) -> Result<(), ErrorBoveda> {
        let (salt, parametros, _) = leer_meta(&self.conn)?;
        if salt.len() != 16 {
            return Err(ErrorBoveda::ArchivoInvalido);
        }
        let plano = self.conn.serialize(DatabaseName::Main).map_err(ErrorBoveda::from)?;
        let cifrado = cipher::cifrar(&self.clave, &plano).map_err(|_| ErrorBoveda::Cripto)?;
        let mut archivo = Vec::with_capacity(CABECERA_BOV2 + cifrado.len());
        archivo.extend_from_slice(MAGIC_BOV2);
        archivo.extend_from_slice(&parametros.memoria_kib.to_le_bytes());
        archivo.extend_from_slice(&parametros.iteraciones.to_le_bytes());
        archivo.extend_from_slice(&parametros.paralelismo.to_le_bytes());
        archivo.extend_from_slice(&salt);
        archivo.extend_from_slice(&cifrado);
        escribir_atomico(&self.ruta, &archivo)
    }

    #[cfg(test)]
    fn fijar_vence(&self, sistema: &str, vence_en: i64) -> Result<(), ErrorBoveda> {
        let cambiadas = self.conn.execute(
            "UPDATE credenciales SET vence_en = ?1 WHERE sistema = ?2",
            params![vence_en, sistema],
        )?;
        if cambiadas == 0 {
            return Err(ErrorBoveda::NoEncontrada);
        }
        self.persistir()
    }
}

fn inicializar(
    conn: &Connection,
    maestra: &str,
    parametros: ParametrosKdf,
) -> Result<[u8; 32], ErrorBoveda> {
    conn.execute_batch(ESQUEMA)?;
    let salt = kdf::generar_salt();
    let clave = derivar(maestra, &salt, parametros)?;
    let verificador = cipher::cifrar(&clave, VERIFICADOR).map_err(|_| ErrorBoveda::Cripto)?;
    conn.execute(
        "INSERT INTO meta (id, salt, memoria_kib, iteraciones, paralelismo, verificador)
         VALUES (1, ?1, ?2, ?3, ?4, ?5)",
        params![
            salt.as_slice(),
            parametros.memoria_kib,
            parametros.iteraciones,
            parametros.paralelismo,
            verificador
        ],
    )?;
    asegurar_esquema(conn)?;
    Ok(clave)
}

fn leer_meta(conn: &Connection) -> Result<(Vec<u8>, ParametrosKdf, Vec<u8>), ErrorBoveda> {
    let fila = conn
        .query_row(
            "SELECT salt, memoria_kib, iteraciones, paralelismo, verificador FROM meta WHERE id = 1",
            [],
            |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Vec<u8>>(4)?,
                ))
            },
        )
        .optional()?;
    let Some((salt, memoria, iteraciones, paralelismo, verificador)) = fila else {
        return Err(ErrorBoveda::ArchivoInvalido);
    };
    let parametros = ParametrosKdf {
        memoria_kib: u32::try_from(memoria).map_err(|_| ErrorBoveda::ArchivoInvalido)?,
        iteraciones: u32::try_from(iteraciones).map_err(|_| ErrorBoveda::ArchivoInvalido)?,
        paralelismo: u32::try_from(paralelismo).map_err(|_| ErrorBoveda::ArchivoInvalido)?,
    };
    Ok((salt, parametros, verificador))
}

fn leer_cabecera(archivo: &[u8]) -> Result<(ParametrosKdf, [u8; 16], &[u8]), ErrorBoveda> {
    if archivo.len() < CABECERA_BOV2 || &archivo[..4] != MAGIC_BOV2 {
        return Err(ErrorBoveda::ArchivoInvalido);
    }
    let memoria = u32::from_le_bytes(archivo[4..8].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
    let iteraciones = u32::from_le_bytes(archivo[8..12].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
    let paralelismo = u32::from_le_bytes(archivo[12..16].try_into().map_err(|_| ErrorBoveda::ArchivoInvalido)?);
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&archivo[16..32]);
    Ok((
        ParametrosKdf {
            memoria_kib: memoria,
            iteraciones,
            paralelismo,
        },
        salt,
        &archivo[CABECERA_BOV2..],
    ))
}

fn conexion_desde(bytes: &[u8]) -> Result<Connection, ErrorBoveda> {
    if bytes.is_empty() {
        return Err(ErrorBoveda::ArchivoInvalido);
    }
    let mut conn = Connection::open_in_memory()?;
    let tamano = i32::try_from(bytes.len()).map_err(|_| ErrorBoveda::ArchivoInvalido)?;
    let ptr = unsafe { rusqlite::ffi::sqlite3_malloc(tamano) } as *mut u8;
    if ptr.is_null() {
        return Err(ErrorBoveda::Cripto);
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr, bytes.len());
        let data = rusqlite::serialize::OwnedData::from_raw_nonnull(NonNull::new_unchecked(ptr), bytes.len());
        conn.deserialize(DatabaseName::Main, data, false)?;
    }
    Ok(conn)
}

fn escribir_atomico(ruta: &Path, bytes: &[u8]) -> Result<(), ErrorBoveda> {
    let temporal = ruta.with_extension("tmp");
    fs::write(&temporal, bytes).map_err(ErrorBoveda::Io)?;
    if ruta.exists() {
        fs::remove_file(ruta).map_err(ErrorBoveda::Io)?;
    }
    fs::rename(&temporal, ruta).map_err(ErrorBoveda::Io)?;
    Ok(())
}

fn derivar(maestra: &str, salt: &[u8], parametros: ParametrosKdf) -> Result<[u8; 32], ErrorBoveda> {
    kdf::derivar_clave(maestra, salt, parametros).map_err(|_| ErrorBoveda::Kdf)
}

fn cifrar_texto(clave: &[u8; 32], texto: &str) -> Result<Vec<u8>, ErrorBoveda> {
    cipher::cifrar(clave, texto.as_bytes()).map_err(|_| ErrorBoveda::Cripto)
}

fn descifrar_texto(clave: &[u8; 32], blob: &[u8]) -> Result<String, ErrorBoveda> {
    let bytes = cipher::descifrar(clave, blob).map_err(|_| ErrorBoveda::Cripto)?;
    String::from_utf8(bytes).map_err(|_| ErrorBoveda::Cripto)
}

fn validar_maestra(maestra: &str) -> Result<(), ErrorBoveda> {
    if maestra.is_empty() {
        Err(ErrorBoveda::DatoInvalido(
            "La contraseña maestra no puede estar vacía.",
        ))
    } else {
        Ok(())
    }
}

fn validar_credencial(sistema: &str, secreto: &str) -> Result<(), ErrorBoveda> {
    if sistema.trim().is_empty() {
        return Err(ErrorBoveda::DatoInvalido("El sistema no puede estar vacío."));
    }
    if secreto.is_empty() {
        return Err(ErrorBoveda::DatoInvalido(
            "La contraseña de la credencial no puede estar vacía.",
        ));
    }
    Ok(())
}

fn claves_iguales(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diferencia = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diferencia |= x ^ y;
    }
    diferencia == 0
}

fn ahora() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn calcular_vence(politica: &Politica) -> Option<i64> {
    if politica.dias_validez == 0 {
        None
    } else {
        Some(ahora() + i64::from(politica.dias_validez) * 86_400)
    }
}

fn validar_secreto(politica: &Politica, secreto: &str, anteriores: &[String]) -> Result<(), ErrorBoveda> {
    if !politica.regex.is_empty() {
        let ok = crate::generator::regex_policy::cumple(&politica.regex, secreto)
            .map_err(|_| ErrorBoveda::DatoInvalido("La expresión regular no es válida."))?;
        if !ok {
            return Err(ErrorBoveda::DatoInvalido(
                "La contraseña no cumple la expresión regular del sistema.",
            ));
        }
    }
    if politica.modo == "passphrase" {
        let palabras = secreto
            .split(|c: char| c == '-' || c.is_whitespace())
            .filter(|p| !p.is_empty())
            .count();
        if palabras < politica.longitud as usize {
            return Err(ErrorBoveda::DatoInvalido(
                "La frase tiene menos palabras que la política del sistema.",
            ));
        }
    } else if secreto.chars().count() < politica.longitud as usize {
        return Err(ErrorBoveda::DatoInvalido(
            "La contraseña es más corta que la política del sistema.",
        ));
    }
    if anteriores.iter().any(|vieja| vieja == secreto) {
        return Err(ErrorBoveda::DatoInvalido(
            "Esa contraseña ya se usó en el historial de esta credencial.",
        ));
    }
    Ok(())
}

fn asegurar_esquema(conn: &Connection) -> Result<(), ErrorBoveda> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS politicas (
            sistema TEXT PRIMARY KEY,
            modo TEXT NOT NULL,
            longitud INTEGER NOT NULL,
            minusculas INTEGER NOT NULL,
            mayusculas INTEGER NOT NULL,
            digitos INTEGER NOT NULL,
            simbolos INTEGER NOT NULL,
            regex TEXT NOT NULL DEFAULT '',
            historial INTEGER NOT NULL DEFAULT 0,
            dias_validez INTEGER NOT NULL DEFAULT 0,
            categoria TEXT NOT NULL DEFAULT ''
        );
        CREATE TABLE IF NOT EXISTS historial (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            credencial_id INTEGER NOT NULL,
            secreto BLOB NOT NULL,
            creado_en INTEGER NOT NULL
        );",
    )?;
    let mut stmt = conn.prepare("PRAGMA table_info(credenciales)")?;
    let nombres: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<_, _>>()?;
    if !nombres.iter().any(|n| n == "vence_en") {
        conn.execute("ALTER TABLE credenciales ADD COLUMN vence_en INTEGER", [])?;
    }
    if !nombres.iter().any(|n| n == "favorito") {
        conn.execute(
            "ALTER TABLE credenciales ADD COLUMN favorito INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    let mut meta = conn.prepare("PRAGMA table_info(meta)")?;
    let columnas: Vec<String> = meta
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<_, _>>()?;
    if !columnas.iter().any(|n| n == "totp_secreto") {
        conn.execute("ALTER TABLE meta ADD COLUMN totp_secreto BLOB", [])?;
    }
    if !columnas.iter().any(|n| n == "totp_activo") {
        conn.execute(
            "ALTER TABLE meta ADD COLUMN totp_activo INTEGER NOT NULL DEFAULT 0",
            [],
        )?;
    }
    Ok(())
}

fn exigir_totp(conn: &Connection, clave: &[u8; 32], codigo: Option<&str>) -> Result<(), ErrorBoveda> {
    let (activo, blob): (i64, Option<Vec<u8>>) = conn.query_row(
        "SELECT totp_activo, totp_secreto FROM meta WHERE id = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    if activo == 0 {
        return Ok(());
    }
    let Some(blob) = blob else {
        return Err(ErrorBoveda::TotpRequerido);
    };
    let Some(codigo) = codigo.map(str::trim).filter(|valor| !valor.is_empty()) else {
        return Err(ErrorBoveda::TotpRequerido);
    };
    let secreto = descifrar_texto(clave, &blob)?;
    if crate::auth::totp::verificar(&secreto, codigo) {
        Ok(())
    } else {
        Err(ErrorBoveda::TotpInvalido)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FilaExport {
    sistema: String,
    usuario: String,
    secreto: String,
    notas: String,
    categoria: String,
    favorito: bool,
    vence_en: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params_rapidos() -> ParametrosKdf {
        ParametrosKdf {
            memoria_kib: 8,
            iteraciones: 1,
            paralelismo: 1,
        }
    }

    fn ruta_tmp(nombre: &str) -> PathBuf {
        let ruta = std::env::temp_dir().join(format!(
            "gestor-{nombre}-{}.sqlite",
            std::process::id()
        ));
        let _ = fs::remove_file(&ruta);
        ruta
    }

    #[test]
    fn alta_no_deja_el_secreto_en_el_archivo_y_se_puede_reabrir() {
        let ruta = ruta_tmp("ronda");
        let secreto = "SECRETO-UNICO-XYZ-91";
        let id = {
            let boveda = Boveda::crear_con(&ruta, "maestra-de-prueba", params_rapidos()).unwrap();
            let creada = boveda
                .alta("Banco", "ana", secreto, "nota local", "finanzas")
                .unwrap();
            let lista = boveda.listar().unwrap();
            assert_eq!(lista.len(), 1);
            assert_eq!(lista[0].sistema, "Banco");
            assert_eq!(boveda.obtener(creada.id).unwrap().secreto, secreto);
            creada.id
        };
        let bytes = fs::read(&ruta).unwrap();
        assert!(bytes.starts_with(b"BOV2"));
        assert!(!bytes.starts_with(b"SQLite format 3"));
        assert!(
            !bytes.windows(secreto.len()).any(|w| w == secreto.as_bytes()),
            "el secreto quedó en texto plano dentro del archivo"
        );
        assert!(
            !bytes.windows(b"Banco".len()).any(|w| w == b"Banco"),
            "el sistema quedó en texto plano dentro del archivo"
        );

        let boveda = Boveda::abrir(&ruta, "maestra-de-prueba").unwrap();
        boveda
            .modificar(id, "Banco", "ana", "otro-secreto-22", "nota local", "finanzas")
            .unwrap();
        assert_eq!(boveda.obtener(id).unwrap().secreto, "otro-secreto-22");
        boveda.borrar(id).unwrap();
        assert!(boveda.listar().unwrap().is_empty());
        assert!(matches!(
            boveda.obtener(id),
            Err(ErrorBoveda::NoEncontrada)
        ));
        drop(boveda);
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn maestra_incorrecta_no_abre_y_no_se_pisa_una_boveda() {
        let ruta = ruta_tmp("maestra");
        Boveda::crear_con(&ruta, "maestra-buena", params_rapidos()).unwrap();
        assert!(matches!(
            Boveda::crear_con(&ruta, "otra", params_rapidos()),
            Err(ErrorBoveda::YaExiste)
        ));
        assert!(matches!(
            Boveda::abrir(&ruta, "maestra-mala"),
            Err(ErrorBoveda::MaestraIncorrecta)
        ));
        let inexistente = ruta_tmp("nada");
        assert!(matches!(
            Boveda::abrir(&inexistente, "maestra-buena"),
            Err(ErrorBoveda::NoExiste)
        ));
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn rechaza_maestra_o_sistema_vacios() {
        let ruta = ruta_tmp("vacio");
        assert!(matches!(
            Boveda::crear_con(&ruta, "", params_rapidos()),
            Err(ErrorBoveda::DatoInvalido(_))
        ));
        let boveda = Boveda::crear_con(&ruta, "maestra", params_rapidos()).unwrap();
        assert!(matches!(
            boveda.alta("  ", "ana", "secreto", "", ""),
            Err(ErrorBoveda::DatoInvalido(_))
        ));
        assert!(matches!(
            boveda.alta("Correo", "ana", "", "", ""),
            Err(ErrorBoveda::DatoInvalido(_))
        ));
        drop(boveda);
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn la_politica_rechaza_regex_reuso_y_pone_vencimiento() {
        let ruta = ruta_tmp("politica");
        let boveda = Boveda::crear_con(&ruta, "maestra", params_rapidos()).unwrap();
        boveda
            .guardar_politica(&Politica {
                sistema: "Cajero".into(),
                modo: "aleatoria".into(),
                longitud: 4,
                minusculas: false,
                mayusculas: false,
                digitos: true,
                simbolos: false,
                regex: r"^[0-9]{4}$".into(),
                historial: 1,
                dias_validez: 30,
                categoria: "banco".into(),
            })
            .unwrap();
        assert!(matches!(
            boveda.alta("Cajero", "ana", "abcd", "", ""),
            Err(ErrorBoveda::DatoInvalido(_))
        ));
        let creada = boveda.alta("Cajero", "ana", "1234", "", "").unwrap();
        assert_eq!(creada.categoria, "banco");
        assert!(creada.vence_en.is_some());
        boveda
            .modificar(creada.id, "Cajero", "ana", "5678", "", "banco")
            .unwrap();
        assert!(matches!(
            boveda.modificar(creada.id, "Cajero", "ana", "1234", "", "banco"),
            Err(ErrorBoveda::DatoInvalido(_))
        ));
        drop(boveda);
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn cambiar_maestra_reabre_con_la_nueva() {
        let ruta = ruta_tmp("cambio");
        let mut boveda = Boveda::crear_con(&ruta, "maestra", params_rapidos()).unwrap();
        let id = boveda.alta("Banco", "ana", "1234", "nota", "").unwrap().id;
        assert!(boveda.cambiar_maestra("mal", "maestra-nueva").is_err());
        boveda.cambiar_maestra("maestra", "maestra-nueva").unwrap();
        drop(boveda);
        let otra = Boveda::abrir(&ruta, "maestra-nueva").unwrap();
        assert_eq!(otra.obtener(id).unwrap().secreto, "1234");
        assert_eq!(otra.obtener(id).unwrap().notas, "nota");
        drop(otra);
        assert!(matches!(
            Boveda::abrir(&ruta, "maestra"),
            Err(ErrorBoveda::MaestraIncorrecta)
        ));
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn favorito_y_copia_cifrada_viajan_a_otra_boveda() {
        let ruta = ruta_tmp("export");
        let copia = ruta_tmp("export-archivo").with_extension("gex");
        let destino = ruta_tmp("import");
        let secreto = "SECRETO-EXPORT-77";
        let boveda = Boveda::crear_con(&ruta, "maestra", params_rapidos()).unwrap();
        let id = boveda.alta("Correo", "ana", secreto, "nota", "trabajo").unwrap().id;
        boveda.marcar_favorito(id, true).unwrap();
        assert!(boveda.listar().unwrap()[0].favorito);
        boveda.exportar_con(&copia, "transporte", params_rapidos()).unwrap();
        let bytes = fs::read(&copia).unwrap();
        assert!(!bytes.windows(secreto.len()).any(|w| w == secreto.as_bytes()));
        assert!(boveda.importar(&copia, "mal").is_err());
        drop(boveda);
        let otra = Boveda::crear_con(&destino, "otra-maestra", params_rapidos()).unwrap();
        assert_eq!(otra.importar(&copia, "transporte").unwrap(), 1);
        let lista = otra.listar().unwrap();
        assert_eq!(lista.len(), 1);
        assert!(lista[0].favorito);
        assert_eq!(lista[0].categoria, "trabajo");
        assert_eq!(otra.obtener(lista[0].id).unwrap().secreto, secreto);
        drop(otra);
        let _ = fs::remove_file(&ruta);
        let _ = fs::remove_file(&copia);
        let _ = fs::remove_file(&destino);
    }

    #[test]
    fn sistemas_vencidos_avisa_solo_los_que_ya_pasaron() {
        let ruta = ruta_tmp("vence");
        {
            let boveda = Boveda::crear_con(&ruta, "maestra-de-prueba", params_rapidos()).unwrap();
            boveda.alta("Banco", "ana", "secreto-uno", "", "finanzas").unwrap();
            boveda.alta("Correo", "ana", "secreto-dos", "", "trabajo").unwrap();
            boveda.fijar_vence("Banco", 1).unwrap();
            boveda.fijar_vence("Correo", super::ahora() + 86_400).unwrap();
        }
        let boveda = Boveda::abrir(&ruta, "maestra-de-prueba").unwrap();
        assert_eq!(boveda.sistemas_vencidos().unwrap(), vec!["Banco".to_string()]);
        drop(boveda);
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn el_totp_pide_el_codigo_para_volver_a_abrir() {
        let ruta = ruta_tmp("totp");
        let secreto = {
            let boveda = Boveda::crear_con(&ruta, "maestra-de-prueba", params_rapidos()).unwrap();
            let (secreto, _) = boveda.enrolar_totp().unwrap();
            let codigo = crate::auth::totp::codigo_actual(&secreto).unwrap();
            boveda.confirmar_totp(&codigo).unwrap();
            secreto
        };
        assert!(matches!(
            Boveda::abrir(&ruta, "maestra-de-prueba"),
            Err(ErrorBoveda::TotpRequerido)
        ));
        let codigo = crate::auth::totp::codigo_actual(&secreto).unwrap();
        let malo = if codigo == "000000" { "111111" } else { "000000" };
        assert!(matches!(
            Boveda::abrir_con(&ruta, "maestra-de-prueba", Some(malo)),
            Err(ErrorBoveda::TotpInvalido)
        ));
        assert!(Boveda::abrir_con(&ruta, "maestra-de-prueba", Some(&codigo)).is_ok());
        drop(Boveda::abrir_con(&ruta, "maestra-de-prueba", Some(&codigo)).unwrap());
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn una_boveda_sqlite_vieja_se_cifra_al_abrirla() {
        let ruta = ruta_tmp("legacy");
        {
            let conn = Connection::open(&ruta).unwrap();
            let clave = inicializar(&conn, "maestra", params_rapidos()).unwrap();
            let secreto = cifrar_texto(&clave, "secreto-viejo").unwrap();
            let notas = cifrar_texto(&clave, "").unwrap();
            let ahora = super::ahora();
            conn.execute(
                "INSERT INTO credenciales (sistema, usuario, secreto, notas, categoria, creado_en, actualizado_en)
                 VALUES ('Banco', 'ana', ?1, ?2, '', ?3, ?3)",
                params![secreto, notas, ahora],
            )
            .unwrap();
        }
        assert!(fs::read(&ruta).unwrap().starts_with(b"SQLite format 3"));
        let boveda = Boveda::abrir(&ruta, "maestra").unwrap();
        assert_eq!(boveda.obtener(1).unwrap().secreto, "secreto-viejo");
        drop(boveda);
        let bytes = fs::read(&ruta).unwrap();
        assert!(bytes.starts_with(b"BOV2"));
        assert_eq!(
            Boveda::abrir(&ruta, "maestra").unwrap().listar().unwrap().len(),
            1
        );
        let _ = fs::remove_file(&ruta);
    }

    #[test]
    fn sembrar_anexo_b_en_una_boveda() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../datos-prueba");
        fs::create_dir_all(&dir).unwrap();
        let ruta = dir.join("boveda-anexo-b.sqlite");
        let _ = fs::remove_file(&ruta);
        let boveda = Boveda::crear(&ruta, "maestra-de-prueba").unwrap();
        let sistemas = [
            ("correo-laboral", "ana.duarte", "trabajo"),
            ("banca-en-linea", "ana.duarte", "finanzas"),
            ("portal-agesic", "ana.duarte", "trabajo"),
            ("vpn-organizacion", "ana.duarte", "infraestructura"),
            ("wifi-oficina", "invitado", "infraestructura"),
            ("repositorio-git", "ana.duarte", "desarrollo"),
            ("panel-grafana", "rsi", "infraestructura"),
            ("servidor-correo", "postmaster", "infraestructura"),
            ("mesa-de-ayuda", "ana.duarte", "trabajo"),
            ("nube-archivos", "ana.duarte", "trabajo"),
            ("firma-digital", "ana.duarte", "acceso"),
            ("registro-horario", "ana.duarte", "trabajo"),
            ("compras-internas", "ana.duarte", "trabajo"),
            ("portal-rrhh", "ana.duarte", "trabajo"),
            ("monitor-wazuh", "rsi", "infraestructura"),
            ("base-de-datos", "gestor", "infraestructura"),
            ("tablero-incidentes", "rsi", "infraestructura"),
            ("acceso-edificio", "ana.duarte", "acceso"),
            ("impresora-segura", "ana.duarte", "oficina"),
            ("respaldo-remoto", "ana.duarte", "infraestructura"),
            ("wiki-interna", "ana.duarte", "trabajo"),
            ("chat-equipo", "ana.duarte", "trabajo"),
        ];
        for (sistema, usuario, categoria) in sistemas {
            let secreto = format!("Lab-{sistema}-2026");
            boveda
                .alta(
                    sistema,
                    usuario,
                    &secreto,
                    "Dato ficticio del conjunto mínimo de prueba.",
                    categoria,
                )
                .unwrap();
        }
        assert_eq!(boveda.listar().unwrap().len(), sistemas.len());
        drop(boveda);
        let abierta = Boveda::abrir(&ruta, "maestra-de-prueba").unwrap();
        assert_eq!(abierta.listar().unwrap().len(), 22);
    }
}
