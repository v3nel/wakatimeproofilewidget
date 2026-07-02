pub fn format_digital_coding_time(
    time: String,
) -> String {
    let Some((hours, minutes)) = time.split_once(":") else {
        return time
    };

    format!("{hours}h {minutes}min")
}