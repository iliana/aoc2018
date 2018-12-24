use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};
use failure::Fallible;
use std::cmp::Ordering;
use std::fmt;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub fn read_events() -> impl Iterator<Item = Fallible<Event>> {
    BufReader::new(io::stdin()).lines().map(|line| {
        line.map_err(failure::Error::from)
            .and_then(|line| line.parse().map_err(failure::Error::from))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

impl Time {
    pub fn range(self, other: Time) -> impl Iterator<Item = Time> {
        (0..(other - self)).map(move |offset| self + offset)
    }
}

impl FromStr for Time {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Time, ParseIntError> {
        let mut iter = s.split(|c: char| !c.is_numeric());
        Ok(Time {
            year: iter.next().unwrap().parse()?,
            month: iter.next().unwrap().parse()?,
            day: iter.next().unwrap().parse()?,
            hour: iter.next().unwrap().parse()?,
            minute: iter.next().unwrap().parse()?,
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

impl From<Time> for NaiveDateTime {
    fn from(t: Time) -> NaiveDateTime {
        NaiveDate::from_ymd(t.year, t.month, t.day).and_hms(t.hour, t.minute, 0)
    }
}

impl From<NaiveDateTime> for Time {
    fn from(t: NaiveDateTime) -> Time {
        Time {
            year: t.year(),
            month: t.month(),
            day: t.day(),
            hour: t.hour(),
            minute: t.minute(),
        }
    }
}

impl Add<i64> for Time {
    type Output = Time;

    fn add(self, minutes: i64) -> Time {
        (NaiveDateTime::from(self) + Duration::minutes(minutes)).into()
    }
}

impl Sub for Time {
    type Output = i64;

    fn sub(self, other: Time) -> i64 {
        let duration = NaiveDateTime::from(self).sub(NaiveDateTime::from(other));
        duration.num_minutes()
    }
}

#[derive(Debug)]
pub struct Event {
    pub time: Time,
    pub what: What,
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time.eq(&other.time)
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl FromStr for Event {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Event, ParseIntError> {
        let mut iter = s.split("] ");
        Ok(Event {
            time: iter.next().unwrap().trim_start_matches('[').parse()?,
            what: iter.next().unwrap().parse()?,
        })
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.time, self.what)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum What {
    Begins(u16),
    FallsAsleep,
    WakesUp,
}

impl FromStr for What {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<What, ParseIntError> {
        Ok(match s {
            "falls asleep" => What::FallsAsleep,
            "wakes up" => What::WakesUp,
            s => What::Begins(
                s.split(' ')
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim_start_matches('#')
                    .parse()?,
            ),
        })
    }
}

impl fmt::Display for What {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            What::Begins(n) => write!(f, "Guard #{} begins shift", n),
            What::FallsAsleep => write!(f, "falls asleep"),
            What::WakesUp => write!(f, "wakes up"),
        }
    }
}

#[cfg(test)]
#[test]
fn test_event_parse() {
    assert_eq!(
        "[1518-11-01 00:00] Guard #10 begins shift".parse(),
        Ok(Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 0
            },
            what: What::Begins(10)
        })
    );
    assert_eq!(
        "[1518-11-01 00:05] falls asleep".parse(),
        Ok(Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 5
            },
            what: What::FallsAsleep
        })
    );
    assert_eq!(
        "[1518-11-01 00:25] wakes up".parse(),
        Ok(Event {
            time: Time {
                year: 1518,
                month: 11,
                day: 1,
                hour: 0,
                minute: 25
            },
            what: What::WakesUp
        })
    );
}
