use rest_types::ExtraHoursTO;
use std::rc::Rc;
use tracing::info;
use uuid::Uuid;

use crate::{
    api,
    error::ShiftyError,
    state::{
        employee::{Employee, ExtraHours},
        sales_person_available::SalesPersonUnavailable,
        shiftplan::{Booking, SalesPerson},
        week::Week,
        working_hours::WorkingHoursMini,
        Config, Shiftplan, Slot, Weekday,
    },
};

pub async fn load_sales_persons(config: Config) -> Result<Rc<[SalesPerson]>, ShiftyError> {
    let sales_person_tos = api::get_sales_persons(config).await?;
    let mut sales_persons: Vec<SalesPerson> =
        sales_person_tos.iter().map(SalesPerson::from).collect();
    sales_persons.sort_by_key(|sales_person| sales_person.name.clone());
    let sales_persons: Rc<[SalesPerson]> = sales_persons.into();

    Ok(sales_persons)
}

pub async fn load_bookings(
    config: Config,
    sales_persons: Rc<[SalesPerson]>,
    week: u8,
    year: u32,
) -> Result<Rc<[Booking]>, ShiftyError> {
    let booking_tos = api::get_bookings_for_week(config, week, year).await?;
    let bookings: Rc<[Booking]> = booking_tos
        .iter()
        .map(|booking_to| booking_to.into())
        .map(|booking: Booking| {
            let sales_person = sales_persons
                .iter()
                .find(|sales_person| sales_person.id == booking.sales_person_id);
            if let Some(sales_person) = sales_person {
                Booking {
                    label: sales_person.name.clone(),
                    background_color: sales_person.background_color.clone(),
                    ..booking
                }
            } else {
                booking
            }
        })
        .collect();
    Ok(bookings)
}

pub async fn load_slots(
    config: Config,
    bookings: Rc<[Booking]>,
) -> Result<Rc<[Slot]>, ShiftyError> {
    let slot_tos = api::get_slots(config).await?;
    let slots: Rc<[Slot]> = slot_tos
        .iter()
        .map(|slot_to| slot_to.into())
        .map(|slot: Slot| Slot {
            bookings: bookings
                .iter()
                .filter(|booking| booking.slot_id == slot.id)
                .map(|booking| booking.clone())
                .collect(),
            ..slot
        })
        .collect();
    Ok(slots)
}

pub async fn load_shift_plan(
    config: Config,
    week: u8,
    year: u32,
) -> Result<Shiftplan, ShiftyError> {
    let sales_persons = load_sales_persons(config.clone()).await?;
    let bookings = load_bookings(config.clone(), sales_persons.clone(), week, year).await?;
    let slots = load_slots(config.clone(), bookings.clone()).await?;

    Ok(Shiftplan { week, year, slots })
}

pub async fn load_current_sales_person(config: Config) -> Result<Option<SalesPerson>, ShiftyError> {
    let sales_person_to = api::get_current_sales_person(config).await?;
    let sales_person = sales_person_to.as_ref().map(SalesPerson::from);
    Ok(sales_person)
}

pub async fn register_user_to_slot(
    config: Config,
    slot_id: uuid::Uuid,
    user_id: uuid::Uuid,
    week: u8,
    year: u32,
) -> Result<(), ShiftyError> {
    info!("Add booking");
    api::add_booking(config, user_id, slot_id, week, year).await?;
    Ok(())
}

pub async fn remove_user_from_slot(
    config: Config,
    slot_id: uuid::Uuid,
    user_id: uuid::Uuid,
    shiftplan: Shiftplan,
) -> Result<(), ShiftyError> {
    info!("Remove booking");
    let slot = shiftplan.slots.iter().find(|slot| slot.id == slot_id);
    if let Some(slot) = slot {
        let booking = slot
            .bookings
            .iter()
            .find(|booking| booking.sales_person_id == user_id);
        if let Some(booking) = booking {
            api::remove_booking(config, booking.id).await?;
        }
    }
    Ok(())
}

pub async fn copy_from_previous_week(
    config: Config,
    week: u8,
    year: u32,
) -> Result<(), ShiftyError> {
    info!("Copy from previous week");
    api::copy_week(config, week - 1, year, week, year).await?;
    Ok(())
}

pub async fn load_employees(
    config: Config,
    year: u32,
    week_until: u8,
) -> Result<Rc<[Employee]>, ShiftyError> {
    let report_tos = api::get_short_reports(config, year, week_until).await?;
    Ok(report_tos.iter().map(Employee::from).collect())
}

pub async fn load_employee_details(
    config: Config,
    year: u32,
    week_until: u8,
    employee_id: uuid::Uuid,
) -> Result<Employee, ShiftyError> {
    let report = api::get_employee_reports(config, employee_id, year, week_until).await?;
    Ok(Employee::from(report.as_ref()))
}

pub async fn load_extra_hours_per_year(
    config: Config,
    year: u32,
    employee_id: uuid::Uuid,
) -> Result<Rc<[ExtraHours]>, ShiftyError> {
    let mut extra_hours: Vec<ExtraHoursTO> =
        api::get_extra_hours_for_year(config, employee_id, year, 53)
            .await?
            .iter()
            .cloned()
            .collect();
    extra_hours.sort_by_key(|extra_hours| extra_hours.date_time.clone());
    Ok(extra_hours.iter().map(ExtraHours::from).collect())
}

pub async fn load_weeks(_config: Config, year: u32) -> Result<Rc<[Week]>, ShiftyError> {
    let weeks: Rc<[Week]> = (1..=53).map(|week| Week { year, week }).collect();
    Ok(weeks)
}

pub async fn load_unavailable_sales_person_days_for_week(
    config: Config,
    sales_person_id: Uuid,
    year: u32,
    week: u8,
) -> Result<Rc<[SalesPersonUnavailable]>, ShiftyError> {
    let unavailable_days =
        api::get_unavailable_sales_person_days_for_week(config, sales_person_id, year, week)
            .await?;
    let weeks: Rc<[SalesPersonUnavailable]> = unavailable_days
        .iter()
        .map(SalesPersonUnavailable::from)
        .collect();
    Ok(weeks)
}

pub async fn create_unavailable_sales_person_day(
    config: Config,
    sales_person_id: Uuid,
    year: u32,
    week: u8,
    day: Weekday,
) -> Result<(), ShiftyError> {
    api::create_unavailable_sales_person_day(config, sales_person_id, year, week, (&day).into())
        .await?;
    Ok(())
}

pub async fn delete_unavailable_sales_person_day(
    config: Config,
    unavailable_id: Uuid,
) -> Result<(), ShiftyError> {
    api::delete_unavailable_sales_person_day(config, unavailable_id).await?;
    Ok(())
}

pub async fn load_working_hours_minified_for_week(
    config: Config,
    year: u32,
    week: u8,
    //sales_persons: Rc<[SalesPerson]>,
) -> Result<Rc<[WorkingHoursMini]>, ShiftyError> {
    let reports = api::get_working_hours_minified_for_week(config, year, week).await?;
    Ok(reports
        .iter()
        .map(move |report| WorkingHoursMini {
            sales_person_id: report.sales_person.id,
            sales_person_name: report.sales_person.name.to_owned().as_ref().into(),
            expected_hours: report.expected_hours,
            actual_hours: report.overall_hours,
        })
        .collect())
}
