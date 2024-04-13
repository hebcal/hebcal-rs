pub mod event;
pub mod hdate;
pub mod hebrew_date_event;
pub mod holyday_event;
pub mod molad_event;

pub use event::Event;
pub use event::Flags;
pub use hdate::Hdate;
pub use hdate_core::hebrew::HebrewMonth;
pub use hebrew_date_event::HebrewDateEvent;
pub use holyday_event::HolidayEvent;
pub use molad_event::MoladEvent;

pub trait Emoji {
    fn get_emoji(&self) -> &str;
}
