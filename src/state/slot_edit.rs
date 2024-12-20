use std::rc::Rc;

use rest_types::SlotTO;
use uuid::Uuid;

use super::Weekday;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotEditItem {
    pub id: Uuid,
    pub day_of_week: Weekday,
    pub from: time::Time,
    pub to: time::Time,
    pub min_resources: u8,
    pub valid_from: time::Date,
    pub valid_to: Option<time::Date>,
    pub version: Uuid,
}
impl SlotEditItem {
    pub fn empty() -> Self {
        SlotEditItem {
            id: Uuid::nil(),
            day_of_week: Weekday::Monday,
            from: time::Time::from_hms(0, 0, 0).unwrap(),
            to: time::Time::from_hms(0, 0, 0).unwrap(),
            min_resources: 1,
            valid_from: time::Date::from_calendar_date(0, time::Month::January, 1).unwrap(),
            valid_to: None,
            version: Uuid::nil(),
        }
    }

    pub fn new_valid_from(year: u32, week: u8) -> Self {
        SlotEditItem {
            valid_from: time::Date::from_iso_week_date(year as i32, week, time::Weekday::Monday)
                .unwrap(),
            ..Self::empty()
        }
    }
}
impl From<&SlotTO> for SlotEditItem {
    fn from(slot: &SlotTO) -> Self {
        SlotEditItem {
            id: slot.id,
            day_of_week: slot.day_of_week.into(),
            from: slot.from,
            to: slot.to,
            min_resources: slot.min_resources,
            valid_from: slot.valid_from,
            valid_to: slot.valid_to,
            version: slot.version,
        }
    }
}
impl From<&SlotEditItem> for SlotTO {
    fn from(slot: &SlotEditItem) -> Self {
        SlotTO {
            id: slot.id,
            day_of_week: (&slot.day_of_week).into(),
            from: slot.from,
            to: slot.to,
            min_resources: slot.min_resources,
            valid_from: slot.valid_from,
            valid_to: slot.valid_to,
            deleted: None,
            version: slot.version,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SlotEditType {
    New,
    Edit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotEdit {
    pub slot_edit_type: SlotEditType,
    pub slot: Rc<SlotEditItem>,
    pub visible: bool,
    pub year: u32,
    pub week: u8,
    pub has_errors: bool,
}
impl SlotEdit {
    pub fn new_edit() -> Self {
        SlotEdit {
            slot_edit_type: SlotEditType::Edit,
            slot: SlotEditItem::empty().into(),
            visible: false,
            year: 0,
            week: 0,
            has_errors: false,
        }
    }
}
