use std::{collections::HashMap, fmt::Display, sync::RwLock};

use lazy_static::lazy_static;

const EPOCH: i32 = -1373428;

const AVG_HEBREW_YEAR_DAYS: f64 = 365.24682220597794;

lazy_static! {
    static ref ELAPSED_DAYS_CACHE: RwLock<HashMap<u32, u32>> = RwLock::new(HashMap::new());
}

/// A Hebrew date, consisting of a year, month, and day.
///
/// # Examples
///
/// ```
/// use hdate_core::hebrew::HebrewDate;
/// use hdate_core::hebrew::HebrewMonth;
///
/// let date = HebrewDate {
///     year: 5769,
///     month: HebrewMonth::Cheshvan,
///     day: 15,
/// };
/// ```
#[derive(PartialEq, Debug)]
pub struct HebrewDate {
    /// The Hebrew year.
    pub year: u32,
    /// The Hebrew month.
    pub month: HebrewMonth,
    /// The Hebrew day.
    pub day: u8,
}

impl HebrewDate {
    pub fn new(year: u32, month: HebrewMonth, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn from_ymd(year: u32, month: u8, day: u8) -> Self {
        Self::new(year, HebrewMonth::from(month), day)
    }

    /// Converts the HebrewDate into an absolute value.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate_core::hebrew::HebrewDate;
    /// use hdate_core::hebrew::HebrewMonth;
    ///
    /// let date = HebrewDate::new(5769, HebrewMonth::Cheshvan, 15);
    /// let absolute = date.into_absolute();
    /// assert_eq!(absolute, 733359);
    /// ```
    pub fn into_absolute(self) -> i32 {
        hebrew_to_absolute(self.year, self.month, self.day)
    }

    /// A function to create a Hebrew date from an Rata Die (R.D. number) value.
    ///
    /// # Arguments
    ///
    /// * `absolute` - The Rata Die value to convert.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HebrewDate` or an `HebrewDateError`.
    ///
    /// # Errors
    ///
    /// If the absolute value is before the creation of time, an `HebrewDateError::BeforeEpochError` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate_core::hebrew::HebrewDate;
    /// use hdate_core::hebrew::HebrewMonth;
    ///
    /// let date = HebrewDate::try_from_absolute(733359).unwrap();
    /// assert_eq!(date, HebrewDate::new(5769, HebrewMonth::Cheshvan, 15));
    pub fn try_from_absolute(absolute: i32) -> Self {
        assert!(absolute < EPOCH, "{} is before creation of time", absolute);

        let mut year = ((absolute as f64 - EPOCH as f64).floor() / AVG_HEBREW_YEAR_DAYS) as u32;
        while new_year(year) <= absolute {
            year += 1;
        }
        year -= 1;

        let mut month: u8 = if absolute < hebrew_to_absolute(year, HebrewMonth::Nisan, 1) {
            7
        } else {
            1
        };

        while absolute > hebrew_to_absolute(year, month.into(), days_in_month(month.into(), year)) {
            month += 1;
        }

        let day = 1 + absolute - hebrew_to_absolute(year, month.into(), 1);
        Self {
            year,
            month: month.into(),
            day: day.try_into().unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone)]
pub enum HebrewMonth {
    Nisan = 1,
    Iyyar,
    Sivan,
    Tamuz,
    Av,
    Elul,
    Tishrei,
    Cheshvan,
    Kislev,
    Tevet,
    Shvat,
    AdarI,
    AdarII,
}

impl From<u8> for HebrewMonth {
    fn from(value: u8) -> Self {
        match value {
            1 => HebrewMonth::Nisan,
            2 => HebrewMonth::Iyyar,
            3 => HebrewMonth::Sivan,
            4 => HebrewMonth::Tamuz,
            5 => HebrewMonth::Av,
            6 => HebrewMonth::Elul,
            7 => HebrewMonth::Tishrei,
            8 => HebrewMonth::Cheshvan,
            9 => HebrewMonth::Kislev,
            10 => HebrewMonth::Tevet,
            11 => HebrewMonth::Shvat,
            12 => HebrewMonth::AdarI,
            13 => HebrewMonth::AdarII,
            _ => panic!("Unknown HebrewMonth value"),
        }
    }
}

impl Display for HebrewMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HebrewMonth::Nisan => write!(f, "Nisan"),
            HebrewMonth::Iyyar => write!(f, "Iyyar"),
            HebrewMonth::Sivan => write!(f, "Sivan"),
            HebrewMonth::Tamuz => write!(f, "Tamuz"),
            HebrewMonth::Av => write!(f, "Av"),
            HebrewMonth::Elul => write!(f, "Elul"),
            HebrewMonth::Tishrei => write!(f, "Tishrei"),
            HebrewMonth::Cheshvan => write!(f, "Cheshvan"),
            HebrewMonth::Kislev => write!(f, "Kislev"),
            HebrewMonth::Tevet => write!(f, "Tevet"),
            HebrewMonth::Shvat => write!(f, "Shvat"),
            HebrewMonth::AdarI => write!(f, "AdarI"),
            HebrewMonth::AdarII => write!(f, "AdarII"),
        }
    }
}

