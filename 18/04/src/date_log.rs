use std::collections::HashMap;
use time::Date;

use crate::event_log::{Event, EventKind, EventLog};

pub struct SleepSession {
    pub fall_asleep: u8,
    pub wake_up: u8,
}

pub struct DateEntry {
    pub guard_id: u32,
    pub sleep_sessions: Vec<SleepSession>,
}

pub struct DateLog(pub HashMap<Date, DateEntry>);

impl From<EventLog> for DateLog {
    fn from(mut event_log: EventLog) -> Self {
        event_log.0.sort_unstable();

        let mut hm = HashMap::new();

        for Event { date, minute, kind } in event_log.0 {
            match kind {
                EventKind::ShiftStart { guard_id } => {
                    hm.insert(
                        date,
                        DateEntry {
                            guard_id,
                            sleep_sessions: vec![],
                        },
                    );
                }
                EventKind::FallAsleep => hm
                    .entry(date)
                    .or_insert_with(|| panic!("fall asleep before shift start!"))
                    .sleep_sessions
                    .push(SleepSession {
                        fall_asleep: minute,
                        wake_up: 0,
                    }),
                EventKind::WakeUp => {
                    hm.entry(date)
                        .or_insert_with(|| panic!("wake up before before shift start!"))
                        .sleep_sessions
                        .last_mut()
                        .unwrap()
                        .wake_up = minute
                }
            };
        }

        DateLog(hm)
    }
}

impl From<&str> for DateLog {
    fn from(s: &str) -> Self {
        DateLog::from(EventLog::from(s))
    }
}
