use std::rc::Rc;

use crate::{
    i18n::{self, I18n, Key, Locale},
    state::AuthInfo,
};
use rest_types::{DayOfWeekTO, SlotTO};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl Weekday {
    pub fn i18n_string(&self, i18n: &I18n<Key, Locale>) -> Rc<str> {
        match self {
            Weekday::Monday => i18n.t(Key::Monday),
            Weekday::Tuesday => i18n.t(Key::Tuesday),
            Weekday::Wednesday => i18n.t(Key::Wednesday),
            Weekday::Thursday => i18n.t(Key::Thursday),
            Weekday::Friday => i18n.t(Key::Friday),
            Weekday::Saturday => i18n.t(Key::Saturday),
            Weekday::Sunday => i18n.t(Key::Sunday),
        }
    }
}
impl From<DayOfWeekTO> for Weekday {
    fn from(day_of_week: DayOfWeekTO) -> Self {
        match day_of_week {
            rest_types::DayOfWeekTO::Monday => Weekday::Monday,
            rest_types::DayOfWeekTO::Tuesday => Weekday::Tuesday,
            rest_types::DayOfWeekTO::Wednesday => Weekday::Wednesday,
            rest_types::DayOfWeekTO::Thursday => Weekday::Thursday,
            rest_types::DayOfWeekTO::Friday => Weekday::Friday,
            rest_types::DayOfWeekTO::Saturday => Weekday::Saturday,
            rest_types::DayOfWeekTO::Sunday => Weekday::Sunday,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub id: Uuid,
    pub day_of_week: Weekday,
    pub from: time::Time,
    pub to: time::Time,
}
impl Slot {
    pub fn from_hour(&self) -> f32 {
        self.from.hour() as f32 + self.from.minute() as f32 / 60.0
    }

    pub fn to_hour(&self) -> f32 {
        self.to.hour() as f32 + self.to.minute() as f32 / 60.0
    }
}
impl From<&SlotTO> for Slot {
    fn from(slot: &SlotTO) -> Self {
        Self {
            id: slot.id,
            day_of_week: slot.day_of_week.into(),
            from: slot.from,
            to: slot.to,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shiftplan {
    pub week: u8,
    pub year: u16,
    pub slots: Rc<[Slot]>,
}

impl Shiftplan {
    pub fn slots_by_weekday(&self, weekday: Weekday) -> Rc<[Slot]> {
        self.slots
            .iter()
            .filter(|slot| slot.day_of_week == weekday)
            .cloned()
            .collect()
    }

    pub fn min_hour(&self) -> f32 {
        self.slots
            .iter()
            .map(|slot| slot.from_hour())
            .fold(f32::INFINITY, f32::min)
    }

    pub fn max_hour(&self) -> f32 {
        self.slots
            .iter()
            .map(|slot| slot.to_hour())
            .fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn weekdays(&self) -> Rc<[Weekday]> {
        let mut weekdays = vec![];
        for slot in self.slots.iter() {
            if !weekdays.contains(&slot.day_of_week) {
                weekdays.push(slot.day_of_week.clone());
            }
        }
        weekdays.into()
    }
}