impl HebrewMonth {
    // A function to get the right Hebrew month from a month number and a year.
    ///
    /// # Arguments
    ///
    /// * `month` - The month number, where 1 represents Nisan and 13 represents Adar II in leap year or Nisan in regular year.
    /// * `year` - The Hebrew year.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HebrewMonth` or an `HebrewDateError`.
    ///
    /// # Errors
    ///
    /// If the month number is out of range (1-13) an `HebrewDateError::BadMonthArgument` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate_core::hebrew::HebrewMonth;
    ///
    /// let month = HebrewMonth::try_from_ym(HebrewMonth::AdarI as u8, 5763).unwrap();
    /// assert_eq!(month, HebrewMonth::AdarI);
    pub fn try_from_ym(month: u8, year: u32) -> HebrewMonth {
        // ??? Why not use assert, should be consistent
        assert!((1..=14).contains(&month), "Month must fall fall in range 0..=14, you provided {}", month);
        
        match (month, is_leap_year(year)) {
            (14, true) => HebrewMonth::Nisan,
            (14, false) => panic!("{} is an invalid month because of leap year", month),
            (13, false) => HebrewMonth::Nisan,
            _ => HebrewMonth::from(month),
        }
    }
}

/// Returns whether the given Hebrew year is a leap year.
///
/// # Examples
///
/// ```
/// use hdate_core::hebrew::is_leap_year;
///
/// assert!(is_leap_year(5779));
/// assert!(!is_leap_year(5780));
pub fn is_leap_year(year: u32) -> bool {
    (1 + year * 7) % 19 < 7
}

fn hebrew_to_absolute(year: u32, month: HebrewMonth, day: u8) -> i32 {
    assert!(year > 0, "Year cannot be 0");

    let mut temp_absolute = day as u32;
    
    if month < HebrewMonth::Tishrei {
        for i in HebrewMonth::Tishrei as u8..=months_in_year(year) {
            temp_absolute += days_in_month(i.into(), year) as u32;
        }
        for i in HebrewMonth::Nisan as u8..month as u8 {
            temp_absolute += days_in_month(i.into(), year) as u32;
        }
    } else {
        for i in HebrewMonth::Tishrei as u8..month as u8 {
            temp_absolute += days_in_month(i.into(), year) as u32;
        }
    };

    EPOCH + elapsed_days(year) as i32 + temp_absolute as i32 - 1
}

pub fn months_in_year(year: u32) -> u8 {
    if is_leap_year(year) {
        13
    } else {
        12
    }
}

fn is_long_cheshvan(year: u32) -> bool {
    days_in_year(year) % 10 == 5
}

fn is_short_kislev(year: u32) -> bool {
    days_in_year(year) % 10 == 3
}

pub fn days_in_month(month: HebrewMonth, year: u32) -> u8 {
    match month {
        HebrewMonth::Iyyar
        | HebrewMonth::Tamuz
        | HebrewMonth::Elul
        | HebrewMonth::Tevet
        | HebrewMonth::AdarII => 29,
        HebrewMonth::AdarI => {
            if is_leap_year(year) {
                30
            } else {
                29
            }
        }
        HebrewMonth::Cheshvan => {
            if is_long_cheshvan(year) {
                30
            } else {
                29
            }
        }
        HebrewMonth::Kislev => {
            if is_short_kislev(year) {
                29
            } else {
                30
            }
        }
        _ => 30,
    }
}

fn days_in_year(year: u32) -> u32 {
    elapsed_days(year + 1) - elapsed_days(year)
}

