// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;

/// A value representing an interval of time, in microseconds.
#[doc(alias = "GTimeSpan")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeSpan(pub i64);

impl FromGlib<i64> for TimeSpan {
    unsafe fn from_glib(v: i64) -> TimeSpan {
        TimeSpan(v)
    }
}

impl IntoGlib for TimeSpan {
    type GlibType = i64;

    fn into_glib(self) -> i64 {
        self.0
    }
}

impl TimeSpan {
    /// Create a new timespan from microseconds.
    pub fn from_microseconds(v: i64) -> TimeSpan {
        TimeSpan(v)
    }

    /// Create a new timespan from milliseconds.
    pub fn from_milliseconds(v: i64) -> TimeSpan {
        TimeSpan(v * ffi::G_TIME_SPAN_MILLISECOND)
    }

    /// Create a new timespan from seconds.
    pub fn from_seconds(v: i64) -> TimeSpan {
        TimeSpan(v * ffi::G_TIME_SPAN_SECOND)
    }

    /// Create a new timespan from minutes.
    pub fn from_minutes(v: i64) -> TimeSpan {
        TimeSpan(v * ffi::G_TIME_SPAN_MINUTE)
    }

    /// Create a new timespan from hours.
    pub fn from_hours(v: i64) -> TimeSpan {
        TimeSpan(v * ffi::G_TIME_SPAN_HOUR)
    }

    /// Create a new timespan from days.
    pub fn from_days(v: i64) -> TimeSpan {
        TimeSpan(v * ffi::G_TIME_SPAN_DAY)
    }

    /// Return the full number of microseconds in this `TimeSpan`.
    pub fn as_microseconds(self) -> i64 {
        self.0
    }

    /// Return the full number of milliseconds in this `TimeSpan`.
    pub fn as_milliseconds(self) -> i64 {
        self.0 / ffi::G_TIME_SPAN_MILLISECOND
    }

    /// Return the full number of seconds in this `TimeSpan`.
    pub fn as_seconds(self) -> i64 {
        self.0 / ffi::G_TIME_SPAN_SECOND
    }

    /// Return the full number of minutes in this `TimeSpan`.
    pub fn as_minutes(self) -> i64 {
        self.0 / ffi::G_TIME_SPAN_MINUTE
    }

    /// Return the full number of hours in this `TimeSpan`.
    pub fn as_hours(self) -> i64 {
        self.0 / ffi::G_TIME_SPAN_HOUR
    }

    /// Return the full number of days in this `TimeSpan`.
    pub fn as_days(self) -> i64 {
        self.0 / ffi::G_TIME_SPAN_DAY
    }
}
