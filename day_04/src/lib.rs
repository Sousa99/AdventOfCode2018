
use std::collections::HashMap;

use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

// ======================================================= CONSTANTS DEFINITIONS =======================================================

const TIMESTAMP_FORMAT : &str = "%Y-%m-%d %H:%M";

// ======================================================== STRUCTS DEFINITIONS ========================================================

#[derive(Debug)]
pub enum LogType {
    StartShift,
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
pub struct Log {
    guard: Option<i64>,
    timestamp: NaiveDateTime,
    log_type: LogType,
}

pub struct Scheduler {
    logs: Vec<Log>
}

// ======================================================== STRUCTS IMPLEMENTATIONS ========================================================

impl Log {

    pub fn new(log_string: String) -> Log {

        let timestamp_regex : Regex = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\]").unwrap();
        let start_shift_regex : Regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] Guard #(\d+) begins shift").unwrap();
        let fall_asleep_regex : Regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] falls asleep").unwrap();
        let wake_up_regex : Regex = Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] wakes up").unwrap();

        let timestamp_string = timestamp_regex.captures(&log_string)
            .unwrap().get(1)
            .unwrap().as_str();
        let timestamp = NaiveDateTime::parse_from_str(timestamp_string, TIMESTAMP_FORMAT)
            .unwrap();

        let mut guard : Option<i64> = None;
        let log_type : LogType;

        if start_shift_regex.is_match(&log_string) {
            let guard_string = start_shift_regex.captures(&log_string)
                .unwrap().get(1)
                .unwrap().as_str();
            guard = Some(guard_string.parse::<i64>().unwrap());
            log_type = LogType::StartShift;

        } else if fall_asleep_regex.is_match(&log_string) { log_type = LogType::FallAsleep; } 
        else if wake_up_regex.is_match(&log_string) { log_type = LogType::WakeUp; }
        else { panic!("🚨 Log could not be parsed into one of the specified types!")}


        Log {
            guard: guard,
            timestamp: timestamp,
            log_type: log_type,
        }
    }
}

impl Scheduler {
    pub fn new(mut logs: Vec<Log>) -> Scheduler {
        logs.sort_by_key(|log| log.timestamp);
        Scheduler { logs: logs }
    }

    pub fn process_log_guards(&mut self) {
        let mut current_guard : Option<i64> = None;
        for log in self.logs.iter_mut() {
            match log.log_type {
                LogType::StartShift => current_guard = log.guard,
                LogType::FallAsleep => log.guard = current_guard,
                LogType::WakeUp => log.guard = current_guard,
            }
        }
    }

    pub fn get_most_sleepy_guard(&self) -> Option<i64> {
        let mut map_sleep_durations : HashMap<i64, i64> = HashMap::new();
        let mut current_guard_start : Option<(i64, Option<NaiveDateTime>)> = None;

        for log in self.logs.iter() {
            if log.guard.is_none() { panic!("🚨 Please make sure that logs have been processed!"); }
            match log.log_type {
                LogType::StartShift => current_guard_start = None,
                LogType::FallAsleep if current_guard_start.is_none() => current_guard_start = Some((log.guard.unwrap(), Some(log.timestamp))),
                LogType::WakeUp if current_guard_start.is_some() && current_guard_start.unwrap().1.is_some() => {
                    let duration = log.timestamp - current_guard_start.unwrap().1.unwrap();
                    let duration_minutes = duration.num_minutes();
                    
                    if !map_sleep_durations.contains_key(&log.guard.unwrap()) { map_sleep_durations.insert(log.guard.unwrap(), 0); }
                    let current_count = map_sleep_durations.get_mut(&log.guard.unwrap()).unwrap();
                    *current_count = *current_count + duration_minutes;
                    current_guard_start = None;
                }
                _ => (),
            }
        }

        return map_sleep_durations.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(guard, _)| guard);
    }

    pub fn get_most_probable_sleep_time(&self, guard_id: i64) -> Option<i64> {
        let mut map_scheduled_minutes : HashMap<i64, i64> = HashMap::new();
        let mut current_guard_start : Option<(i64, Option<NaiveDateTime>)> = None;

        for log in self.logs.iter() {
            if log.guard.is_none() { panic!("🚨 Please make sure that logs have been processed!"); }
            if log.guard.unwrap() != guard_id { continue; }
            match log.log_type {
                LogType::StartShift => current_guard_start = None,
                LogType::FallAsleep if current_guard_start.is_none() => current_guard_start = Some((log.guard.unwrap(), Some(log.timestamp))),
                LogType::WakeUp if current_guard_start.is_some() && current_guard_start.unwrap().1.is_some() => {
                    let start_time = current_guard_start.unwrap().1.unwrap();
                    let end_time = log.timestamp;
                    for minute in start_time.minute()..end_time.minute() {

                        if !map_scheduled_minutes.contains_key(&(minute as i64)) { map_scheduled_minutes.insert(minute as i64, 0); }
                        let current_count = map_scheduled_minutes.get_mut(&(minute as i64)).unwrap();
                        *current_count = *current_count + 1;

                        current_guard_start = None;
                    }
                }
                _ => (),
            }
        }

        return map_scheduled_minutes.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(minute, _)| minute);
    }

    pub fn get_most_probable_sleep_time_all(&self) -> Option<(i64, i64)> {
        let mut map_scheduled_minutes : HashMap<i64, HashMap<i64, i64>> = HashMap::new();
        let mut current_guard_start : Option<(i64, Option<NaiveDateTime>)> = None;

        for log in self.logs.iter() {
            if log.guard.is_none() { panic!("🚨 Please make sure that logs have been processed!"); }
            if !map_scheduled_minutes.contains_key(&log.guard.unwrap()) { map_scheduled_minutes.insert(log.guard.unwrap(), HashMap::new()); }
            match log.log_type {
                LogType::StartShift => current_guard_start = None,
                LogType::FallAsleep if current_guard_start.is_none() => current_guard_start = Some((log.guard.unwrap(), Some(log.timestamp))),
                LogType::WakeUp if current_guard_start.is_some() && current_guard_start.unwrap().1.is_some() => {
                    let start_time = current_guard_start.unwrap().1.unwrap();
                    let end_time = log.timestamp;
                    for minute in start_time.minute()..end_time.minute() {

                        let map_schedule_minutes_by_guard = map_scheduled_minutes.get_mut(&log.guard.unwrap()).unwrap();
                        if !map_schedule_minutes_by_guard.contains_key(&(minute as i64)) { map_schedule_minutes_by_guard.insert(minute as i64, 0); }
                        let current_count = map_schedule_minutes_by_guard.get_mut(&(minute as i64)).unwrap();
                        *current_count = *current_count + 1;

                        current_guard_start = None;
                    }
                }
                _ => (),
            }
        }

        return map_scheduled_minutes.into_iter()
            .filter(|(_, map_by_guard)| map_by_guard.len() != 0)
            .map(|(guard, map_by_guard)| {
                let max_by_guard = map_by_guard.into_iter()
                    .max_by_key(|(_, count)| *count).unwrap();
                return (guard, max_by_guard.0, max_by_guard.1);
            })
            .max_by_key(|(_, _, count)| *count)
            .map(|(guard, minute, _)| (guard, minute));
    }
}