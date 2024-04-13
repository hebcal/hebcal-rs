use std::cmp::Ordering;
use std::fmt::Display;
use std::ops::{Add, Sub};

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

    /// Returns the difference in days between the two given HDates.
    /// The result is positive if `self` date is comes chronologically
    /// after the `other` date, and negative
    /// if the order of the two dates is reversed.
    ///
    /// # Examples
    ///
    /// let hdate1 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// let hdate2 = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 2);
    /// let days = hdate1.delta_days(hdate2);
    /// assert_eq!(days, 1);
    /// ```
    pub fn delta_days(&self, other: Self) -> i32 {
        self.rd - other.rd
    }
}

// Traits implementations

impl Default for Hdate {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<i32> for Hdate {
    type Output = Self;
    /// Adds an integer number of days to an Hdate and returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// let result = hdate + 40;
    /// assert_eq!(result.year, 5782);
    /// assert_eq!(result.month, HebrewMonth::Cheshvan);
    /// assert_eq!(result.day, 11);
    /// ```
    fn add(self, rhs: i32) -> Self::Output {
        let rd = self.rd + rhs;

        let naive_hdate = HebrewDate::try_from_absolute(rd).unwrap();
        Hdate {
            year: naive_hdate.year,
            month: naive_hdate.month,
            day: naive_hdate.day,
            rd,
        }
    }
}

impl Sub<i32> for Hdate {
    type Output = Self;

    /// Subtracts an integer number of days from an Hdate and returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth};
    ///
    /// let hdate = Hdate::from_ymd(5782, HebrewMonth::Tishrei, 1);
    /// let result = hdate - 40;
    /// assert_eq!(result.year, 5781);
    /// assert_eq!(result.month, HebrewMonth::Av);
    /// assert_eq!(result.day, 20);
    /// ```
    fn sub(self, rhs: i32) -> Self::Output {
        self + -rhs
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

impl From<Hdate> for NaiveDate {
    fn from(value: Hdate) -> Self {
        hdate_core::gregorian::absolute_to_gregorian(value.rd).unwrap()
    }
}

impl Display for Hdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.day, self.month, self.year)
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

    #[test]
    fn test_delta_days() {
        let hdate1 = Hdate::from_ymd(5770, HebrewMonth::Kislev, 25);
        let hdate2 = Hdate::from_ymd(5769, HebrewMonth::Cheshvan, 15);
        assert_eq!(hdate1.delta_days(hdate2), 394);
        assert_eq!(hdate2.delta_days(hdate1), -394);
        assert_eq!(hdate1.delta_days(hdate1), 0);
    }

    #[test]
    fn test_into_naive_date() {
        let hdate = Hdate::from_ymd(5784, HebrewMonth::AdarII, 26);
        let gregorian_date = NaiveDate::from_ymd_opt(2024, 4, 5).unwrap();
        assert_eq!(Into::<NaiveDate>::into(hdate), gregorian_date);
    }
}
