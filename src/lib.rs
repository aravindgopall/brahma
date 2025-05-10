// ## Ergonomic Job Scheduler Library
// ### Part 1: Core Functionality
// Build an ergonomic job scheduling library with an intuitive, fluent public API. Enable tasks to run on:
// - Specific dates/times, eg: 20 Sept 10:00 pm. // MONTH DATE TIME
// - Recurring intervals, eg: hourly, daily, weekly, monthly, every third Saturday // REPEAT a
// - Random intervals, eg: between 9-10 am
// - Repetition: 10 times, until 3rd of March etc.
#![allow(dead_code)]

mod time;
pub mod types;

use crate::types::*;

impl Schedule {
    pub fn monthly(self) -> Schedule {
        self.every(FrequencyPattern::Frequency(Frequency::Monthly))
    }

    pub fn weekly(self) -> Schedule {
        self.every(FrequencyPattern::Frequency(Frequency::Weekly))
    }

    pub fn daily(self) -> Schedule {
        self.every(FrequencyPattern::Frequency(Frequency::Daily))
    }

    pub fn hourly(self) -> Schedule {
        self.every(FrequencyPattern::Frequency(Frequency::Hourly))
    }

    pub fn at(self, hour: u8, minute: u8) -> Schedule {
        self.hour(hour).minute(minute)
    }

    pub fn date(self, month: u8, day: u8) -> Schedule {
        // date would have month as number
        self.month(month).day(day)
    }

    pub fn date_with_time(self, month: u8, day: u8, hour: u8, min: u8) -> Schedule {
        self.month(month).day(day).hour(hour).minute(min)
    }

    pub fn day_with_time(self, day: u8, hour: u8, min: u8) -> Schedule {
        self.day(day).hour(hour).minute(min)
    }

    pub fn on_day(self, day: u8) -> Schedule {
        self.day(day)
    }

    pub fn every_nth_day(self, n: u8, day: Days) -> Schedule {
        self.every(FrequencyPattern::ByDay((Some(n), day)))
    }

    pub fn every_on_day(self, day: Days) -> Schedule {
        self.every(FrequencyPattern::ByDay((None, day)))
    }

    pub fn except_on_date(self, n: u8) -> Schedule {
        self.except(Except::N(n))
    }

    pub fn except_on_nthday(self, n: u8, day: Days) -> Schedule {
        self.except(Except::NthDay((n,day)))
    }

    pub fn except_on_day(self, day: Days) -> Schedule {
        self.except(Except::Day(day))
    }

    pub fn except_on_month(self, month: Month) -> Schedule {
        self.except(Except::Month(month))
    }

    pub fn except_on_month_in_number(self, month: u8) -> Schedule {
        match Month::from_u8(month) {
            Some(m) => self.except(Except::Month(m)),
            None => {
                eprintln!("Invalid month: {}", month);
                self
            }
        }
    }

    pub fn repeat_until_date(self, n: u8, day: u8, month: Month) -> Schedule {
        self.repeat(n).until(Some(day), Some(month), None, None)
    }

    pub fn until_date(self, day: u8, month: u8) -> Schedule {
        match Month::from_u8(month){
            Some(m) => self.until(Some(day),Some(m),None,None),
            None => self
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    // - Specific dates/times, eg: 20 Sept 10:00 pm.
    #[test]
    fn basic_scheduler() {
        let schedule = Schedule::new().date_with_time(9, 20, 22, 00);

        assert_eq!(get_day(&schedule), Some(20));
        assert_eq!(get_hour(&schedule), Some(22));
        assert_eq!(get_month(&schedule), Some(Month::SEP));
    }

    // Monthly schedule on 20th at 10:30 PM (22:30)
    #[test]
    fn recurring_schedule_monthly_on_date() {
        let schedule = Schedule::new().day_with_time(20, 22, 30).monthly();
        assert_eq!(get_day(&schedule), Some(20));
        assert_eq!(get_hour(&schedule), Some(22));
        assert_eq!(
            get_frequency(&schedule).unwrap(),
            FrequencyPattern::Frequency(Frequency::Monthly)
        );
    }

    // - Recurring intervals, eg: hourly, daily, weekly, monthly, every third Saturday
    #[test]
    fn recurring_schedule_daily() {
        let s = Schedule::new().daily();

        assert_eq!(
            get_frequency(&s).unwrap(),
            FrequencyPattern::Frequency(Frequency::Daily)
        );
    }

    #[test]
    fn recurring_schedule_monthly() {
        let s = Schedule::new().monthly();

        assert_eq!(
            get_frequency(&s).unwrap(),
            FrequencyPattern::Frequency(Frequency::Monthly)
        );
    }

    // - Recurring intervals, eg: every third Saturday
    #[test]
    fn recurring_schedule_every_third_sat() {
        let s = Schedule::new().every_nth_day(3, Days::SAT);

        assert_eq!(
            get_frequency(&s).unwrap(),
            FrequencyPattern::ByDay((Some(3), Days::SAT))
        );
    }

    // above test but for all saturday.
    #[test]
    fn recurring_schedule_every_sat() {
        let s = Schedule::new().every_on_day(Days::SAT);

        assert_eq!(
            get_frequency(&s).unwrap(),
            FrequencyPattern::ByDay((None, Days::SAT))
        );
    }

    // all saturday except the 3rd one.
    #[test]
    fn recurring_schedule_every_sat_except() {
        let s = Schedule::new().every_on_day(Days::SAT).except_on_date(3);

        assert_eq!(
            get_frequency(&s).unwrap(),
            FrequencyPattern::ByDay((None, Days::SAT))
        );
        assert_eq!(get_except(&s).unwrap(), Except::N(3));
    }

    // - Random intervals, eg: between 9-10 am
    #[test]
    fn schedule_between() {
        let s = Schedule::new().between((9, 0), (10, 0));
        assert_eq!(
            get_range(&s),
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
        let s = Schedule::new().repeat_until_date(10, 3, Month::MAR);

        let repeat = get_repeat(&s).unwrap();
        assert_eq!(repeat.total, 10);
        assert_eq!(repeat.day, Some(3));
        assert_eq!(repeat.month, Some(Month::MAR));
    }
}
