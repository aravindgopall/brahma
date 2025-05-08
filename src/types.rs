use crate::time::is_valid_day_for_month;

#[derive(Debug, PartialEq)]
pub enum FrequencyPattern {
    Frequency(Frequency),
    ByDay((Option<u8>, Days)),
}

#[derive(Debug, PartialEq)]
pub enum Frequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, PartialEq)]
pub enum Days {
    SUN,
    MON,
    TUE,
    WED,
    THUR,
    FRI,
    SAT,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Month {
    JAN,
    FEB,
    MAR,
    APR,
    MAY,
    JUN,
    JULY,
    AUG,
    SEPT,
    OCT,
    NOV,
    DEC,
}

#[derive(Debug, PartialEq)]
pub enum Except {
    Day(Days),
    N(u8),
    NDay((u8, Days)),
    MONTH(Month),
}

#[derive(Debug, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug)]
pub struct Recurring {
    frequency: Option<FrequencyPattern>,
    except: Option<Except>,
}

#[derive(Debug)]
pub struct Until {
    total: u8,
    day: Option<u8>,
    month: Option<Month>,
    hr: Option<u8>,
    minute: Option<u8>,
}

#[derive(Debug)]
pub struct Schedule {
    recurring: Recurring,
    year: Option<u16>,
    day: Option<u8>,
    month: Option<Month>,
    hour: Option<u8>,
    minute: Option<u8>,
    repeat: Option<Until>,
    range: Option<(Time, Time)>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            recurring: Recurring {
                frequency: None,
                except: None,
            },
            year: None,
            day: None,
            month: None,
            hour: None,
            minute: None,
            repeat: None,
            range: None,
        }
    }

    pub fn year(mut self, year: u16) -> Self {
        if self.year.is_none() {
            self.year = Some(year);
        } else {
            eprintln!("Year is already set. Ignoring {}", year);
        }
        self
    }

    pub fn day(mut self, d: u8) -> Self {
        if d >= 1 && d <= 31 {
            if let Some(m) = self.month {
                if !is_valid_day_for_month(m as u8, d) {
                    eprintln!("Invalid day {} for month {:?}.", d, m);
                    return self;
                }
            }
            if self.day.is_none() {
                self.day = Some(d);
            } else {
                eprintln!("Day is already set. Ignoring {}", d);
            }
        } else {
            eprintln!("Invalid day: {}. Must be 1–31.", d);
        }
        self
    }

    pub fn month(mut self, m: Month) -> Self {
        if let Some(d) = self.day {
            if !is_valid_day_for_month(m as u8, d) {
                eprintln!("Invalid day {} for month {:?}.", d, m);
                return self;
            }
        }
        if self.month.is_none() {
            self.month = Some(m);
        } else {
            eprintln!("Month is already set. Ignoring {:?}", m);
        }
        self
    }

    pub fn hour(mut self, h: u8) -> Self {
        if self.hour.is_some() {
            eprintln!("Hour is already set. Ignoring {}", h);
            return self;
        }
        if h < 24 {
            self.hour = Some(h);
        } else {
            eprintln!("Invalid hour: {}. Must be 0–23.", h);
        }
        self
    }

    pub fn minute(mut self, m: u8) -> Self {
        if self.minute.is_some() {
            eprintln!("Minute is already set. Ignoring {}", m);
            return self;
        }
        if m < 60 {
            self.minute = Some(m);
        } else {
            eprintln!("Invalid minute: {}. Must be 0–59.", m);
        }
        self
    }

    pub fn every(mut self, f: FrequencyPattern) -> Self {
        if self.recurring.frequency.is_none() {
            self.recurring.frequency = Some(f);
        } else {
            eprintln!("Recurring frequency already set. Ignoring.");
        }
        self
    }

    pub fn except(mut self, e: Except) -> Self {
        if self.recurring.except.is_none() {
            self.recurring.except = Some(e);
        } else {
            eprintln!("Except is already set. Ignoring.");
        }
        self
    }

    pub fn repeat(mut self, n: u8) -> Self {
        if self.repeat.is_none() {
            self.repeat = Some(Until {
                total: n,
                day: None,
                month: None,
                hr: None,
                minute: None,
            });
        } else {
            eprintln!("Repeat count already set. Ignoring {}", n);
        }
        self
    }

    pub fn until(
        mut self,
        d: Option<u8>,
        m: Option<Month>,
        h: Option<u8>,
        min: Option<u8>,
    ) -> Self {
        if self.repeat.is_none() {
            eprintln!("repeat should be invoked before until, ignoring this");
        } else {
            self.repeat = Some(Until {
                total: self.repeat.unwrap().total,
                day: d,
                month: m,
                hr: h,
                minute: min,
            })
        }
        self
    }

    pub fn between(mut self, start: (u8, u8), end: (u8, u8)) -> Self {
        if self.range.is_none() {
            self.range = Some((
                Time {
                    hour: start.0,
                    minute: start.1,
                },
                Time {
                    hour: end.0,
                    minute: end.1,
                },
            ));
        } else {
            eprintln!("Range already set. Ignoring new range.");
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_set() {
        let s = Schedule::new().day(2);
        assert_eq!(s.day, Some(2));
    }

    #[test]
    fn day_not_set() {
        let s = Schedule::new().day(32);
        assert_eq!(s.day, None);
    }

    #[test]
    fn year_set() {
        let s = Schedule::new().year(2025);
        assert_eq!(s.year, Some(2025));
    }

    #[test]
    fn month_set() {
        let s = Schedule::new().month(Month::APR);
        assert_eq!(s.month, Some(Month::APR));
    }

    #[test]
    fn hour_set() {
        let s = Schedule::new().hour(23);
        assert_eq!(s.hour, Some(23));
    }

    #[test]
    fn hour_not_set() {
        let s = Schedule::new().hour(24);
        assert_eq!(s.hour, None);
    }

    #[test]
    fn hour_ignored_on_second_call() {
        let s = Schedule::new().hour(8).hour(10);
        assert_eq!(s.hour, Some(8));
    }

    #[test]
    fn minute_set() {
        let s = Schedule::new().minute(45);
        assert_eq!(s.minute, Some(45));
    }

    #[test]
    fn minute_not_set() {
        let s = Schedule::new().minute(60);
        assert_eq!(s.minute, None);
    }

    #[test]
    fn minute_ignored_on_second_call() {
        let s = Schedule::new().minute(15).minute(45);
        assert_eq!(s.minute, Some(15));
    }

    #[test]
    fn every_frequency_set() {
        let s = Schedule::new().every(FrequencyPattern::Frequency(Frequency::Daily));
        match s.recurring.frequency {
            Some(FrequencyPattern::Frequency(Frequency::Daily)) => {}
            _ => panic!("Expected Frequency::Daily"),
        }
    }

    #[test]
    fn every_byday_set() {
        let s = Schedule::new().every(FrequencyPattern::ByDay((Some(3), Days::SAT)));
        match s.recurring.frequency {
            Some(FrequencyPattern::ByDay((Some(3), Days::SAT))) => {}
            _ => panic!("Expected ByDay((Some(3), SAT))"),
        }
    }

    #[test]
    fn every_ignored_on_second_call() {
        let s = Schedule::new()
            .every(FrequencyPattern::Frequency(Frequency::Hourly))
            .every(FrequencyPattern::Frequency(Frequency::Daily));
        match s.recurring.frequency {
            Some(FrequencyPattern::Frequency(Frequency::Hourly)) => {}
            _ => panic!("Expected Frequency::Hourly to remain"),
        }
    }

    #[test]
    fn except_day_set() {
        let s = Schedule::new().except(Except::Day(Days::MON));
        match s.recurring.except {
            Some(Except::Day(Days::MON)) => {}
            _ => panic!("Expected Except::Day(MON)"),
        }
    }

    #[test]
    fn except_nday_set() {
        let s = Schedule::new().except(Except::NDay((2, Days::FRI)));
        match s.recurring.except {
            Some(Except::NDay((2, Days::FRI))) => {}
            _ => panic!("Expected Except::NDay((2, FRI))"),
        }
    }

    #[test]
    fn except_month_set() {
        let s = Schedule::new().except(Except::MONTH(Month::JAN));
        match s.recurring.except {
            Some(Except::MONTH(Month::JAN)) => {}
            _ => panic!("Expected Except::MONTH(JAN)"),
        }
    }

    #[test]
    fn except_ignored_on_second_call() {
        let s = Schedule::new()
            .except(Except::Day(Days::WED))
            .except(Except::Day(Days::FRI));
        match s.recurring.except {
            Some(Except::Day(Days::WED)) => {}
            _ => panic!("Expected Except::Day(WED) to remain"),
        }
    }

    #[test]
    fn repeat_set() {
        let s = Schedule::new().repeat(10);
        assert_eq!(s.repeat.unwrap().total, 10);
    }

    #[test]
    fn repeat_ignored_on_second_call() {
        let s = Schedule::new().repeat(10).repeat(20);
        assert_eq!(s.repeat.unwrap().total, 10);
    }

    #[test]
    fn until_sets_day_month_hour_minute() {
        let s = Schedule::new()
            .repeat(5)
            .until(Some(3), Some(Month::MAR), Some(10), Some(30));

        let repeat = s.repeat.unwrap();
        assert_eq!(repeat.total, 5);
        assert_eq!(repeat.day, Some(3));
        assert_eq!(repeat.month, Some(Month::MAR));
        assert_eq!(repeat.hr, Some(10));
        assert_eq!(repeat.minute, Some(30));
    }

    #[test]
    fn until_without_repeat_is_ignored() {
        let s = Schedule::new().until(Some(5), Some(Month::JAN), Some(8), Some(45));

        // until should not be set because repeat was not set
        assert!(s.repeat.is_none());
    }

    #[test]
    fn between_set_correctly() {
        let s = Schedule::new().between((9, 0), (10, 0));
        assert_eq!(
            s.range,
            Some((
                Time { hour: 9, minute: 0 },
                Time {
                    hour: 10,
                    minute: 0
                }
            ))
        );
    }

    #[test]
    fn between_ignored_on_second_call() {
        let s = Schedule::new()
            .between((9, 0), (10, 0))
            .between((11, 0), (12, 0));
        assert_eq!(
            s.range,
            Some((
                Time { hour: 9, minute: 0 },
                Time {
                    hour: 10,
                    minute: 0
                }
            ))
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    // - Specific dates/times, eg: 20 Sept 10:00 pm.
    #[test]
    fn basic_scheduler() {
        let schedule = Schedule::new()
                    .month(Month::SEPT)
                    .day(20)
                    .hour(22);

        assert_eq!(schedule.day, Some(20));
        assert_eq!(schedule.hour, Some(22));
        assert_eq!(schedule.month, Some(Month::SEPT));
    }

    // - Recurring intervals, eg: hourly, daily, weekly, monthly, every third Saturday
    #[test]
    fn recurring_schedule_daily() {
        let s = Schedule::new()
              .every(FrequencyPattern::Frequency(Frequency::Daily));

        assert_eq!(
            s.recurring.frequency.unwrap(),
            FrequencyPattern::Frequency(Frequency::Daily)
        );
    }

    #[test]
    fn recurring_schedule_monthly() {
        let s = Schedule::new()
              .every(FrequencyPattern::Frequency(Frequency::Monthly));

        assert_eq!(
            s.recurring.frequency.unwrap(),
            FrequencyPattern::Frequency(Frequency::Monthly)
        );
    }

    // - Recurring intervals, eg: every third Saturday
    #[test]
    fn recurring_schedule_every_third_sat() {
        let s = Schedule::new()
              .every(FrequencyPattern::ByDay((Some(3), Days::SAT)));

        assert_eq!(
            s.recurring.frequency.unwrap(),
            FrequencyPattern::ByDay((Some(3), Days::SAT))
        );
    }

    // above test but for all saturday.
    #[test]
    fn recurring_schedule_every_sat() {
        let s = Schedule::new()
              .every(FrequencyPattern::ByDay((None, Days::SAT)));

        assert_eq!(
            s.recurring.frequency.unwrap(),
            FrequencyPattern::ByDay((None, Days::SAT))
        );
    }

    // all saturday except the 3rd one.
    #[test]
    fn recurring_schedule_every_sat_except() {
        let s = Schedule::new()
            .every(FrequencyPattern::ByDay((None, Days::SAT)))
            .except(Except::N(3));

        assert_eq!(
            s.recurring.frequency.unwrap(),
            FrequencyPattern::ByDay((None, Days::SAT))
        );
        assert_eq!(s.recurring.except.unwrap(), Except::N(3));
    }

    // - Random intervals, eg: between 9-10 am
    #[test]
    fn schedule_between() {
        let s = Schedule::new()
              .between((9, 0), (10, 0));
        assert_eq!(
            s.range,
            Some((
                Time { hour: 9, minute: 0 },
                Time {
                    hour: 10,
                    minute: 0
                }
            ))
        );
    }

    // - Repetition: 10 times, until 3rd of March etc.
    #[test]
    fn until_sets_day_month() {
        let s = Schedule::new()
            .repeat(10)
            .until(Some(3), Some(Month::MAR), None, None); // create until_day_and_month, ...?

        let repeat = s.repeat.unwrap();
        assert_eq!(repeat.total, 10);
        assert_eq!(repeat.day, Some(3));
        assert_eq!(repeat.month, Some(Month::MAR));
    }
}
