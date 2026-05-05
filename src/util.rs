pub fn fmt_duration(secs: i64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

// Howard Hinnant civil calendar algorithm — correct for all Unix timestamps.
pub fn fmt_datetime(unix_secs: i64) -> String {
    let tod = unix_secs.rem_euclid(86400) as u32;
    let h = tod / 3600;
    let m = (tod % 3600) / 60;

    let z = unix_secs.div_euclid(86400) as i32 + 719468;
    let era = z.div_euclid(146097);
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe as i32 + era * 400 + if month <= 2 { 1 } else { 0 };

    format!("{year}-{month:02}-{day:02} {h:02}:{m:02}")
}
