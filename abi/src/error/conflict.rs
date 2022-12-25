// parser

use std::{convert::Infallible, str::FromStr};

use chrono::{DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum ReservationConflictInfo {
    Parsed(ReservationConflict),
    Unparsed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationConflict {
    pub old: ReservationWindow,
    pub new: ReservationWindow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationWindow {
    pub rid: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl FromStr for ReservationConflictInfo {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(conflict) = s.parse() {
            Ok(ReservationConflictInfo::Parsed(conflict))
        } else {
            Ok(ReservationConflictInfo::Unparsed(s.to_string()))
        }
    }
}

impl FromStr for ReservationConflict {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"=\((?P<rid>\w*),\s\[(?P<span>[\d\-\s:+",]*)"#).unwrap();
        let mut v = vec![];
        for cap in re.captures_iter(s) {
            let rid = &cap["rid"];
            let span = &cap["span"].replace('"', "");
            let time_v: Vec<&str> = span.splitn(2, ',').collect();
            let start = parse_time_from_str(time_v.first().ok_or("start str empty")?)?;
            let end = parse_time_from_str(time_v.get(1).ok_or("end str empty")?)?;
            v.push(ReservationWindow {
                rid: rid.to_string(),
                start,
                end,
            })
        }
        Ok(ReservationConflict {
            new: v.get(0).unwrap().clone(),
            old: v.get(1).unwrap().clone(),
        })
    }
}

fn parse_time_from_str(s: &str) -> Result<DateTime<Utc>, String> {
    Ok(DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %#z")
        .map_err(|e| e.to_string())?
        .with_timezone(&Utc))
}
