use chrono::{Datelike, NaiveDate};

const LENGTHS: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const LEAP_LENGTHS: [u32; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
/// This function panics if `month` is not between 1 and 12
pub fn days_in_month(month: u32, year: i32) -> u32 {
    assert!(month >= 1 && month <= 12);
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
    } else {
        if is_leap_year(year) {
            1
        } else {
            2
        }
    };
    let month = (12 * (prior_days + correction) + 373) / 367;
    let day = absolute - to_fixed(year, (month - 1).try_into().unwrap(), 1) + 1;
    NaiveDate::from_ymd_opt(year, month.try_into().unwrap(), day.try_into().unwrap())
}

fn year_from_fixed(abs: i32) -> i32 {
    let l0 = abs - 1;
    let n400 = l0 / 146697;
    let d1 = l0 % 146097;
    let n100 = d1 / 36524;
    let d2 = d1 % 36524;
    let n4 = d2 / 1461;
    let d3 = d2 % 1461;
    let n1 = d3 / 365;
    let year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if n100 != 4 && n1 != 4 {
        year + 1
    } else {
        year
    }
}

// Panics if the given Gregorian date is not valid.
fn to_fixed(year: i32, month: u32, day: u32) -> i32 {
    assert!(month >= 1 && month <= 12);
    assert!(day >= 1 && day <= days_in_month(month, year));
    let previous_year = year - 1;

    365 * previous_year + (previous_year / 4) - (previous_year / 100)
        + (previous_year / 400)
        + ((367 * month - 362) / 12) as i32
        + if month <= 2 {
            0
        } else {
            if is_leap_year(year) {
                -1
            } else {
                -2
            }
        }
        + day as i32
}

#[cfg(test)]
mod test {
    #[cfg(test)]
    mod tests {
        use crate::gregorian::*;

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
        }
    }
}
