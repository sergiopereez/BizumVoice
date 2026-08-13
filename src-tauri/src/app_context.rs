//! Contexto de la aplicación que va a recibir la transcripción.
//!
//! Un correo no se escribe como un mensaje de chat, ni una nota como un
//! comentario de código. Saber qué aplicación está en primer plano justo antes
//! de escribir permite que el post-procesado adapte el formato al destino.
//!
//! Este módulo se limita a averiguar el dato; qué hacer con él lo decide el
//! prompt de post-procesado, a través de las variables `${app}`, `${bundle}` y
//! `${window}`.

/// La aplicación que tiene el foco en el momento de escribir.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TargetApp {
    /// Nombre visible de la aplicación ("Slack", "Mail", "chrome").
    pub name: String,
    /// Identificador estable: el bundle id en macOS
    /// ("com.tinyspeck.slackmacgap"), el nombre del ejecutable en Windows
    /// ("slack.exe"). Más fiable que el nombre, que cambia con el idioma.
    pub bundle_id: String,
    /// Título de la ventana en primer plano, cuando la plataforma lo permite.
    ///
    /// Es lo que distingue "Gmail" de cualquier otra pestaña dentro de un
    /// navegador: el sistema solo dice "Google Chrome", pero el título sí lo
    /// delata. Vacío cuando no se puede obtener.
    pub window_title: String,
}

impl TargetApp {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.bundle_id.is_empty() && self.window_title.is_empty()
    }
}

/// macOS: se consulta a `NSWorkspace`. No requiere permisos adicionales, porque
/// es información pública del espacio de trabajo.
///
/// El título de la ventana se queda vacío: leerlo exige la API de
/// Accesibilidad, y aunque la aplicación ya tiene ese permiso concedido para
/// escribir texto, usarlo aquí merece su propio trabajo. En Windows sí viene
/// relleno.
#[cfg(target_os = "macos")]
pub fn frontmost_app() -> Option<TargetApp> {
    use objc2_app_kit::NSWorkspace;

    let workspace = NSWorkspace::sharedWorkspace();
    let app = workspace.frontmostApplication()?;
    let name = app.localizedName().map(|s| s.to_string());
    let bundle_id = app.bundleIdentifier().map(|s| s.to_string());

    let target = TargetApp {
        name: name.unwrap_or_default(),
        bundle_id: bundle_id.unwrap_or_default(),
        window_title: String::new(),
    };

    if target.is_empty() {
        None
    } else {
        Some(target)
    }
}

/// Windows: la ventana en primer plano da tanto el ejecutable que la posee como
/// su título, y ninguno de los dos exige permisos especiales.
#[cfg(target_os = "windows")]
pub fn frontmost_app() -> Option<TargetApp> {
    use windows::Win32::Foundation::{CloseHandle, MAX_PATH};
    use windows::Win32::System::Threading::{
        OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
        PROCESS_QUERY_LIMITED_INFORMATION,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }

        let mut buffer_titulo = [0u16; 512];
        let leidos = GetWindowTextW(hwnd, &mut buffer_titulo);
        let window_title = if leidos > 0 {
            String::from_utf16_lossy(&buffer_titulo[..leidos as usize])
        } else {
            String::new()
        };

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let mut ejecutable = String::new();
        if pid != 0 {
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                let mut buffer = [0u16; MAX_PATH as usize];
                let mut longitud = buffer.len() as u32;
                if QueryFullProcessImageNameW(
                    handle,
                    PROCESS_NAME_WIN32,
                    windows::core::PWSTR(buffer.as_mut_ptr()),
                    &mut longitud,
                )
                .is_ok()
                {
                    let ruta = String::from_utf16_lossy(&buffer[..longitud as usize]);
                    ejecutable = ruta
                        .rsplit(['\\', '/'])
                        .next()
                        .unwrap_or(ruta.as_str())
                        .to_string();
                }
                let _ = CloseHandle(handle);
            }
        }

        // `chrome.exe` se queda en `chrome` para el nombre visible; el
        // identificador conserva la extensión, que es lo que lo hace único.
        let name = ejecutable
            .strip_suffix(".exe")
            .unwrap_or(ejecutable.as_str())
            .to_string();

        let target = TargetApp {
            name,
            bundle_id: ejecutable,
            window_title,
        };

        if target.is_empty() {
            None
        } else {
            Some(target)
        }
    }
}

