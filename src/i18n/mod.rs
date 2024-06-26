pub mod de;
pub mod en;
pub mod i18n;

pub use i18n::I18n;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    De,
}
impl Locale {
    pub fn from_str(locale: &str) -> Self {
        match locale {
            "en" => Locale::En,
            "de" => Locale::De,
            _ => Locale::En,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Home,
    About,

    // Weekdays
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,

    // Top bar
    Shiftplan,
    Employees,
    MyTime,
    Logout,

    // Shiftplan
    ShiftplanCalendarWeek,
    ShiftplanTakeLastWeek,
    ShiftplanEditAs,
    ShiftplanYouAre,

    // Employee report
    OverallHeading,
    WorkingHoursPerWeekHeading,
    WorkingHoursPerDayHeading,
    ExtraHoursHeading,

    Balance,
    Required,
    Overall,
    CategoryShiftplan,
    CategoryExtraWork,
    CategoryVacation,
    CategorySickLeave,
    CategoryHolidays,

    ShowDetails,
    HideDetails,

    Hours,

    AddEntry,
    WorkHoursDescription,

    // Add extra hours form
    AddExtraHoursFormTitle,
    Category,
    AmountOfHours,
    Description,
    When,
    Submit,
    Cancel,
}

pub fn generate(locale: Locale) -> I18n<Key, Locale> {
    let mut i18n = I18n::new(locale, Locale::En);

    match locale {
        Locale::En => en::add_i18n_en(&mut i18n),
        Locale::De => de::add_i18n_de(&mut i18n),
    }

    i18n
}

pub type I18nType = I18n<Key, Locale>;
