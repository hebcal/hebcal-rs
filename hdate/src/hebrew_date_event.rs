use crate::{Event, Flags, Hdate};

#[derive(Debug, Clone)]
pub struct HebrewDateEvent(pub Event);

impl HebrewDateEvent {
    pub fn new(date: Hdate) -> Self {
        Self(Event::new(date, date.to_string(), Flags::HebrewDate))
    }
}
