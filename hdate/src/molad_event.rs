use std::fmt::Display;

use hdate_core::hebrew::months_in_year;

use crate::{Event, Flags, Hdate, HebrewMonth};

const SHORT_DAY_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

pub struct MoladEvent {
    pub event: Event,
    molad: Molad,
}

impl MoladEvent {
    pub fn new(date: Hdate, to_month: HebrewMonth, to_year: u32) -> Self {
        let molad = Molad::new(to_year, to_month);
        let event = Event::new(date, format!("Molad {to_month} {to_year}"), Flags::Molad);
        Self { event, molad }
    }
}

impl Display for MoladEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.molad.fmt(f)
    }
}

struct Molad {
    pub year: u32,
    pub month: HebrewMonth,
    pub day_of_week: u8,
    pub hour: u8,
    pub minute: u8,
    pub parts: u16,
}

impl Molad {
    pub fn new(year: u32, month: HebrewMonth) -> Self {
        let adjusted_month: f32 = month as u8 as f32 - 7.0;
        let adjusted_month = if adjusted_month > 0.0 {
            adjusted_month
        } else {
            adjusted_month + months_in_year(year) as f32
        };

        let last_year = year as f32 - 1.0;
        let overall_months = 235.0 * (last_year / 19.0).floor();
        let regular_months = 12.0 * (last_year % 19.0);
        let leap_months = ((7.0 * (last_year % 19.0) + 1.0) / 19.0).floor();
        let elapsed_months = overall_months + regular_months + leap_months + adjusted_month;

        let elapsed_parts = 204.0 + (793.0 * (elapsed_months % 1080.0)).floor();
        let elapsed_hours = 5.0
            + (12.0 * elapsed_months)
            + (793.0 * (elapsed_months / 1080.0).floor())
            + (elapsed_parts / 1080.0).floor()
            - 6.0;

        let parts = (elapsed_parts % 1080.0) + (1080.0 * (elapsed_hours % 24.0));
        let parts = parts % 1080.0;
        let day = 1.0 + (29.0 * elapsed_months) + (elapsed_hours / 24.0).floor();

        let day_of_week = day % 7.0;
        let hour = elapsed_hours % 24.0;
        let minutes = (parts / 18.0).floor();
        let parts = parts % 18.0;
        Self {
            year,
            month,
            day_of_week: day_of_week as u8,
            hour: hour as u8,
            minute: minutes as u8,
            parts: parts as u16,
        }
    }
}

impl Display for Molad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_name = SHORT_DAY_NAMES[self.day_of_week as usize];
        write!(
            f,
            "Molad {} {}: {}, {} minutes and {} chalakim after {}:00",
            self.month, self.year, day_name, self.minute, self.parts, self.hour
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Hdate;

    use super::*;

    #[test]
    fn test_display() {
        let hd = Hdate::from_ymd(5769, HebrewMonth::Kislev, 23);
        let molad_event = MoladEvent::new(hd, HebrewMonth::Tevet, 5769);
        assert_eq!(molad_event.event.description, "Molad Tevet 5769");
        assert_eq!(
            format!("{molad_event}"),
            "Molad Tevet 5769: Sat, 10 minutes and 16 chalakim after 16:00"
        )
    }
}
