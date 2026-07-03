pub fn format_total_lines(total: i64) -> String {
    if total < 1_000 {
        return total.to_string();
    }

    if total < 1_000_000 {
        return format_compact(total as f64 / 1_000.0, "k");
    }

    format_compact(total as f64 / 1_000_000.0, "M")
}

fn format_compact(value: f64, suffix: &str) -> String {
    format!("{value:.1}{suffix}").replace('.', ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_total_lines_with_compact_suffixes() {
        assert_eq!(format_total_lines(999), "999");
        assert_eq!(format_total_lines(1_000), "1,0k");
        assert_eq!(format_total_lines(193_300), "193,3k");
        assert_eq!(format_total_lines(2_100_000), "2,1M");
    }
}
