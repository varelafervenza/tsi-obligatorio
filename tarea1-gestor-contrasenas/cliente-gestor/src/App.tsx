import { FormEvent, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { QRCodeSVG } from "qrcode.react";

type Resumen = {
  id: number;
  sistema: string;
  usuario: string;
  categoria: string;
  vence_en: number | null;
  favorito: boolean;
};

type Politica = {
  modo: string;
  longitud: number;
  minusculas: boolean;
  mayusculas: boolean;
  digitos: boolean;
  simbolos: boolean;
  regex: string;
  historial: number;
  dias_validez: number;
  categoria: string;
};

type EstadoAud = {
  agente_id: string;
  url: string;
  carpeta_claves: string;
  en_cola: number;
  ultimo: string;
};
const politicaInicial: Politica = {
  modo: "aleatoria",
  longitud: 16,
  minusculas: true,
  mayusculas: true,
  digitos: true,
  simbolos: false,
  regex: "",
  historial: 3,
  dias_validez: 90,
  categoria: "",
};

function contarVencidas(lista: Resumen[]): number {
  const ahora = Math.floor(Date.now() / 1000);
  return lista.filter((item) => item.vence_en != null && item.vence_en < ahora).length;
}

function textoVencidas(cantidad: number): string {
  if (cantidad === 1) return "Hay 1 credencial vencida.";
  return `Hay ${cantidad} credenciales vencidas.`;
}

function textoVence(vence: number | null): string | null {
  if (vence == null) return null;
  const ahora = Math.floor(Date.now() / 1000);
  if (vence < ahora) return "vencida";
  const dias = Math.max(1, Math.ceil((vence - ahora) / 86400));
  return `vence en ${dias} d`;
}

function visibles(lista: Resumen[], busqueda: string, filtro: string, categoria: string): Resumen[] {
  const texto = busqueda.trim().toLowerCase();
  const ahora = Math.floor(Date.now() / 1000);
  return lista.filter((item) => {
    const coincide =
      texto === "" ||
      `${item.sistema} ${item.usuario} ${item.categoria}`.toLowerCase().includes(texto);
    if (!coincide) return false;
    if (categoria !== "" && item.categoria !== categoria) return false;
    if (filtro === "favoritos" && !item.favorito) return false;
    if (filtro === "vencidas" && (item.vence_en == null || item.vence_en >= ahora)) return false;
    return true;
  });
}

type Detalle = Resumen & {
  secreto: string;
  notas: string;
};

type Formulario = {
  id: number | null;
  sistema: string;
  usuario: string;
  secreto: string;
  notas: string;
  categoria: string;
};

const formularioVacio: Formulario = {
  id: null,
  sistema: "",
  usuario: "",
  secreto: "",
  notas: "",
  categoria: "",
};

function mensaje(error: unknown): string {
  if (typeof error === "string") return error;
  if (error instanceof Error) return error.message;
  return "No se pudo completar la operación.";
}

export default function App() {
  const [ruta, setRuta] = useState("");
  const [maestra, setMaestra] = useState("");
  const [codigoTotp, setCodigoTotp] = useState("");
  const [secretoTotp, setSecretoTotp] = useState("");
  const [uriTotp, setUriTotp] = useState("");
  const [codigoEnroll, setCodigoEnroll] = useState("");
  const [abierta, setAbierta] = useState(false);
  const [lista, setLista] = useState<Resumen[]>([]);
  const [form, setForm] = useState<Formulario>(formularioVacio);
  const [verSecreto, setVerSecreto] = useState(false);
  const [politica, setPolitica] = useState<Politica>(politicaInicial);
  const [aviso, setAviso] = useState("");
  const [urlCentral, setUrlCentral] = useState("http://localhost:8001/api/events/");
  const [carpetaClaves, setCarpetaClaves] = useState("");
  const [agenteId, setAgenteId] = useState("");
  const [enCola, setEnCola] = useState(0);
  const [maestraActual, setMaestraActual] = useState("");
  const [maestraNueva, setMaestraNueva] = useState("");
  const [busqueda, setBusqueda] = useState("");
  const [filtro, setFiltro] = useState("todas");
  const [categoriaFiltro, setCategoriaFiltro] = useState("");
  const [transporte, setTransporte] = useState("");
  const [rutaCopia, setRutaCopia] = useState("");
  const [ocupado, setOcupado] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    invoke<string>("ruta_por_defecto")
      .then(setRuta)
      .catch((e: unknown) => setError(mensaje(e)));
    invoke<EstadoAud>("estado_auditoria")
      .then((estado) => {
        setUrlCentral(estado.url);
        setCarpetaClaves(estado.carpeta_claves);
        setAgenteId(estado.agente_id);
        setEnCola(estado.en_cola);
      })
      .catch((e: unknown) => setError(mensaje(e)));
  }, []);

  async function anotarEvento() {
    const estado = await invoke<EstadoAud>("estado_auditoria");
    setAgenteId(estado.agente_id);
    setEnCola(estado.en_cola);
    if (estado.ultimo) setAviso(estado.ultimo);
  }

  async function reintentarEnvio() {
    await conBoveda(async () => {
      const texto = await invoke<string>("reintentar_eventos");
      setAviso(texto);
      await anotarEvento();
    });
  }

  async function refrescar() {
    const items = await invoke<Resumen[]>("listar_credenciales");
    setLista(items);
  }

  async function conBoveda(accion: () => Promise<void>) {
    setOcupado(true);
    setError("");
    try {
      await accion();
    } catch (e) {
      setError(mensaje(e));
    } finally {
      setOcupado(false);
    }
  }

  function crearOAbrir(comando: "crear_boveda" | "abrir_boveda") {
    return (evento: { preventDefault: () => void }) => {
      evento.preventDefault();
      void conBoveda(async () => {
        if (comando === "abrir_boveda") {
          const vencidos = await invoke<string[]>("abrir_boveda", {
            ruta,
            maestra,
            codigo: codigoTotp.trim() === "" ? null : codigoTotp.trim(),
          });
          setMaestra("");
          setAbierta(true);
          setForm(formularioVacio);
          await refrescar();
          if (vencidos.length > 0) await anotarEvento();
          return;
        }
        await invoke("crear_boveda", { ruta, maestra });
        setMaestra("");
        setAbierta(true);
        setForm(formularioVacio);
        await refrescar();
      });
    };
  }

  function guardar(evento: FormEvent) {
    evento.preventDefault();
    void conBoveda(async () => {
      if (form.id === null) {
        await invoke("alta_credencial", {
          sistema: form.sistema,
          usuario: form.usuario,
          secreto: form.secreto,
          notas: form.notas,
          categoria: form.categoria,
        });
        setForm(formularioVacio);
      } else {
        await invoke("modificar_credencial", {
          id: form.id,
          sistema: form.sistema,
          usuario: form.usuario,
          secreto: form.secreto,
          notas: form.notas,
          categoria: form.categoria,
        });
      }
      setVerSecreto(false);
      await refrescar();
      await anotarEvento();
    });
  }

  async function cargarPolitica(sistema: string) {
    if (!sistema.trim()) return;
    const guardada = await invoke<Politica & { sistema: string } | null>("obtener_politica", {
      sistema,
    });
    if (!guardada) return;
    setPolitica({
      modo: guardada.modo,
      longitud: guardada.longitud,
      minusculas: guardada.minusculas,
      mayusculas: guardada.mayusculas,
      digitos: guardada.digitos,
      simbolos: guardada.simbolos,
      regex: guardada.regex,
      historial: guardada.historial,
      dias_validez: guardada.dias_validez,
      categoria: guardada.categoria,
    });
    setForm((actual) => ({
      ...actual,
      categoria: actual.categoria || guardada.categoria,
    }));
  }

  function elegir(id: number) {
    void conBoveda(async () => {
      const detalle = await invoke<Detalle>("obtener_credencial", { id });
      setForm({
        id: detalle.id,
        sistema: detalle.sistema,
        usuario: detalle.usuario,
        secreto: detalle.secreto,
        notas: detalle.notas,
        categoria: detalle.categoria,
      });
      setVerSecreto(false);
      await cargarPolitica(detalle.sistema);
    });
  }

  async function generar() {
    await conBoveda(async () => {
      const secreto = await invoke<string>("generar_contrasena", {
        modo: politica.modo,
        longitud: Number(politica.longitud),
        minusculas: politica.minusculas,
        mayusculas: politica.mayusculas,
        digitos: politica.digitos,
        simbolos: politica.simbolos,
        regex: politica.regex,
      });
      setForm((actual) => ({
        ...actual,
        secreto,
        categoria: actual.categoria || politica.categoria,
      }));
      setVerSecreto(true);
    });
  }

  function guardarPolitica() {
    if (!form.sistema.trim()) {
      setError("Escribí el sistema antes de guardar la política.");
      return;
    }
    void conBoveda(async () => {
      await invoke("guardar_politica", {
        sistema: form.sistema,
        modo: politica.modo,
        longitud: Number(politica.longitud),
        minusculas: politica.minusculas,
        mayusculas: politica.mayusculas,
        digitos: politica.digitos,
        simbolos: politica.simbolos,
        regex: politica.regex,
        historial: Number(politica.historial),
        diasValidez: Number(politica.dias_validez),
        categoria: politica.categoria,
      });
      setAviso(`Política guardada para ${form.sistema.trim()}.`);
    });
  }

  function borrar() {
    if (form.id === null) return;
    void conBoveda(async () => {
      await invoke("borrar_credencial", { id: form.id, sistema: form.sistema });
      setForm(formularioVacio);
      await refrescar();
      await anotarEvento();
    });
  }

  function cerrar() {
    void conBoveda(async () => {
      await invoke("cerrar_boveda");
      setAbierta(false);
      setLista([]);
      setForm(formularioVacio);
      setPolitica(politicaInicial);
      setAviso("");
      setVerSecreto(false);
    });
  }

  if (!abierta) {
    return (
      <main className="app">
        <header>
          <h1>Gestor de contraseñas</h1>
          <p>La bóveda queda en este equipo. No hace falta red para usarla.</p>
        </header>
        <Auditoria
          urlCentral={urlCentral}
          carpetaClaves={carpetaClaves}
          agenteId={agenteId}
          enCola={enCola}
          ocupado={ocupado}
          onUrl={setUrlCentral}
          onCarpeta={setCarpetaClaves}
          onReintentar={reintentarEnvio}
          onGuardar={() => {
            void conBoveda(async () => {
              const texto = await invoke<string>("guardar_auditoria", {
                url: urlCentral,
                carpetaClaves,
              });
              setAviso(texto);
              await anotarEvento();
            });
          }}
        />
        <form className="panel" onSubmit={crearOAbrir("abrir_boveda")}>
          <label className="fila">
            <span>Archivo de la bóveda</span>
            <input value={ruta} onChange={(e) => setRuta(e.target.value)} spellCheck={false} />
          </label>
          <label className="fila">
            <span>Contraseña maestra</span>
            <input
              type="password"
              value={maestra}
              onChange={(e) => setMaestra(e.target.value)}
              autoComplete="new-password"
            />
          </label>
          <label className="fila">
            <span>Código TOTP</span>
            <input
              value={codigoTotp}
              inputMode="numeric"
              autoComplete="one-time-code"
              placeholder="solo si la bóveda lo tiene activo"
              onChange={(e) => setCodigoTotp(e.target.value)}
            />
          </label>
          <p className="ayuda">Argon2id. Abrir puede tardar un segundo. El código solo hace falta si enrolaste TOTP.</p>
          <div className="acciones">
            <button type="submit" disabled={ocupado || ruta === "" || maestra === ""}>
              Abrir bóveda
            </button>
            <button
              type="button"
              className="secundario"
              disabled={ocupado || ruta === "" || maestra === ""}
              onClick={crearOAbrir("crear_boveda")}
            >
              Crear bóveda
            </button>
          </div>
          {error ? <p className="error">{error}</p> : null}
          {aviso ? <p className="ayuda">{aviso}</p> : null}
        </form>
      </main>
    );
  }

  return (
    <main className="app">
      <header className="cabecera-abierta">
        <div>
          <h1>Bóveda abierta</h1>
          <p>{ruta}</p>
        </div>
        <button type="button" className="secundario" onClick={cerrar} disabled={ocupado}>
          Cerrar
        </button>
      </header>
      <details className="politica">
        <summary>TOTP de esta bóveda</summary>
        <div className="panel">
          <p className="ayuda">
            El segundo factor queda en este equipo, cifrado con la maestra. No se envía al control central.
          </p>
          <button
            type="button"
            className="secundario"
            disabled={ocupado}
            onClick={() => {
              void conBoveda(async () => {
                const alta = await invoke<{ secreto: string; otpauth_uri: string }>("enrolar_totp");
                setSecretoTotp(alta.secreto);
                setUriTotp(alta.otpauth_uri);
                setAviso("QR listo. Confirmalo con el código de la app antes de cerrar la bóveda.");
              });
            }}
          >
            Generar TOTP
          </button>
          {uriTotp ? (
            <>
              <div className="qr-totp">
                <QRCodeSVG value={uriTotp} size={180} includeMargin />
              </div>
              <p className="ayuda">
                Escanealo con la app de autenticación del celular. El código de 6 dígitos de esa app es el que confirma y el que pide la próxima apertura.
              </p>
              <details>
                <summary>No puedo escanear</summary>
                <p className="ayuda">Secreto para carga manual: {secretoTotp}</p>
              </details>
              <label className="fila">
                <span>Código para confirmar</span>
                <input
                  value={codigoEnroll}
                  inputMode="numeric"
                  onChange={(e) => setCodigoEnroll(e.target.value)}
                />
              </label>
              <button
                type="button"
                disabled={ocupado || codigoEnroll.trim().length < 6}
                onClick={() => {
                  void conBoveda(async () => {
                    await invoke("confirmar_totp", { codigo: codigoEnroll.trim() });
                    setAviso("TOTP activo. La próxima apertura pide el código de la app.");
                    setCodigoEnroll("");
                    setUriTotp("");
                    setSecretoTotp("");
                  });
                }}
              >
                Confirmar TOTP
              </button>
            </>
          ) : null}
        </div>
      </details>
      <details className="politica">
        <summary>Cambiar contraseña maestra</summary>
        <div className="panel">
          <label className="fila">
            <span>Contraseña actual</span>
            <input
              type="password"
              value={maestraActual}
              onChange={(e) => setMaestraActual(e.target.value)}
              autoComplete="current-password"
            />
          </label>
          <label className="fila">
            <span>Contraseña nueva</span>
            <input
              type="password"
              value={maestraNueva}
              onChange={(e) => setMaestraNueva(e.target.value)}
              autoComplete="new-password"
            />
          </label>
          <button
            type="button"
            disabled={ocupado || maestraActual === "" || maestraNueva === ""}
            onClick={() => {
              void conBoveda(async () => {
                await invoke("cambiar_maestra", { actual: maestraActual, nueva: maestraNueva });
                setMaestraActual("");
                setMaestraNueva("");
                await anotarEvento();
              });
            }}
          >
            Cambiar maestra
          </button>
        </div>
      </details>
      {error ? <p className="error">{error}</p> : null}
      {aviso ? <p className="ayuda">{aviso}</p> : null}
      <Auditoria
        urlCentral={urlCentral}
        carpetaClaves={carpetaClaves}
        agenteId={agenteId}
        enCola={enCola}
        ocupado={ocupado}
        onUrl={setUrlCentral}
        onCarpeta={setCarpetaClaves}
        onReintentar={reintentarEnvio}
        onGuardar={() => {
          void conBoveda(async () => {
            const texto = await invoke<string>("guardar_auditoria", {
              url: urlCentral,
              carpetaClaves,
            });
            setAviso(texto);
            await anotarEvento();
          });
        }}
      />
      <details className="politica">
        <summary>Copia cifrada</summary>
        <div className="panel">
          <p className="ayuda">
            La copia usa otra contraseña, de transporte. No incluye la maestra. Sirve para llevar la bóveda a otro equipo.
          </p>
          <label className="fila">
            <span>Archivo de la copia</span>
            <input
              value={rutaCopia}
              spellCheck={false}
              placeholder={ruta.replace(/\.sqlite$/i, "") + ".gex"}
              onChange={(e) => setRutaCopia(e.target.value)}
            />
          </label>
          <label className="fila">
            <span>Contraseña de transporte</span>
            <input
              type="password"
              value={transporte}
              autoComplete="new-password"
              onChange={(e) => setTransporte(e.target.value)}
            />
          </label>
          <div className="acciones">
            <button
              type="button"
              disabled={ocupado || transporte === ""}
              onClick={() => {
                const destino = (rutaCopia.trim() || ruta.replace(/\.sqlite$/i, "") + ".gex").trim();
                void conBoveda(async () => {
                  await invoke("exportar_boveda", { ruta: destino, transporte });
                  setAviso("Copia cifrada guardada.");
                });
              }}
            >
              Exportar
            </button>
            <button
              type="button"
              className="secundario"
              disabled={ocupado || transporte === ""}
              onClick={() => {
                const origen = (rutaCopia.trim() || ruta.replace(/\.sqlite$/i, "") + ".gex").trim();
                void conBoveda(async () => {
                  const cantidad = await invoke<number>("importar_boveda", { ruta: origen, transporte });
                  await refrescar();
                  setAviso(
                    cantidad === 1 ? "Se importó 1 credencial." : `Se importaron ${cantidad} credenciales.`,
                  );
                });
              }}
            >
              Importar
            </button>
          </div>
        </div>
      </details>
      <div className="layout">
        <section className="panel">
          <h2>Credenciales</h2>
          {contarVencidas(lista) > 0 ? <p className="ayuda">{textoVencidas(contarVencidas(lista))}</p> : null}
          <label className="fila">
            <span>Buscar</span>
            <input
              value={busqueda}
              placeholder="sistema, usuario o categoría"
              onChange={(e) => setBusqueda(e.target.value)}
            />
          </label>
          <label className="fila">
            <span>Filtro</span>
            <select value={filtro} onChange={(e) => setFiltro(e.target.value)}>
              <option value="todas">Todas</option>
              <option value="favoritos">Favoritos</option>
              <option value="vencidas">Vencidas</option>
            </select>
          </label>
          <label className="fila">
            <span>Categoría</span>
            <select value={categoriaFiltro} onChange={(e) => setCategoriaFiltro(e.target.value)}>
              <option value="">Todas</option>
              {[...new Set(lista.map((item) => item.categoria).filter((c) => c !== ""))].map((categoria) => (
                <option key={categoria} value={categoria}>
                  {categoria}
                </option>
              ))}
            </select>
          </label>
          {lista.length === 0 ? <p className="vacio">Todavía no hay credenciales.</p> : null}
          {lista.length > 0 && visibles(lista, busqueda, filtro, categoriaFiltro).length === 0 ? (
            <p className="vacio">Ninguna credencial coincide con el filtro.</p>
          ) : null}
          <ul className="lista">
            {visibles(lista, busqueda, filtro, categoriaFiltro).map((item) => (
              <li key={item.id}>
                <button
                  type="button"
                  className="estrella"
                  title={item.favorito ? "Quitar de favoritos" : "Marcar favorito"}
                  onClick={() => {
                    void conBoveda(async () => {
                      await invoke("marcar_favorito", { id: item.id, favorito: !item.favorito });
                      await refrescar();
                    });
                  }}
                >
                  {item.favorito ? "★" : "☆"}
                </button>
                <button
                  type="button"
                  className={item.id === form.id ? "activa" : undefined}
                  onClick={() => elegir(item.id)}
                >
                  {item.sistema}
                  <br />
                  <small>
                    {item.usuario || "sin usuario"}
                    {textoVence(item.vence_en) ? ` · ${textoVence(item.vence_en)}` : ""}
                  </small>
                </button>
              </li>
            ))}
          </ul>
          <button
            type="button"
            className="secundario"
            onClick={() => {
              setForm(formularioVacio);
              setPolitica(politicaInicial);
              setAviso("");
              setVerSecreto(false);
            }}
          >
            Nueva
          </button>
        </section>
        <form className="panel" onSubmit={guardar}>
          <h2>{form.id === null ? "Alta" : "Editar"}</h2>
          <label className="fila">
            <span>Sistema</span>
            <input
              value={form.sistema}
              onChange={(e) => setForm({ ...form, sistema: e.target.value })}
              onBlur={() => {
                void cargarPolitica(form.sistema).catch((e: unknown) => setError(mensaje(e)));
              }}
            />
          </label>
          <details className="politica">
            <summary>Política de este sistema</summary>
            <label className="fila">
              <span>Modo</span>
              <select
                value={politica.modo}
                onChange={(e) => setPolitica({ ...politica, modo: e.target.value })}
              >
                <option value="aleatoria">Contraseña</option>
                <option value="passphrase">Frase (varias palabras)</option>
              </select>
            </label>
            <label className="fila">
              <span>{politica.modo === "passphrase" ? "Cantidad de palabras" : "Longitud mínima"}</span>
              <input
                type="number"
                min={politica.modo === "passphrase" ? 3 : 4}
                max={politica.modo === "passphrase" ? 8 : 64}
                value={politica.longitud}
                onChange={(e) => setPolitica({ ...politica, longitud: Number(e.target.value) })}
              />
            </label>
            {politica.modo === "aleatoria" ? (
              <div className="checks">
                <label>
                  <input
                    type="checkbox"
                    checked={politica.minusculas}
                    onChange={(e) => setPolitica({ ...politica, minusculas: e.target.checked })}
                  />
                  minúsculas
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={politica.mayusculas}
                    onChange={(e) => setPolitica({ ...politica, mayusculas: e.target.checked })}
                  />
                  mayúsculas
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={politica.digitos}
                    onChange={(e) => setPolitica({ ...politica, digitos: e.target.checked })}
                  />
                  dígitos
                </label>
                <label>
                  <input
                    type="checkbox"
                    checked={politica.simbolos}
                    onChange={(e) => setPolitica({ ...politica, simbolos: e.target.checked })}
                  />
                  símbolos
                </label>
              </div>
            ) : null}
            <label className="fila">
              <span>Expresión regular (opcional)</span>
              <input
                value={politica.regex}
                spellCheck={false}
                onChange={(e) => setPolitica({ ...politica, regex: e.target.value })}
              />
            </label>
            <label className="fila">
              <span>Historial (cuántas anteriores no se pueden repetir)</span>
              <input
                type="number"
                min={0}
                max={20}
                value={politica.historial}
                onChange={(e) => setPolitica({ ...politica, historial: Number(e.target.value) })}
              />
            </label>
            <label className="fila">
              <span>Días de validez (0 = no vence)</span>
              <input
                type="number"
                min={0}
                max={3650}
                value={politica.dias_validez}
                onChange={(e) => setPolitica({ ...politica, dias_validez: Number(e.target.value) })}
              />
            </label>
            <label className="fila">
              <span>Categoría por defecto</span>
              <input
                value={politica.categoria}
                onChange={(e) => setPolitica({ ...politica, categoria: e.target.value })}
              />
            </label>
            <div className="acciones">
              <button type="button" onClick={() => void generar()} disabled={ocupado}>
                Generar
              </button>
              <button type="button" className="secundario" onClick={guardarPolitica} disabled={ocupado}>
                Guardar política
              </button>
            </div>
          </details>
          <label className="fila">
            <span>Usuario</span>
            <input
              value={form.usuario}
              onChange={(e) => setForm({ ...form, usuario: e.target.value })}
            />
          </label>
          <label className="fila">
            <span>Contraseña</span>
            <input
              type={verSecreto ? "text" : "password"}
              value={form.secreto}
              onChange={(e) => setForm({ ...form, secreto: e.target.value })}
              autoComplete="off"
            />
          </label>
          <label className="fila">
            <span>Categoría</span>
            <input
              value={form.categoria}
              onChange={(e) => setForm({ ...form, categoria: e.target.value })}
            />
          </label>
          <label className="fila">
            <span>Notas</span>
            <textarea
              rows={3}
              value={form.notas}
              onChange={(e) => setForm({ ...form, notas: e.target.value })}
            />
          </label>
          <div className="acciones">
            <button type="submit" disabled={ocupado}>
              {form.id === null ? "Agregar" : "Guardar cambios"}
            </button>
            <button type="button" className="secundario" onClick={() => setVerSecreto((v) => !v)}>
              {verSecreto ? "Ocultar" : "Ver"}
            </button>
            {form.id !== null ? (
              <button type="button" className="secundario" onClick={borrar} disabled={ocupado}>
                Borrar
              </button>
            ) : null}
          </div>
        </form>
      </div>
    </main>
  );
}

