use std::rc::Rc;

use dioxus::prelude::*;
use tracing::{error, info};
use web_sys::window;

use crate::{
    service::dropdown::{DropdownAction, DROPDOWN},
    state::dropdown::DropdownEntry,
};

#[component]
pub fn DropdownBase() -> Element {
    let dropdown = DROPDOWN.read().clone();
    let dropdown_service = use_coroutine_handle::<DropdownAction>();
    let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
    use_effect({
        move || {
            let dropdown = DROPDOWN.read().clone();
            if let Some(dropdown) = &dropdown {
                let dropdown_base = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("dropdown-base")
                    .unwrap();
                let dropdown_width = dropdown_base.client_width() as f64;
                let x = if dropdown.x + dropdown_width > width {
                    width - dropdown_width
                } else {
                    dropdown.x
                };
                if let Err(_) = dropdown_base
                    .set_attribute("style", &format!("top: {}px; left: {}px", dropdown.y, x))
                {
                    error!("Failed to set dropdown position");
                }
            }
        }
    });
    if let Some(dropdown) = dropdown.clone() {
        rsx! {
            div {
                class: "absolute inset-0 z-40 bg-gray",
                onclick: {
                    to_owned![dropdown_service];
                    move |_| dropdown_service.send(DropdownAction::CloseDropdown)
                },
                div {
                    class: "absolute z-50 bg-white border border-gray-300 shadow-lg",
                    id: "dropdown-base",
                    for entry in dropdown.entries.iter().filter(|entry| entry.disabled == false).cloned() {
                        div {
                            class: "p-2 cursor-pointer",
                            onclick: {
                                to_owned![dropdown_service, dropdown];
                                move |_| {
                                    (entry.action)(dropdown.context.clone());
                                    dropdown_service.send(DropdownAction::CloseDropdown);
                                }
                            },
                            "{entry.text.clone()}"
                        }
                    }
                }
            }
        }
    } else {
        VNode::empty()
    }
}

#[derive(Clone, Props, PartialEq)]
pub struct DropdownTriggerProps {
    pub children: Element,
    pub entries: Rc<[DropdownEntry]>,
    pub context: Option<Rc<str>>,
}

#[component]
pub fn DropdownTrigger(props: DropdownTriggerProps) -> Element {
    let dropdown_service = use_coroutine_handle::<DropdownAction>();
    rsx! {
        div {
            onclick: move |e| {
                let coordinates = e.data().page_coordinates();
                info!("Clicked at: {:?}", coordinates);
                dropdown_service
                    .send(
                        DropdownAction::ToggleDropdown(
                            coordinates.x,
                            coordinates.y,
                            props.entries.clone(),
                            props.context.clone(),
                        ),
                    );
            },
            {props.children}
        }
    }
}
