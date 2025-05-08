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
