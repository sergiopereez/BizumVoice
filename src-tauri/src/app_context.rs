//! Contexto de la aplicación que va a recibir la transcripción.
//!
//! Un correo no se escribe como un mensaje de chat, ni una nota como un
//! comentario de código. Saber qué aplicación está en primer plano justo antes
//! de escribir permite que el post-procesado adapte el formato al destino.
//!
//! Este módulo se limita a averiguar el dato; qué hacer con él lo decide el
//! prompt de post-procesado, a través de las variables `${app}` y `${bundle}`.

/// La aplicación que tiene el foco en el momento de escribir.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TargetApp {
    /// Nombre visible, tal y como lo muestra el sistema ("Slack", "Mail").
    pub name: String,
    /// Identificador del paquete ("com.tinyspeck.slackmacgap"). Más estable que
    /// el nombre, que cambia con el idioma del sistema.
    pub bundle_id: String,
}

/// Devuelve la aplicación en primer plano, o `None` si el sistema no la expone.
///
/// En macOS se consulta a `NSWorkspace`. No requiere permisos adicionales: es
/// información pública del espacio de trabajo, no lectura del contenido de la
/// ventana ajena.
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
    };

    if target.name.is_empty() && target.bundle_id.is_empty() {
        None
    } else {
        Some(target)
    }
}

/// En el resto de plataformas todavía no está implementado: el post-procesado
/// simplemente no recibe contexto y se comporta como antes.
#[cfg(not(target_os = "macos"))]
pub fn frontmost_app() -> Option<TargetApp> {
    None
}

/// Sustituye las variables de destino en una plantilla de prompt.
///
/// Cuando no se conoce la aplicación —o la detección está desactivada— las
/// variables se resuelven a un texto neutro en lugar de quedar a medias: un
/// prompt con un `${app}` literal confundiría al modelo.
pub fn expand_target_variables(template: &str, target: Option<&TargetApp>) -> String {
    const DESCONOCIDA: &str = "unknown";

    let (name, bundle) = match target {
        Some(app) => (
            if app.name.is_empty() {
                DESCONOCIDA
            } else {
                app.name.as_str()
            },
            if app.bundle_id.is_empty() {
                DESCONOCIDA
            } else {
                app.bundle_id.as_str()
            },
        ),
        None => (DESCONOCIDA, DESCONOCIDA),
    };

    template.replace("${app}", name).replace("${bundle}", bundle)
}

/// Indica si una plantilla usa alguna variable de destino. Sirve para no pagar
/// la consulta al sistema cuando el prompt no la necesita.
pub fn template_uses_target(template: &str) -> bool {
    template.contains("${app}") || template.contains("${bundle}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(name: &str, bundle: &str) -> TargetApp {
        TargetApp {
            name: name.to_string(),
            bundle_id: bundle.to_string(),
        }
    }

    #[test]
    fn sustituye_nombre_y_bundle() {
        let salida = expand_target_variables(
            "Escribiendo en ${app} (${bundle})",
            Some(&app("Slack", "com.tinyspeck.slackmacgap")),
        );
        assert_eq!(salida, "Escribiendo en Slack (com.tinyspeck.slackmacgap)");
    }

    #[test]
    fn sin_destino_usa_texto_neutro() {
        let salida = expand_target_variables("app=${app} bundle=${bundle}", None);
        assert_eq!(salida, "app=unknown bundle=unknown");
    }

    #[test]
    fn campos_vacios_no_dejan_huecos() {
        let salida = expand_target_variables("[${app}]", Some(&app("", "com.ejemplo")));
        assert_eq!(salida, "[unknown]");
    }

    #[test]
    fn detecta_uso_de_variables() {
        assert!(template_uses_target("hola ${app}"));
        assert!(template_uses_target("hola ${bundle}"));
        assert!(!template_uses_target("hola ${output}"));
    }

    /// La consulta corre en el hilo del post-procesado, no en el principal.
    /// Este test existe para detectar que `NSWorkspace` no exija hilo principal:
    /// si lo exigiera, aquí habría un pánico en vez de un resultado.
    #[test]
    fn consultar_el_sistema_no_entra_en_panico() {
        let resultado = frontmost_app();
        if let Some(app) = resultado {
            assert!(
                !app.name.is_empty() || !app.bundle_id.is_empty(),
                "si devuelve algo, debe traer al menos un identificador"
            );
        }
    }

    #[test]
    fn no_toca_otras_variables() {
        let salida = expand_target_variables("${output} en ${app}", Some(&app("Mail", "com.apple.mail")));
        assert_eq!(salida, "${output} en Mail");
    }
}
