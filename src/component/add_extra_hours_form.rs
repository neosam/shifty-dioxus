use dioxus::prelude::*;
use futures_util::StreamExt;
use time::macros::{date, format_description};
use tracing::info;
use uuid::Uuid;

use crate::{
    api,
    error::{result_handler, ShiftyError},
    i18n::Key,
    js,
    service::{config::CONFIG, i18n::I18N},
    state::employee::WorkingHoursCategory,
};

pub enum AddExtraHoursFormAction {
    Submit,
}

#[derive(Clone, PartialEq, Props)]
pub struct AddExtraHoursFormProps {
    pub sales_person_id: Uuid,
    pub onabort: EventHandler<()>,
    pub onsaved: EventHandler<()>,
}

#[component]
pub fn AddExtraHoursForm(props: AddExtraHoursFormProps) -> Element {
    let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
    let date_format = format_description!("[year]-[month]-[day]");
    let mut category = use_signal(|| WorkingHoursCategory::ExtraWork("".into()));
    let mut amount = use_signal(|| 0.0);
    let mut description = use_signal(|| "".to_string());
    let mut when = use_signal(|| js::current_datetime().format(&format).unwrap());
    let mut from = use_signal(|| js::current_datetime().date().format(&date_format).unwrap());
    let mut to = use_signal(|| js::current_datetime().date().format(&date_format).unwrap());

    let config = CONFIG.read().clone();
    let sales_person_id = props.sales_person_id;

    let i18n = I18N.read().clone();
    let form_title = i18n.t(Key::AddExtraHoursFormTitle);
    let category_str = i18n.t(Key::Category);
    let amount_of_hours_str = i18n.t(Key::AmountOfHours);
    let description_str = i18n.t(Key::Description);
    let when_str = i18n.t(Key::When);
    let submit_str = i18n.t(Key::Submit);
    let cancel_str = i18n.t(Key::Cancel);
    let extra_work_str = i18n.t(Key::CategoryExtraWork);
    let vacation_str = i18n.t(Key::CategoryVacationHours);
    let vacation_days_str = i18n.t(Key::CategoryVacationDays);
    let sick_leave_str = i18n.t(Key::CategorySickLeave);
    let holidays_str = i18n.t(Key::CategoryHolidays);
    let unavailable_str = i18n.t(Key::CategoryUnavailable);

    let cr = use_coroutine(move |mut rx: UnboundedReceiver<AddExtraHoursFormAction>| {
        to_owned![category, amount, description, when, config];
        async move {
            while let Some(action) = rx.next().await {
                match action {
                    AddExtraHoursFormAction::Submit => {
                        let category: WorkingHoursCategory = (*category.read()).clone();
                        let amount = *amount.read();
                        let description = (*description.read()).clone();
                        let when = (*when.read()).clone();

                        if category == WorkingHoursCategory::VacationDays {
                            let amount = amount as i32;
                            let format = format_description!("[year]-[month]-[day]");
                            info!("Adding vacation days: {amount} on {when}");
                            let from = time::Date::parse(&*from.read(), &format)
                                .unwrap_or(date!(1970 - 01 - 01));
                            let to = time::Date::parse(&*to.read(), &format)
                                .unwrap_or(date!(1970 - 01 - 01));
                            result_handler(
                                api::add_vacation(
                                    config.to_owned(),
                                    sales_person_id,
                                    from,
                                    to,
                                    description.into(),
                                )
                                .await
                                .map_err(ShiftyError::from),
                            );
                        } else {
                            result_handler(
                                api::add_extra_hour(
                                    config.to_owned(),
                                    sales_person_id,
                                    amount,
                                    (&category).into(),
                                    description,
                                    when,
                                )
                                .await,
                            );
                        }

                        props.onsaved.call(());
                    }
                }
            }
        }
    });

    rsx! {
        form {
            h1 { class: "text-2xl font-bold", "{form_title}" }

            div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                label { class: "block mt-4 mr-4 grow", "{category_str}" }
                select {
                    class: "block mt-2 pl-2 pr-2 w-full md:w-1/2",
                    value: "{category.read().identifier()}",
                    onchange: move |event| {
                        let value = event.data.value();
                        *category.write() = WorkingHoursCategory::from_identifier(&value);
                    },
                    option { value: "extra_work", "{extra_work_str}" }
                    option { value: "holiday", "{holidays_str}" }
                    option { value: "sick_leave", "{sick_leave_str}" }
                    option { value: "vacation_days", "{vacation_days_str}" }
                    option { value: "unavailable", "{unavailable_str}" }
                    option { disabled: true, "──────────" }
                    option { value: "vacation", "{vacation_str}" }
                }
            }

            div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                label { class: "block mt-4 mr-4 grow", "{description_str}" }
                input {
                    class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                    value: "{description.read()}",
                    onchange: move |event| {
                        let value = event.data.value();
                        *description.write() = value;
                    },
                }
            }

            if *category.read() == WorkingHoursCategory::VacationDays {
                div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                    label { class: "block mt-4 mr-4 grow", "From" }
                    input {
                        class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                        value: "{*from.read()}",
                        onchange: move |event| {
                            let value = event.data.value();
                            info!("Setting when to: {value}");
                            *from.write() = value;
                        },
                        "type": "date",
                    }
                }

                div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                    label { class: "block mt-4 mr-4 grow", "To" }
                    input {
                        class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                        value: "{*to.read()}",
                        onchange: move |event| {
                            let value = event.data.value();
                            info!("Setting when to: {value}");
                            *to.write() = value;
                        },
                        "type": "date",
                    }
                }
            } else {
                div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                    label { class: "block mt-4 mr-4 grow", "{amount_of_hours_str}" }
                    input {
                        class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                        value: "{amount.read()}",
                        onchange: move |event| {
                            let value = event.data.value().parse::<f32>().unwrap_or(0.0);
                            *amount.write() = value;
                        },
                        "type": "number",
                        "step": "0.001",
                    }
                }

                div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1",
                    label { class: "block mt-4 mr-4 grow", "{when_str}" }
                    input {
                        class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                        value: "{*when.read()}",
                        onchange: move |event| {
                            let value = event.data.value();
                            info!("Setting when to: {value}");
                            *when.write() = value;
                        },
                        "type": "datetime-local",
                    }
                }
            }

            div { class: "flex flex-col md:flex-row md:border-b-2 border-gray-300 border-dashed mb-1 mt-8",
                button {
                    class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        props.onabort.call(())
                    },
                    "{cancel_str}"
                }
                button {
                    class: "block mt-2 pl-2 pr-2 border border-black w-full md:w-1/2",
                    onclick: move |event| {
                        event.prevent_default();
                        event.stop_propagation();
                        cr.send(AddExtraHoursFormAction::Submit)
                    },
                    "{submit_str}"
                }
            }
        }
    }
}
