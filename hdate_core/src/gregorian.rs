use chrono::{Datelike, NaiveDate};

const LENGTHS: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_LENGTHS: [u32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[inline]
fn quotient(x: i32, y: i32) -> i32 {
    x.div_euclid(y)
}

#[inline]
fn reminder(x: i32, y: i32) -> i32 {
    x.rem_euclid(y)
}

/// # Parameters
///
/// * `year`: The Gregorian year.
///
/// # Returns
///
/// `true` if the given Gregorian year is a leap year, or `false` if it is not.
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// # Parameters
///
/// * `month`: The Gregorian month, between 1 and 12.
/// * `year`: The Gregorian year.
///
/// # Returns
///
/// The number of days in the given month, or `None` if the month or year is out of range.
///
/// # Panics
///
/// This function panics if `month` is not between 1 and 12 || year % 400 == 0
pub fn days_in_month(month: u32, year: i32) -> u32 {
    assert!((1..=12).contains(&month), "Invalid month, {} is not in range 1..=12", month);
    if is_leap_year(year) {
        LEAP_LENGTHS[month as usize]
    } else {
        LENGTHS[month as usize]
    }
}

/// Converts Gregorian date to absolute R.D. (Rata Die) days
///
/// # Parameters
///
/// * `date`: The Gregorian date to convert.
///
/// # Returns
///
/// The absolute R.D. (Rata Die) days since January 1, 4713 BCE.
pub fn gregorian_to_absolute(date: NaiveDate) -> i32 {
    to_fixed(date.year(), date.month(), date.day())
}

/// Converts from Rata Die (R.D. number) to Gregorian date.
pub fn absolute_to_gregorian(absolute: i32) -> Option<NaiveDate> {
    let year = year_from_fixed(absolute);
    let prior_days = absolute - to_fixed(year, 1, 1);
    
    let correction = if absolute < to_fixed(year, 3, 1) {
        0
    } else if is_leap_year(year) {
        1
    } else {
        2
    };
    let month: u32 = quotient(12 * (prior_days + correction) + 373, 367)
        .try_into()
        .unwrap();
    let day: u32 = (absolute - to_fixed(year, month, 1) + 1)
        .try_into()
        .unwrap();
    NaiveDate::from_ymd_opt(year, month, day)
}

fn year_from_fixed(abs: i32) -> i32 {
    let l0 = abs - 1;
    let n400 = quotient(l0, 146097);
    let d1 = reminder(l0, 146097);
    let n100 = quotient(d1, 36524);
    let d2 = reminder(d1, 36524);
    let n4 = quotient(d2, 1461);
    let d3 = reminder(d2, 1461);
    let n1 = quotient(d3, 365);
    
    let year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if n100 != 4 && n1 != 4 {
        year + 1
    } else {
        year
    }
}

// Panics if the given Gregorian date is not valid.
fn to_fixed(year: i32, month: u32, day: u32) -> i32 {
    assert!((1..=12).contains(&month), "Invalid month, {} is not in range 1..=12", month);
    assert!(day >= 1 && day <= days_in_month(month, year), "Invalid day, {} is not valid", day);
    
    let month = month as i32;
    let day = day as i32;
    let previous_year = year - 1;

    365 * previous_year + quotient(previous_year, 4) - quotient(previous_year, 100)
        + quotient(previous_year, 400)
        + quotient(367 * month - 362, 12)
        + if month <= 2 {
            0
        } else if is_leap_year(year) {
            -1
        } else {
            -2
        }
        + day
}

#[cfg(test)]
mod tests {
    use crate::gregorian::*;

    #[test]
    fn test_to_fixed() {
        assert_eq!(to_fixed(-1, 3, 1), -671);
    }

    #[test]
    fn test_gregorian_to_absolute() {
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(1995, 12, 17).unwrap()),
            728644
        );
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(1888, 12, 31).unwrap()),
            689578
        );
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(2005, 4, 2).unwrap()),
            732038
        );
    }

    #[test]
    fn test_gregorian_to_absolute_early_ce() {
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(88, 12, 30).unwrap()),
            32141
        );
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(1, 1, 1).unwrap()),
            1
        );
    }

    #[test]
    fn test_gregorian_to_absolute_negative() {
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(-1, 1, 1).unwrap()),
            -730
        );
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(-100, 12, 20).unwrap()),
            -36536
        );
        assert_eq!(
            gregorian_to_absolute(NaiveDate::from_ymd_opt(-1000, 6, 15).unwrap()),
            -365442
        );
    }

    #[test]
    fn test_absolute_to_gregorian() {
        assert_eq!(
            absolute_to_gregorian(737553).unwrap(),
            NaiveDate::from_ymd_opt(2020, 5, 8).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(689578).unwrap(),
            NaiveDate::from_ymd_opt(1888, 12, 31).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(732038).unwrap(),
            NaiveDate::from_ymd_opt(2005, 4, 2).unwrap()
        );
    }

    #[test]
    fn test_absolute_to_gregorian_88ce() {
        assert_eq!(
            absolute_to_gregorian(32141).unwrap(),
            NaiveDate::from_ymd_opt(88, 12, 30).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(32142).unwrap(),
            NaiveDate::from_ymd_opt(88, 12, 31).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(32143).unwrap(),
            NaiveDate::from_ymd_opt(89, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_absolute_to_gregorian_1ce() {
        assert_eq!(
            absolute_to_gregorian(1).unwrap(),
            NaiveDate::from_ymd_opt(1, 1, 1).unwrap()
        );
    }

    #[test]
    fn test_absolute_to_gregorian_negative() {
        assert_eq!(
            absolute_to_gregorian(-730).unwrap(),
            NaiveDate::from_ymd_opt(-1, 1, 1).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(-36536).unwrap(),
            NaiveDate::from_ymd_opt(-100, 12, 20).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(0).unwrap(),
            NaiveDate::from_ymd_opt(0, 12, 31).unwrap()
        );
        assert_eq!(
            absolute_to_gregorian(-1).unwrap(),
            NaiveDate::from_ymd_opt(0, 12, 30).unwrap()
        );
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2, 2020), 29);
        assert_eq!(days_in_month(2, 2019), 28);
        assert_eq!(days_in_month(5, 2020), 31);
        assert_eq!(days_in_month(2, 2100), 28);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(2019));
        assert!(!is_leap_year(2018));
        assert!(!is_leap_year(2017));
        assert!(is_leap_year(2016));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(1980));
    }
}
