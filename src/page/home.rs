use dioxus::prelude::*;

use crate::component::TopBar;
use crate::router::Route;
use crate::service::AUTH;
use crate::state::auth_info;

#[component]
pub fn Home() -> Element {
    let auth_info = AUTH.read().auth_info.clone()?;
    let nav = navigator();

    if auth_info.privileges.len() == 1 && auth_info.has_privilege("sales") {
        nav.push(Route::ShiftPlan {});
    } else if auth_info.privileges.len() == 1 && auth_info.has_privilege("shiftplanner") {
        nav.push(Route::ShiftPlan {});
    } else if auth_info.privileges.len() == 2
        && auth_info.has_privilege("sales")
        && auth_info.has_privilege("shiftplanner")
    {
        nav.push(Route::ShiftPlan {});
    } else if auth_info.privileges.len() == 1 && auth_info.has_privilege("hr") {
        nav.push(Route::Employees {});
    }

    rsx! {
        TopBar {}
        div { class: "flex place-content-center mt-16",
            div {
                h1 { class: "text-6xl font-bold", "Welcome to Shifty!" }
                p { class: "mt-8 mb-8", "Choose your view from the menu on top of the page." }
                img { src: "/shifty.webp" }
            }
        }
    }
}
