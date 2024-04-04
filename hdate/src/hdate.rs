use std::cmp::Ordering;

use chrono::{Local, NaiveDate};
use hdate_core::gregorian::gregorian_to_absolute;
use hdate_core::hebrew::{self, HebrewDate, HebrewDateErrors};

use crate::HebrewMonth;

#[derive(Eq, Debug, Clone, Copy)]
pub struct Hdate {
    pub year: u32,
    pub month: HebrewMonth,
    pub day: u8,
    rd: i32,
}

impl Hdate {
    /// Creates a new `Hdate` from the current system date.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::new();
    /// ```
    pub fn new() -> Self {
        // The `expect` function will panic if the conversion fails, which can never happen
        // because `Local::now()` should never be before the creation of time.
        Local::now()
            .date_naive()
            .try_into()
            .expect("How that now it's before the creation of time?")
    }

    /// Creates a new `Hdate` from the given year, month, and day.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// ```
    pub fn from_ymd(year: u32, month: HebrewMonth, day: u8) -> Self {
        let naive = hebrew::HebrewDate::new(year, month, day);
        let rd = naive.into_absolute();
        Self {
            year,
            month,
            day,
            rd,
        }
    }

    /// Returns `true` if the given date in a leap year
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// assert!(hdate.is_leap_year());
    /// ```
    pub fn is_leap_year(&self) -> bool {
        hebrew::is_leap_year(self.year)
    }

    pub fn days_in_month(&self) -> u8 {
        hebrew::days_in_month(self.month, self.year)
    }

    /// Returns the day of the week as a number from 0 to 6, where 0 represents Sunday and 6 represents Saturday.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// assert_eq!(hdate.get_week_day(), 2);
    /// ```
    pub fn get_week_day(&self) -> u8 {
        (self.rd as f32 % 7.0).floor() as u8
    }
}

// Traits implementations

impl Default for Hdate {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Hdate {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.rd == other.rd
    }
}

impl PartialOrd for Hdate {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rd.partial_cmp(&other.rd)
    }
}

impl TryFrom<NaiveDate> for Hdate {
    type Error = HebrewDateErrors;

    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        let rd = gregorian_to_absolute(value);
        let naive_hdate = HebrewDate::try_from_absolute(rd)?;
        Ok(Self {
            year: naive_hdate.year,
            month: naive_hdate.month,
            day: naive_hdate.day,
            rd,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ymd() {
        let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
        assert_eq!(hdate.year, 5782);
        assert_eq!(hdate.month as u8, 7);
        assert_eq!(hdate.day, 1);
    }

    #[test]
    fn test_partial_ord_partial_eq() {
        let hdate1 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
        let hdate2 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
        let hdate3 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 2);
        let hdate4 = Hdate::from_ymd(5782, HebrewMonth::Cheshvan, 2);
        let hdate5 = Hdate::from_ymd(5783, HebrewMonth::Cheshvan, 4);
        let hdate6 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 5);
        assert_eq!(hdate1, hdate2);
        assert_ne!(hdate1, hdate3);
        assert!(hdate3 > hdate2);
        assert!(hdate4 > hdate3);
        assert!(hdate5 > hdate6);
    }

    #[test]
    fn test_get_week_day() {
        let hdate = Hdate::from_ymd(5784, HebrewMonth::AdarII, 22);
        assert_eq!(hdate.get_week_day(), 1);
        let hdate = Hdate::from_ymd(5784, HebrewMonth::AdarII, 27);
        assert_eq!(hdate.get_week_day(), 6);
        let hdate = Hdate::from_ymd(5784, HebrewMonth::AdarII, 28);
        assert_eq!(hdate.get_week_day(), 0);
    }
}