function Auditoria({
  urlCentral,
  carpetaClaves,
  agenteId,
  enCola,
  ocupado,
  onUrl,
  onCarpeta,
  onReintentar,
  onGuardar,
}: {
  urlCentral: string;
  carpetaClaves: string;
  agenteId: string;
  enCola: number;
  ocupado: boolean;
  onUrl: (valor: string) => void;
  onCarpeta: (valor: string) => void;
  onReintentar: () => void;
  onGuardar: () => void;
}) {
  return (
    <details className="politica">
      <summary>Control central{enCola > 0 ? ` (${enCola} sin enviar)` : ""}</summary>
      <div className="panel">
        <p className="ayuda">
          Agente {agenteId || "…"}. La bóveda no se envía: solo el aviso de alta, cambio, borrado o
          vencimiento. Para que el central lo marque como firmado, pegá la ruta absoluta a
          keys\agentes del repo y tocá Guardar y copiar clave pública antes del primer alta.
        </p>
        <label className="fila">
          <span>URL de eventos</span>
          <input value={urlCentral} spellCheck={false} onChange={(e) => onUrl(e.target.value)} />
        </label>
        <label className="fila">
          <span>Carpeta de claves públicas del central</span>
          <input
            value={carpetaClaves}
            spellCheck={false}
            placeholder="...\tarea1-gestor-contrasenas\keys\agentes"
            onChange={(e) => onCarpeta(e.target.value)}
          />
        </label>
        <button type="button" onClick={onGuardar} disabled={ocupado}>
          Guardar y copiar clave pública
        </button>
        {enCola > 0 ? (
          <>
            <p className="ayuda">
              Hay {enCola} evento{enCola === 1 ? "" : "s"} de auditoría sin enviar, guardados en este
              equipo. La app ya reintenta sola al abrir y al cerrar; este botón lo hace ahora mismo.
            </p>
            <button type="button" onClick={onReintentar} disabled={ocupado}>
              Reintentar envío ({enCola} pendiente{enCola === 1 ? "" : "s"})
            </button>
          </>
        ) : null}
      </div>
    </details>
  );
}
