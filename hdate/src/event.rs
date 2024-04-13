use crate::Hdate;
use bitflags::bitflags;
use chrono::NaiveDate;

/// Holiday flags for Event
#[derive(Clone, Debug, PartialEq)]
pub struct Flags(u32);

bitflags! {
  impl Flags: u32 {
      const None = 0;
      /// Chag, yontiff, yom tov
      const Chag = 0x000001;
      /// Light candles 18 minutes before sundown
      const LightCandles = 0x000002;
      // End of holiday (end of Yom Tov)
      const YomTovEnds = 0x000004;
      /// Observed only in the Diaspora
      const ChulOnly = 0x000008;
      /// Observed only in Israel
      const IsraelOnly = 0x000010;
      /// Light candles in the evening at Tzeit time (3 small stars)
      const LightCandlesTzeis = 0x000020;
      /// Candle-lighting for Chanukah
      const ChanukahCandles = 0x000040;
      /// Rosh Chodesh; beginning of a new Hebrew month
      const RoshChodesh = 0x000080;
      /// Minor fasts like Tzom Tammuz; Ta'anit Esther, ...
      const MinorFast = 0x000100;
      /// Shabbat Shekalim, Zachor, ...
      const SpecialShabbat = 0x000200;
      /// Weekly sedrot on Saturdays
      const ParshaHashavua = 0x000400;
      /// Daily page of Talmud (Bavli)
      const DafYomi = 0x000800;
      /// Days of the Omer
      const OmerCount = 0x001000;
      /// Yom HaShoah, Yom HaAtzma'ut, ...
      const ModernHoliday = 0x002000;
      /// Yom Kippur and Tish'a B'Av
      const MajorFast = 0x004000;
      /// On the Saturday before Rosh Chodesh
      const ShabbatMevarchim = 0x008000;
      /// Molad
      const Molad = 0x010000;
      /// Yahrzeit or Hebrew Anniversary
      const UserEvent = 0x020000;
      /// Daily Hebrew date ("11th of Sivan, 5780")
      const HebrewDate = 0x040000;
      /// A holiday that's not major, modern, rosh chodesh, or a fast day
      const MinorHoliday = 0x080000;
      /// Evening before a major or minor holiday
      const Erev = 0x100000;
      /// Chol haMoed, intermediate days of Pesach or Sukkot
      const CholHamoed = 0x200000;
      /// Mishna Yomi
      const MishnaYomi = 0x400000;
      /// Yom Kippur Katan, minor day of atonement on the day preceeding each Rosh Chodesh
      const YomKippurKatan = 0x800000;
      /// Daily page of Jerusalem Talmud (Yerushalmi)
      const YerushalmiYomi = 0x1000000;
      /// Nach Yomi
      const NachYomi = 0x2000000;
      /// Daily Learning
      const DailyLearning = 0x4000000;
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub date: Hdate,
    pub description: String,
    pub mask: Flags,
}

impl Event {
    pub fn new(date: Hdate, description: String, mask: Flags) -> Self {
        Self {
            date,
            description,
            mask,
        }
    }

    /// Returns whether the event is observed in Israel.
    ///
    /// # Examples
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth, Event, Flags};
    ///
    /// let event = Event::new(
    ///     Hdate::from_ymd(5749, HebrewMonth::Sivan, 7),
    ///     "Shavuot II".to_string(),
    ///     Flags::Chag | Flags::ChulOnly,
    /// );
    ///
    /// assert!(!event.observed_in_israel());
    ///
    /// let event = Event::new(
    ///     Hdate::from_ymd(5749, HebrewMonth::Kislev, 26),
    ///     "Chanukah: 3 Candles".to_string(),
    ///     Flags::None
    /// );
    ///
    /// assert!(event.observed_in_israel());
    /// ```
    pub fn observed_in_israel(&self) -> bool {
        !self.mask.intersects(Flags::ChulOnly)
    }

    /// Returns whether the event is observed in the Diaspora.
    ///
    /// # Example
    ///
    /// ```
    /// use hdate::{Hdate, HebrewMonth, Event, Flags};
    ///
    /// let event = Event::new(
    ///     Hdate::from_ymd(5749, HebrewMonth::Sivan, 7),
    ///     "Shavuot II".to_string(),
    ///     Flags::Chag | Flags::ChulOnly,
    /// );
    ///
    /// assert!(event.observed_in_diaspora());
    ///
    /// let event = Event::new(
    ///     Hdate::from_ymd(5749, HebrewMonth::Kislev, 26),
    ///     "Chanukah: 3 Candles".to_string(),
    ///     Flags::None
    /// );
    ///
    /// assert!(event.observed_in_diaspora());
    /// ```
    pub fn observed_in_diaspora(&self) -> bool {
        !self.mask.intersects(Flags::IsraelOnly)
    }

    pub fn get_gregorian_date(&self) -> NaiveDate {
        self.date.into()
    }
}
