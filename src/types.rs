use crate::time::is_valid_day_for_month;

#[derive(Debug)]
pub enum FrequencyPattern {
    Frequency(Frequency),
    ByDay((Option<u8>, Days)),
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub enum Except {
    Day(Days),
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
pub struct Schedule {
    recurring: Recurring,
    year: Option<u16>,
    day: Option<u8>,
    month: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    total: Option<u8>,
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
            total: None,
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
                if !is_valid_day_for_month(m, d) {
                    eprintln!("Invalid day {} for month {}.", d, m);
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

    pub fn month(mut self, m: u8) -> Self {
        if m >= 1 && m <= 12 {
            if let Some(d) = self.day {
                if !is_valid_day_for_month(m, d) {
                    eprintln!("Invalid day {} for month {}.", d, m);
                    return self;
                }
            }
            if self.month.is_none() {
                self.month = Some(m);
            } else {
                eprintln!("Month is already set. Ignoring {}", m);
            }
        } else {
            eprintln!("Invalid month: {}. Must be 1–12.", m);
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
        if self.total.is_none() {
            self.total = Some(n);
        } else {
            eprintln!("Repeat count already set. Ignoring {}", n);
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
        let s = Schedule::new().month(5);
        assert_eq!(s.month, Some(5));
    }

    #[test]
    fn month_not_set() {
        let s = Schedule::new().month(13);
        assert_eq!(s.month, None);
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
        assert_eq!(s.total, Some(10));
    }

    #[test]
    fn repeat_ignored_on_second_call() {
        let s = Schedule::new().repeat(10).repeat(20);
        assert_eq!(s.total, Some(10));
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
