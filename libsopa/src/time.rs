/*
Copyright (C) 2025 Mateusz Mazur (Mazurel) <mateusz.mazur@e.email>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, see
<https://www.gnu.org/licenses/>.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimePoint {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TimeSpan {
    pub from: TimePoint,
    pub to: TimePoint,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OpenedHours {
    time_span_per_day: HashMap<Day, TimeSpan>,
}

impl Default for OpenedHours {
    fn default() -> Self {
        OpenedHours {
            time_span_per_day: HashMap::new(),
        }
    }
}

impl OpenedHours {
    pub fn new() -> Self {
        OpenedHours {
            time_span_per_day: HashMap::new(),
        }
    }

    pub fn get_time_span_per_day(&self) -> &HashMap<Day, TimeSpan> {
        &self.time_span_per_day
    }

    pub fn get_time_span_per_day_mut(&mut self) -> &mut HashMap<Day, TimeSpan> {
        &mut self.time_span_per_day
    }

    pub fn remove_day(&mut self, day: &Day) {
        self.time_span_per_day.remove(&day);
    }

    pub fn set_day_time_span(&mut self, day: Day, time_span: TimeSpan) {
        self.time_span_per_day.insert(day, time_span);
    }

    pub fn get_day_time_span(&self, day: &Day) -> Option<&TimeSpan> {
        self.time_span_per_day.get(day)
    }

    pub fn is_opened_on_day(&self, day: &Day) -> bool {
        self.get_day_time_span(day).is_some()
    }
}

impl TimePoint {
    pub fn to_time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    pub fn from_time_string(time_str: &str) -> Option<Self> {
        let parts: Vec<&str> = time_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(hour), Ok(minute)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                if hour < 24 && minute < 60 {
                    return Some(TimePoint { hour, minute });
                }
            }
        }
        None
    }
}

impl Day {
    pub fn to_display_name(&self) -> std::borrow::Cow<'_, str> {
        match self {
            Day::Monday => t!("monday"),
            Day::Tuesday => t!("tuesday"),
            Day::Wednesday => t!("wednesday"),
            Day::Thursday => t!("thursday"),
            Day::Friday => t!("friday"),
            Day::Saturday => t!("saturday"),
            Day::Sunday => t!("sunday"),
        }
    }

    pub fn get_all_days_in_week() -> Vec<Day> {
        vec![
            Day::Monday,
            Day::Tuesday,
            Day::Wednesday,
            Day::Thursday,
            Day::Friday,
            Day::Saturday,
            Day::Sunday,
        ]
    }
}
