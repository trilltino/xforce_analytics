// Utils module - for utility functions

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_large_number(num: f64) -> String {
    if num >= 1_000_000.0 {
        format!("${:.2}M", num / 1_000_000.0)
    } else if num >= 1_000.0 {
        format!("${:.0}K", num / 1_000.0)
    } else {
        format!("${:.2}", num)
    }
}
