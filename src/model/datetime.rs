use chrono::prelude::*;
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct TimeDiff {
    pub days: i32,
    pub hours: i8,
    pub minutes: i8,
    pub is_negative: bool,
}

#[allow(dead_code)]
impl TimeDiff {
    pub fn to_hours(&self) -> f32 {
        let total = (self.days as f32 * 24.0) + (self.hours as f32) + (self.minutes as f32 / 60.0);
        if self.is_negative {
            -total
        } else {
            total
        }
    }
    pub fn to_minutes(&self) -> i32 {
        let total = (self.days * 24 * 60) + (self.hours as i32 * 60) + (self.minutes as i32);
        if self.is_negative {
            -total
        } else {
            total
        }
    }
    pub fn to_days(&self) -> f32 {
        let total =
            (self.days as f32) + (self.hours as f32 / 24.0) + (self.minutes as f32 / 1440.0);
        if self.is_negative {
            -total
        } else {
            total
        }
    }
    pub fn to_string(&self) -> String {
        let sign = if self.is_negative { "-" } else { "" };
        format!("{}{}d {}h {}m", sign, self.days, self.hours, self.minutes)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub struct Datetime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}
#[allow(dead_code)]
impl Datetime {
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    /// Get the current datetime
    pub fn now() -> Self {
        let now = Local::now();
        Self {
            year: now.year() as u16,
            month: now.month() as u8,
            day: now.day() as u8,
            hour: now.hour() as u8,
            minute: now.minute() as u8,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }

    /// calculate time difference
    pub fn time_diff(&self, other: &Datetime) -> TimeDiff {
        // to minutes
        let self_total_minutes = self.to_total_minutes();
        let other_total_minutes = other.to_total_minutes();

        let diff_minutes = self_total_minutes - other_total_minutes;
        let is_negative = diff_minutes < 0;
        let abs_minutes = diff_minutes.abs();

        let days = (abs_minutes / (24 * 60)) as i32;
        let hours = ((abs_minutes % (24 * 60)) / 60) as i8;
        let minutes = (abs_minutes % 60) as i8;

        TimeDiff {
            days,
            hours,
            minutes,
            is_negative,
        }
    } // use .time_diff(other).to_hours() would be useful

    /// to total minutes
    pub fn to_total_minutes(&self) -> i64 {
        let days_in_month = 30;
        let total_days = (self.year as i64 * 365)
            + ((self.month as i64 - 1) * days_in_month)
            + (self.day as i64);
        let total_minutes = (total_days * 24 * 60) + (self.hour as i64 * 60) + (self.minute as i64);
        total_minutes
    }

    pub fn cmp(&self, other: &Datetime) -> Ordering {
        self.to_total_minutes().cmp(&other.to_total_minutes())
    }
}
