use crate::types::{Schedule, Month};
use std::marker::PhantomData;

pub trait Set {}
pub trait NotSet {}

pub struct Yes;
pub struct No;

impl Set for Yes {}
impl NotSet for No {}

// pub struct Schedule {
//     recurring: Recurring,
//     year: Option<u16>,
//     day: Option<u8>,
//     month: Option<u8>,
//     hour: Option<u8>,
//     minute: Option<u8>,
//     repeat: Option<Until>,
//     range: Option<(Time, Time)>,
// }

pub struct ScheduleBuilder<DaySet, MonthSet, HourSet, MinuteSet> {
    month: Option<Month>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    _month: PhantomData<MonthSet>,
    _day: PhantomData<DaySet>,
    _hour: PhantomData<HourSet>,
    _minute: PhantomData<MinuteSet>,
}

impl ScheduleBuilder<No, No, No, No> {
    pub fn new() -> Self {
        Self {
            month: None,
            day: None,
            hour: None,
            minute: None,
            _month: PhantomData,
            _day: PhantomData,
            _hour: PhantomData,
            _minute: PhantomData,
        }
    }
    pub fn day(self, day: u8) -> ScheduleBuilder<Yes, No, No, No> {
        ScheduleBuilder {
            month: self.month,
            day: Some(day),
            hour: self.hour,
            minute: self.minute,
            _month: PhantomData,
            _day: PhantomData,
            _hour: PhantomData,
            _minute: PhantomData,
        }
    }
}

impl<MonthSet> ScheduleBuilder<Yes, MonthSet, No, No> {
    pub fn hour(self, hour: u8) -> ScheduleBuilder<Yes, MonthSet, Yes, No> {
        ScheduleBuilder {
            month: self.month,
            day: self.day,
            hour: Some(hour),
            minute: self.minute,
            _month: PhantomData,
            _day: PhantomData,
            _hour: PhantomData,
            _minute: PhantomData,
        }
    }
}

impl<MonthSet> ScheduleBuilder<Yes, MonthSet, Yes, No> {
    pub fn minute(self, minute: u8) -> ScheduleBuilder<Yes, MonthSet, Yes, Yes> {
        ScheduleBuilder {
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: Some(minute),
            _month: PhantomData,
            _day: PhantomData,
            _hour: PhantomData,
            _minute: PhantomData,
        }
    }
}
impl<HourSet> ScheduleBuilder<Yes, No, HourSet, No> {
    pub fn month(self, month: Month) -> ScheduleBuilder<Yes, Yes, HourSet, No> {
        ScheduleBuilder {
            month: Some(month),
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            _month: PhantomData,
            _day: PhantomData,
            _hour: PhantomData,
            _minute: PhantomData,
        }
    }
}

impl ScheduleBuilder<Yes,No,No,No> {
    pub fn build(self) -> Schedule {
        Schedule::new().day(self.day.unwrap())
    }
}

impl<MonthSet> ScheduleBuilder<Yes,MonthSet,Yes,Yes> {
    pub fn build(self) -> Schedule {
        let s = Schedule::new().day(self.day.unwrap()).hour(self.hour.unwrap()).minute(self.minute.unwrap());
        if let Some(y) = self.month {
            s.month(y)
        } else {
            s
        }
    }
}

impl ScheduleBuilder<Yes,Yes,No,No> {
    pub fn build(self) -> Schedule {
        Schedule::new().day(self.day.unwrap()).month(self.month.unwrap())
    }
}
