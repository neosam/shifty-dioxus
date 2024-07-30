use crate::api;
use crate::auth::Auth;
use crate::component::{Footer, TopBar};
use crate::page::NotAuthenticated;
use crate::{i18n, router::Route};
use dioxus::prelude::*;
use web_sys::window;

pub fn App() -> Element {
    let config_resource = use_resource(|| api::load_config());
    let language = web_sys::window()
        .map(|w| w.navigator())
        .and_then(|n| n.language())
        .map(|locale| locale[..2].to_string())
        .unwrap_or_else(|| "en".to_string());
    use_context_provider(|| i18n::generate(i18n::Locale::from_str(&language)));
    match &*config_resource.read_unchecked() {
        Some(Ok(config)) => {
            let title = config.application_title.clone();
            let is_prod = config.is_prod;
            let env_short_description = config.env_short_description.clone();
            use_effect(move || {
                let window = window().unwrap();
                let document = window.document().unwrap();
                if is_prod {
                    document.set_title(title.as_ref());
                } else {
                    document.set_title(format!("{} ({})", title, env_short_description).as_str());
                }
            });
            use_context_provider(|| config.clone());

            rsx! {
                /* Router::<Route> {} */
                div {
                    class: "flex flex-col",
                    Auth {
                        authenticated: {
                            eval(
                                format!("window.oidcLoginKeepAliveURL = '{}/authenticate';", config.backend.clone()).as_str()
                            );
                            rsx! { div {
                                Router::<Route> {}
                            }}
                        },
                        unauthenticated: rsx! {
                            TopBar {}
                            NotAuthenticated {}
                        }
                    }
                    Footer {}
                }
            }
        }
        Some(Err(err)) => {
            rsx! {
                div { "Error while loading technical configuration: {err}" }
            }
        }
        None => {
            rsx! {
                div { "Loading application configuration." }
            }
        }
    }
}
