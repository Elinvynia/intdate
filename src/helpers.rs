pub(crate) fn get_day_part(date: u32) -> u8 {
    (date % 100) as u8
}

pub(crate) fn get_month_part(date: u32) -> u8 {
    ((date / 100) % 100) as u8
}

pub(crate) fn get_year_part(date: u32) -> u32 {
    (date / 100) / 100
}

// Allowed to make the code clearer.
#[allow(clippy::needless_bool)]
pub(crate) fn is_year_leap(date: u32) -> bool {
    let year = get_year_part(date);
    if year % 4 != 0 {
        false
    } else if year % 100 != 0 {
        true
    } else if year % 400 != 0 {
        false
    } else {
        true
    }
}

pub(crate) fn is_month_valid(date: u32) -> bool {
    let month = get_month_part(date);
    (1..=12).contains(&month)
}

pub(crate) fn is_day_valid(date: u32) -> bool {
    let day = get_day_part(date);
    if day == 0 || day > 31 {
        return false;
    }

    match get_month_part(date) {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => day <= 31,
        4 | 6 | 9 | 11 => day <= 30,
        2 => {
            if is_year_leap(date) {
                day <= 29
            } else {
                day <= 28
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_days() {
        // Valid days
        assert!(is_day_valid(20001225));

        // Invalid days
        assert!(!is_day_valid(20001200));
        assert!(!is_day_valid(20001232));
        assert!(!is_day_valid(20020229));
    }

    #[test]
    fn validate_months() {
        // Valid months
        assert!(is_month_valid(20000100));
        assert!(is_month_valid(20000200));
        assert!(is_month_valid(20000300));
        assert!(is_month_valid(20000400));
        assert!(is_month_valid(20000500));
        assert!(is_month_valid(20000600));
        assert!(is_month_valid(20000700));
        assert!(is_month_valid(20000800));
        assert!(is_month_valid(20000900));
        assert!(is_month_valid(20001000));
        assert!(is_month_valid(20001100));
        assert!(is_month_valid(20001200));

        // Invalid months
        assert!(!is_month_valid(20000000));
        assert!(!is_month_valid(20001300));
        assert!(!is_month_valid(20009900));
    }

    #[test]
    fn check_parts() {
        assert_eq!(get_day_part(20201222), 22);
        assert_eq!(get_month_part(20001122), 11);
        assert_eq!(get_year_part(99990101), 9999);
    }

    #[test]
    fn leap_years() {
        // Leap years
        assert!(is_year_leap(20040101));
        assert!(is_year_leap(24000101));

        // Common years
        assert!(!is_year_leap(21000101));
        assert!(!is_year_leap(20210101));
    }
}