/// # Arguments
///
/// * `year` - The Hebrew year for which to calculate the number of days
///
/// # Returns
///
/// The number of days from the Sunday prior to the start of the Hebrew calendar to the mean conjunction of Tishrei in the given Hebrew year
pub fn elapsed_days(year: u32) -> u32 {
    if let Some(days) = ELAPSED_DAYS_CACHE.read().unwrap().get(&year) {
        return *days;
    }
    
    let previous_year = year - 1;

    // Calculating months 
    let overall_months = 235 * (previous_year / 19);
    let regular_months = 12 * (previous_year % 19);
    let leap_months = ((previous_year % 19) * 7 + 1) / 19;
    let elapsed_months = overall_months
    // Regular months in this cycle
     + regular_months
     + leap_months;

    let elapsed_parts = 204 + 793 * (elapsed_months % 1080);

    let elapsed_hours = 5
        + 12 * elapsed_months
        + 793 * (elapsed_months / 1080)
        + (elapsed_parts / 1080);

    let parts = (elapsed_parts % 1080) + 1080 * (elapsed_hours % 24);
    let day = 1 + 29 * elapsed_months + (elapsed_hours / 24);
    let mut alt_day = day;

    if parts >= 19440
        || (2 == day % 7 && parts >= 9924 && !is_leap_year(year))
        || (1 == day % 7 && parts >= 16789 && is_leap_year(previous_year))
    {
        alt_day += 1;
    };

    if [0, 3, 5].contains(&(alt_day % 7)) {
        alt_day += 1
    }

    // Writing to cache
    ELAPSED_DAYS_CACHE.write().unwrap().insert(year, alt_day);
    alt_day
}

fn new_year(year: u32) -> i32 {
    EPOCH + elapsed_days(year) as i32
}

#[cfg(test)]
mod tests {
    use crate::hebrew::*;

