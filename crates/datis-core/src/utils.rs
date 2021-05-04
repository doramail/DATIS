pub fn round(n: f64, max_decimal_places: i32) -> f64 {
    if max_decimal_places == 0 {
        return n.round();
    }
    let m = (10.0f64).powi(max_decimal_places);
    (n * m).round() / m
}

pub fn round_hundreds(n: i32) -> i32 {
    (n / 100) * 100
}

static PHONETIC_NUMBERS: &[&str] = &["ZERO", "1", "2", "3", "4", "5", "6", "7", "8", "NINER"];

pub fn pronounce_number<S>(n: S, pronounce: bool) -> String
where
    S: ToString,
{
    if !pronounce {
        return n.to_string();
    }

    n.to_string()
        .chars()
        .map(|c| match c {
            '.' => String::from("DECIMAL"),
            '0'..='9' => String::from(PHONETIC_NUMBERS[c.to_digit(10).unwrap() as usize]),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
