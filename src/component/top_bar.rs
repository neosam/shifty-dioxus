use dioxus::prelude::*;

use crate::{
    i18n::{self, Key},
    loader,
    router::Route,
    state::config,
};

#[component]
pub fn TopBar() -> Element {
    let i18n = use_context::<i18n::I18nType>();
    let auth_info = try_use_context::<crate::state::AuthInfo>();
    let config = use_context::<config::Config>();
    let show_my_time = config.show_my_time.unwrap_or(false);
    let backend_url = config.backend.clone();
    let employee = use_resource(move || loader::load_current_sales_person(config.clone()));
    let show_shiftplan = if let Some(ref auth_info) = auth_info {
        auth_info.has_privilege("sales") || auth_info.has_privilege("shiftplanner")
    } else {
        false
    };
    let show_reports = if let Some(ref auth_info) = auth_info {
        auth_info.has_privilege("hr")
    } else {
        false
    };
    let is_paid = if let Some(Ok(Some(employee))) = &*employee.read_unchecked() {
        employee.is_paid
    } else {
        false
    };

    rsx! {
        div {
            class: "flex bg-gray-800 text-white p-4 items-center",
            h1 {
                class: "text-2xl font-bold",
                "Shifty"
            }

            nav {
                class: "flex grow ml-4 justify-between",
                ul {
                    class: "flex space-x-4",
                    if show_shiftplan {
                        li {
                            Link {
                                to: Route::ShiftPlan {},
                                {i18n.t(Key::Shiftplan)}
                            }
                        }
                    }
                    if show_reports {
                        li {
                            Link {
                                to: Route::Employees {},
                                {i18n.t(Key::Employees)}
                            }
                        }
                    }
                    if is_paid && !show_reports && show_my_time {
                        li {
                            Link {
                                to: Route::MyEmployeeDetails {},
                                {i18n.t(Key::MyTime)}
                            }
                        }
                    }
                }
                ul {
                    li {
                        class: "flex",
                        if let Some(auth_info) = auth_info {
                            a {
                                href: "{backend_url}/logout",
                                "Logout {auth_info.user}"
                            }
                        } else {
                            a {
                                href: "/authenticate",
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}
