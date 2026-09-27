# Excel de apoyo al MCU 5.0 — Perfil **Avanzado** (obligatorio)

Estos archivos se descargan/completan conforme al **Marco de Ciberseguridad 5.0 (AGESIC)** y la **Guía de Implementación**. El **perfil objetivo del curso es AVANZADO** (el más exigente de los tres perfiles comunitarios: Básico, Estándar, Avanzado).

## Archivos

| Archivo | Contenido | Quién lo completa |
|---|---|---|
| `01-controles-mcu5-perfil-avanzado.xlsx` | Registro de controles por función (Gobernar/Identificar/Proteger/Detectar/Responder/Recuperar) con **evidencia y forma de demostración** | Blue Team, hoja por hoja a medida que implementa |
| `02-registro-activos-mcu5.xlsx` | Inventario de activos alineado al MCU 5.0 (ID-01) | Blue Team |
| `03-matriz-raci-mcu5.xlsx` | Matriz de responsabilidades por proceso de seguridad | Blue Team (con el dueño/RSI) |
| `04-bitacora-planilla.xlsx` | Bitácora de trabajo diario (Blue y Red Team) | Ambos equipos |

## Reglas de uso

1. **Perfil AVANZADO** obligatorio: el registro de controles refleja el alcance avanzado (más controles y de mayor madurez).
2. Cada control debe completar:
   - **Aplica (Sí/No/N.A.)**: marcar lo que no corresponde con justificación (por ejemplo, controles físicos no aplicarán a una solución virtual; se justifica y se indica "N.A. justificado").
   - **Evidencia**: archivo/captura/registro concreto en `docs/evidencias/`.
   - **Cómo se demuestra**: el procedimiento de demostración en auditoría.
3. Los controles marcados **N.A.** deben estar **justificados y aceptados** (no se eliminan silenciosamente).
4. Entregar el archivo en cada **pre-entrega** y su **versión final congelada** en la entrega.

## Relación con los formatos oficiales de AGESIC

- Los perfiles comunitarios oficiales (Básico/Estándar/Avanzado) se descargan de AGESIC (planillas XLSX).
- Los archivos de esta carpeta son de **apoyo del curso** y complementan (no reemplazan) las planillas oficiales si un grupo las obtiene.
- La Comunicación BCU 2026/098 solicita el **Tipo de dato 957 – Cumplimiento MCU 5.0 – Nivel de Madurez** trimestral: los datos de estas planillas alimentan ese reporte.