/// En el resto de plataformas todavía no está implementado: el post-procesado
/// no recibe contexto y se comporta como antes.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn frontmost_app() -> Option<TargetApp> {
    None
}

/// Sustituye las variables de destino en una plantilla de prompt.
///
/// Cuando no se conoce un dato —o la detección no está disponible en esa
/// plataforma— la variable se resuelve a un texto neutro en lugar de quedarse a
/// medias: un `${app}` literal llegando al modelo solo lo confundiría.
pub fn expand_target_variables(template: &str, target: Option<&TargetApp>) -> String {
    const DESCONOCIDO: &str = "unknown";

    fn o_neutro(valor: &str) -> &str {
        if valor.is_empty() {
            DESCONOCIDO
        } else {
            valor
        }
    }

    let (name, bundle, window) = match target {
        Some(app) => (
            o_neutro(&app.name),
            o_neutro(&app.bundle_id),
            o_neutro(&app.window_title),
        ),
        None => (DESCONOCIDO, DESCONOCIDO, DESCONOCIDO),
    };

    template
        .replace("${app}", name)
        .replace("${bundle}", bundle)
        .replace("${window}", window)
}

/// Indica si una plantilla usa alguna variable de destino. Sirve para no pagar
/// la consulta al sistema cuando el prompt no la necesita.
pub fn template_uses_target(template: &str) -> bool {
    template.contains("${app}")
        || template.contains("${bundle}")
        || template.contains("${window}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(name: &str, bundle: &str, window: &str) -> TargetApp {
        TargetApp {
            name: name.to_string(),
            bundle_id: bundle.to_string(),
            window_title: window.to_string(),
        }
    }

    #[test]
    fn sustituye_las_tres_variables() {
        let salida = expand_target_variables(
            "${app} · ${bundle} · ${window}",
            Some(&app("Slack", "com.tinyspeck.slackmacgap", "general - Bizum")),
        );
        assert_eq!(salida, "Slack · com.tinyspeck.slackmacgap · general - Bizum");
    }

    #[test]
    fn sin_destino_usa_texto_neutro() {
        let salida = expand_target_variables("${app}/${bundle}/${window}", None);
        assert_eq!(salida, "unknown/unknown/unknown");
    }

    #[test]
    fn campos_vacios_no_dejan_huecos() {
        // El caso de macOS, donde el título de ventana no se puede leer.
        let salida = expand_target_variables(
            "[${app}][${window}]",
            Some(&app("Mail", "com.apple.mail", "")),
        );
        assert_eq!(salida, "[Mail][unknown]");
    }

    #[test]
    fn detecta_uso_de_variables() {
        assert!(template_uses_target("hola ${app}"));
        assert!(template_uses_target("hola ${bundle}"));
        assert!(template_uses_target("hola ${window}"));
        assert!(!template_uses_target("hola ${output}"));
    }

    #[test]
    fn no_toca_otras_variables() {
        let salida = expand_target_variables(
            "${output} en ${app}",
            Some(&app("Mail", "com.apple.mail", "")),
        );
        assert_eq!(salida, "${output} en Mail");
    }

    /// La consulta corre en el hilo del post-procesado, no en el principal.
    /// Este test existe para detectar que la API del sistema no exija hilo
    /// principal: si lo exigiera, aquí habría un pánico en vez de un resultado.
    #[test]
    fn consultar_el_sistema_no_entra_en_panico() {
        if let Some(app) = frontmost_app() {
            assert!(!app.is_empty(), "si devuelve algo, debe traer algún dato");
        }
    }
}
