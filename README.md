# Brahma — The Ergonomic Job Scheduling Library for Rust

---

## Features

- **Specific Date & Time**: Set day, month, year, hour, and minute
- **Recurring Jobs**: Hourly, Daily, Weekly, Monthly, or every N-th weekday (e.g., 3rd Saturday)
- **Exclusions**: Exclude days (e.g., not on Mondays), specific N-day patterns, or entire months
- **Until Rules**: Stop after N repetitions or on a certain date+time
- **Time Ranges**: Run only within a time window (e.g., between 9:00–10:00 AM)
- **Field-level validation**: Prevents illegal combinations (e.g., Feb 31st)

---

## Installation

### Add to `Cargo.toml` (from github)

```toml
[dependencies]
brahma = { git = "https://github.com/aravindgopall/brahma.git" }
```

---

```rust
use brahma::{Schedule, Frequency, FrequencyPattern, Days, Month, Except};

// Example: Monthly Schedule on 20th at 10:30 PM
let schedule1 = Schedule::new()
    .day_with_time(20, 22, 30)
    .monthly();

// Example: Every 3rd Saturday Until March 3rd at 10:00
let schedule2 = Schedule::new()
    .every_nth_day(3, Days::SAT)
    .until_date(3, 3); // 3 March

// Example: Exclude Specific Days or Months
let schedule3 = Schedule::new()
    .weekly()
    .except_on_nthday(2, Days::FRI)
    .except_on_month(Month::JAN);

// Example: Time Window
let schedule4 = Schedule::new().between((9, 0), (10, 0));
```

## TODO

- [ ] More validations (for eg: leap year).
- [ ] Implement the execute Functionality.
- [ ] Type check to not let invoke multiple times. (currently ignoring with log).
