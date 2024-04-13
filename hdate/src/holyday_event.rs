use chrono::NaiveDate;
use hdate_core::hebrew::HebrewMonth;

use crate::{Emoji, Event, Flags, Hdate};

pub struct HolidayEvent(Event);

impl HolidayEvent {
    pub fn new(date: Hdate, description: String, mask: Flags) -> Self {
        Self(Event::new(date, description, mask))
    }

    pub fn get_gregorian_date(&self) -> NaiveDate {
        self.0.date.into()
    }
}

impl Emoji for HolidayEvent {
    fn get_emoji(&self) -> &str {
        if self.0.mask.intersects(Flags::SpecialShabbat) {
            "🕍"
        } else {
            "✡️"
        }
    }
}

pub struct RoshChodeshEvent(pub HolidayEvent);

impl RoshChodeshEvent {
    pub fn new(date: Hdate) -> Self {
        Self(HolidayEvent::new(
            date,
            format!("Rosh Chodesh {}", date.month),
            Flags::RoshChodesh,
        ))
    }
}

impl Emoji for RoshChodeshEvent {
    fn get_emoji(&self) -> &str {
        "🌒"
    }
}

pub struct AsaraBTevetEvent(pub HolidayEvent);

impl AsaraBTevetEvent {
    pub fn new(date: Hdate, mask: Flags) -> Self {
        Self(HolidayEvent::new(
            date,
            format!("Asara B Tevet {}", date.year),
            mask,
        ))
    }
}

pub struct ShabbatMevarchimEvent {
    pub holyday_event: HolidayEvent,
    pub memo: String,
}

impl ShabbatMevarchimEvent {
    pub fn new(date: Hdate, of_month: HebrewMonth, memo: Option<String>) -> Self {
        let holyday_event = HolidayEvent::new(
            date,
            format!("Shabbat Mevarchim {}", of_month),
            Flags::ShabbatMevarchim,
        );
        let memo = memo.unwrap_or_default();
        Self {
            holyday_event,
            memo,
        }
    }
}
