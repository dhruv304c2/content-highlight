use chrono::Duration;
use regex::Regex;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum DurationError {
    InvalidFormat,
    ParseError(ParseIntError),
}

pub fn iso8601_duration_to_seconds(iso8601: &str) -> Result<i64, DurationError> {
    let re = Regex::new(r"^P(?:(?P<years>\d+)Y)?(?:(?P<months>\d+)M)?(?:(?P<weeks>\d+)W)?(?:(?P<days>\d+)D)?(?:T(?:(?P<hours>\d+)H)?(?:(?P<minutes>\d+)M)?(?:(?P<seconds>\d+)S)?)?$")
        .map_err(|_| DurationError::InvalidFormat)?;

    if let Some(caps) = re.captures(iso8601) {
        let mut duration = Duration::zero();

        if let Some(years) = caps.name("years") {
            duration = duration + Duration::days(365 * years.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(months) = caps.name("months") {
            duration = duration + Duration::days(30 * months.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(weeks) = caps.name("weeks") {
            duration = duration + Duration::weeks(weeks.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(days) = caps.name("days") {
            duration = duration + Duration::days(days.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(hours) = caps.name("hours") {
            duration = duration + Duration::hours(hours.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(minutes) = caps.name("minutes") {
            duration = duration + Duration::minutes(minutes.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        if let Some(seconds) = caps.name("seconds") {
            duration = duration + Duration::seconds(seconds.as_str().parse::<i64>().map_err(DurationError::ParseError)?);
        }

        Ok(duration.num_seconds())
    } else {
        Err(DurationError::InvalidFormat)
    }
}

pub fn seconds_to_iso8601_duration(seconds: i64) -> String {
    let duration = Duration::seconds(seconds);

    let mut iso_string = String::from("P");

    let days = duration.num_days();
    if days > 0 {
        iso_string.push_str(&format!("{}D", days));
    }

    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let secs = duration.num_seconds() % 60;

    if hours > 0 || minutes > 0 || secs > 0 {
        iso_string.push('T');
        if hours > 0 {
            iso_string.push_str(&format!("{}H", hours));
        }
        if minutes > 0 {
            iso_string.push_str(&format!("{}M", minutes));
        }
        if secs > 0 {
            iso_string.push_str(&format!("{}S", secs));
        }
    }

    iso_string
}
