# Bizum Voice

**Dictado por voz en local, sin enviar audio a la nube.**

Bizum Voice es una aplicación de escritorio que transcribe voz a texto: pulsas un
atajo de teclado, hablas, y el texto aparece en el campo donde tengas el cursor.
Todo el procesamiento ocurre en el propio equipo — el audio no sale de la
máquina, lo que la hace apta para tratar información interna.

## Cómo funciona

1. **Pulsa** el atajo configurado para empezar a grabar (o usa el modo
   pulsar-para-hablar).
2. **Habla** mientras el atajo está activo.
3. **Suelta** y la aplicación transcribe con Whisper o Parakeet.
4. **Listo**: el texto se pega en la aplicación que estuvieras usando.

El proceso es enteramente local:

- Los silencios se descartan con detección de actividad de voz (Silero VAD).
- La transcripción usa modelos Whisper (GGML/GGUF) o Parakeet, ejecutados en
  CPU/GPU del propio equipo.
- No hay telemetría ni llamadas a servicios externos durante la transcripción.

## Requisitos de compilación

- **Rust** (toolchain estable) y **CMake** — necesarios para `transcribe-cpp`.
- **Bun** para las dependencias del frontend.
- En macOS, las Command Line Tools de Xcode.

```bash
bun install
bun run tauri dev     # desarrollo
bun run tauri build   # binario distribuible
```

## Identidad visual

La marca está centralizada para que un cambio se propague a toda la interfaz:

| Elemento | Dónde vive |
| --- | --- |
| Paleta de color | `src/styles/theme.css` |
| Tipografía (DM Sans) | `src/styles/fonts.css` + `public/fonts/` |
| Logotipo | `src/components/icons/BizumVoiceLogo.tsx` |
| Icono de la app | `src/components/icons/BizumVoiceMark.tsx` |
| Iconos del sistema | `src-tauri/icons/` |
| Iconos de la bandeja | `src-tauri/resources/tray_*.png` |
| Textos (24 idiomas) | `src/i18n/locales/` |

Los colores provienen de los tokens de marca de Bizum (turquesa `#05C0C7`,
`#088387`, negro `#202020`) y la tipografía es DM Sans, distribuida bajo la SIL
Open Font License 1.1.

## Créditos y licencia

Bizum Voice es un fork de **[Handy](https://github.com/cjpais/Handy)**, creado
por [cjpais](https://github.com/cjpais) y sus colaboradores, distribuido bajo
licencia MIT. Todo el mérito de la arquitectura, el motor de transcripción y la
integración con el sistema es del proyecto original; este fork solo adapta la
capa visual y el nombre.

Este proyecto mantiene la licencia MIT y el aviso de copyright original (ver
[LICENSE](LICENSE)).

El motor de transcripción se apoya además en
[transcribe.cpp](https://github.com/ggerganov/whisper.cpp) y
[ggml](https://github.com/ggerganov/ggml), de Georgi Gerganov y colaboradores.

> "Bizum" es una marca registrada de Sociedad de Procedimientos de Pago, S.L. El
> uso de la marca en este proyecto requiere la autorización del titular.
