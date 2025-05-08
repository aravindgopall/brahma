pub fn is_valid_day_for_month(month: u8, day: u8) -> bool {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11             => day <= 30,
        2                          => day <= 29, 
        _                          => false,
    }
}
