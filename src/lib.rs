// ## Ergonomic Job Scheduler Library
// ### Part 1: Core Functionality
// Build an ergonomic job scheduling library with an intuitive, fluent public API. Enable tasks to run on:
// - Specific dates/times, eg: 20 Sept 10:00 pm. // MONTH DATE TIME
// - Recurring intervals, eg: hourly, daily, weekly, monthly, every third Saturday // REPEAT a
// - Random intervals, eg: between 9-10 am
// - Repetition: 10 times, until 3rd of March etc.
#![allow(dead_code)]

mod time;

use crate::time::is_valid_day_for_month;

#[derive(Debug)]
pub struct Schedule {
   year: Option<u16>,
   day: Option<u8>,
   month: Option<u8>,
   hour: Option<u8>,
   minute: Option<u8>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            year: None,
            day: None,
            month: None,
            hour: None,
            minute: None,
        }
    }

    pub fn year(mut self, year: u16) -> Self {
        if self.year.is_none(){
            self.year = Some(year)
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
            self.day = Some(d);
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
            self.month = Some(m);
        } else {
            eprintln!("Invalid month: {}. Must be 1–12.", m);
        }
        self
    }

    pub fn hour(mut self, h: u8) -> Self {
        if h < 24 {
            self.hour = Some(h);
        } else {
            eprintln!("Invalid hour: {}. Must be 0–23.", h);
        }
        self
    }

    pub fn minute(mut self, m: u8) -> Self {
        if m < 60 {
            self.minute = Some(m);
        } else {
            eprintln!("Invalid minute: {}. Must be 0–59.", m);
        }
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_set(){
        let s = Schedule::new().day(2);
        assert_eq!(s.day, Some(2));
    }

    #[test]
    fn day_not_set(){
        let s= Schedule::new().day(32);
        assert_eq!(s.day, None);
    }
}