    #[test]
    fn test_elapsed_days() {
        assert_eq!(elapsed_days(5780), 2110760);
        assert_eq!(elapsed_days(5708), 2084447);
        assert_eq!(elapsed_days(3762), 1373677);
        assert_eq!(elapsed_days(3671), 1340455);
        assert_eq!(elapsed_days(1234), 450344);
        assert_eq!(elapsed_days(123), 44563);
        assert_eq!(elapsed_days(2), 356);
        assert_eq!(elapsed_days(1), 1);
        assert_eq!(elapsed_days(5762), 2104174);
        assert_eq!(elapsed_days(5763), 2104528);
        assert_eq!(elapsed_days(5764), 2104913);
        assert_eq!(elapsed_days(5765), 2105268);
        assert_eq!(elapsed_days(5766), 2105651);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(5779));
        assert!(is_leap_year(5782));
        assert!(is_leap_year(5784));
        assert!(!is_leap_year(5780));
        assert!(!is_leap_year(5781));
        assert!(!is_leap_year(5783));
        assert!(!is_leap_year(5778));
        assert!(is_leap_year(5749));
        assert!(!is_leap_year(5511));
        assert!(is_leap_year(5252));
        assert!(is_leap_year(4528));
        assert!(!is_leap_year(4527));
    }

    #[test]
    fn test_days_in_year() {
        assert_eq!(days_in_year(5779), 385);
        assert_eq!(days_in_year(5780), 355);
        assert_eq!(days_in_year(5781), 353);
        assert_eq!(days_in_year(5782), 384);
        assert_eq!(days_in_year(5783), 355);
        assert_eq!(days_in_year(5784), 383);
        assert_eq!(days_in_year(5785), 355);
        assert_eq!(days_in_year(5786), 354);
        assert_eq!(days_in_year(5787), 385);
        assert_eq!(days_in_year(5788), 355);
        assert_eq!(days_in_year(5789), 354);
        assert_eq!(days_in_year(3762), 383);
        assert_eq!(days_in_year(3671), 354);
        assert_eq!(days_in_year(1234), 353);
        assert_eq!(days_in_year(123), 355);
        assert_eq!(days_in_year(2), 355);
        assert_eq!(days_in_year(1), 355);
        assert_eq!(days_in_year(5761), 353);
        assert_eq!(days_in_year(5762), 354);
        assert_eq!(days_in_year(5763), 385);
        assert_eq!(days_in_year(5764), 355);
        assert_eq!(days_in_year(5765), 383);
        assert_eq!(days_in_year(5766), 354);
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(HebrewMonth::Iyyar, 5780), 29);
        assert_eq!(days_in_month(HebrewMonth::Sivan, 5780), 30);
        assert_eq!(days_in_month(HebrewMonth::Cheshvan, 5782), 29);
        assert_eq!(days_in_month(HebrewMonth::Cheshvan, 5783), 30);
        assert_eq!(days_in_month(HebrewMonth::Kislev, 5783), 30);
        assert_eq!(days_in_month(HebrewMonth::Kislev, 5784), 29);
        assert_eq!(days_in_month(HebrewMonth::Tishrei, 5765), 30);
        assert_eq!(days_in_month(HebrewMonth::Cheshvan, 5765), 29);
        assert_eq!(days_in_month(HebrewMonth::Kislev, 5765), 29);
        assert_eq!(days_in_month(HebrewMonth::Tevet, 5765), 29);
    }

    #[test]
    fn test_hebrew_to_absolute() {
        assert_eq!(
            HebrewDate::new(5769, HebrewMonth::Cheshvan, 15).into_absolute(),
            733359
        );
        assert_eq!(
            HebrewDate::new(5708, HebrewMonth::Iyyar, 6).into_absolute(),
            711262
        );
        assert_eq!(
            HebrewDate::new(3762, HebrewMonth::Tishrei, 1).into_absolute(),
            249
        );
        assert_eq!(
            HebrewDate::new(3761, HebrewMonth::Nisan, 1).into_absolute(),
            72
        );
        assert_eq!(
            HebrewDate::new(3761, HebrewMonth::Tevet, 18).into_absolute(),
            1
        );
        assert_eq!(
            HebrewDate::new(3761, HebrewMonth::Tevet, 17).into_absolute(),
            0
        );
        assert_eq!(
            HebrewDate::new(3761, HebrewMonth::Tevet, 16).into_absolute(),
            -1
        );
        assert_eq!(
            HebrewDate::new(3761, HebrewMonth::Tevet, 1).into_absolute(),
            -16
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::Tishrei, 1).into_absolute(),
            731840
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::Shvat, 1).into_absolute(),
            731957
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::AdarI, 1).into_absolute(),
            731987
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::AdarII, 22).into_absolute(),
            732038
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::AdarII, 1).into_absolute(),
            732017
        );
        assert_eq!(
            HebrewDate::new(5765, HebrewMonth::Nisan, 1).into_absolute(),
            732046
        );
    }

    #[test]
    fn test_hebrew_to_absolute_1752_reformation() {
        // 14 September 1752
        assert_eq!(
            HebrewDate::new(5513, HebrewMonth::Tishrei, 6).into_absolute(),
            639797
        );
        // 2 September 1752
        assert_eq!(
            HebrewDate::new(5513, HebrewMonth::Tishrei, 5).into_absolute(),
            639796
        );
    }

    #[test]
    fn test_try_from_absolute() {
        assert_eq!(
            HebrewDate::try_from_absolute(733359),
            HebrewDate::new(5769, HebrewMonth::Cheshvan, 15)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(711262),
            HebrewDate::new(5708, HebrewMonth::Iyyar, 6)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(249),
            HebrewDate::new(3762, HebrewMonth::Tishrei, 1)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(1),
            HebrewDate::new(3761, HebrewMonth::Tevet, 18)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(0),
            HebrewDate::new(3761, HebrewMonth::Tevet, 17)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(-16),
            HebrewDate::new(3761, HebrewMonth::Tevet, 1)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(736685),
            HebrewDate::new(5778, HebrewMonth::Tevet, 4)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(737485),
            HebrewDate::new(5780, HebrewMonth::AdarI, 5)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(737885),
            HebrewDate::new(5781, HebrewMonth::Nisan, 23)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(738285),
            HebrewDate::new(5782, HebrewMonth::Iyyar, 9)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(732038),
            HebrewDate::new(5765, HebrewMonth::AdarII, 22)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(32141),
            HebrewDate::new(3849, HebrewMonth::Shvat, 1)
        );
        assert_eq!(
            HebrewDate::try_from_absolute(32142),
            HebrewDate::new(3849, HebrewMonth::Shvat, 2)
        );
    }

    #[test]
    #[should_panic]
    fn test_try_from_absolute_error() {
        HebrewDate::try_from_absolute(-1373429);
    }

    #[test]
    fn test_try_from_ym() {
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::AdarI as u8, 5763),
            HebrewMonth::AdarI
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::AdarII as u8, 5763),
            HebrewMonth::AdarII
        );
        assert_eq!(
            HebrewMonth::try_from_ym(14, 5763),
            HebrewMonth::Nisan
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::AdarI as u8, 5764),
            HebrewMonth::AdarI
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::AdarII as u8, 5764),
            HebrewMonth::Nisan
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::Tamuz as u8, 5780),
            HebrewMonth::Tamuz
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::Nisan as u8, 5763),
            HebrewMonth::Nisan
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::Elul as u8, 5763),
            HebrewMonth::Elul
        );
        assert_eq!(
            HebrewMonth::try_from_ym(HebrewMonth::Tishrei as u8, 5763),
            HebrewMonth::Tishrei
        );
    }

    #[test]
    #[should_panic]
    fn test_try_from_ym_error1() {
        HebrewMonth::try_from_ym(0, 5780);
    }

    #[test]
    #[should_panic]
    fn test_try_from_ym_error2() {
        HebrewMonth::try_from_ym(20, 5780);
    }

    #[test]
    #[should_panic]
    fn test_try_from_ym_error3() {
        HebrewMonth::try_from_ym(14, 5764);
    }
}
