use std::collections::HashMap;

use time::Date;

use crate::date_log::{DateLog, SleepSession};

pub struct GuardShift {
    pub _date: Date,
    pub sleep_sessions: Vec<SleepSession>,
}

#[derive(Default)]
pub struct GuardEntry(pub Vec<GuardShift>);

pub struct GuardLog(pub HashMap<u32, GuardEntry>);

impl From<DateLog> for GuardLog {
    fn from(date_log: DateLog) -> Self {
        let mut hm: HashMap<_, GuardEntry> = HashMap::new();

        for (date, date_entry) in date_log.0 {
            let guard_id = date_entry.guard_id;
            let sleep_sessions = date_entry.sleep_sessions;
            hm.entry(guard_id).or_default().0.push(GuardShift {
                _date: date,
                sleep_sessions,
            });
        }

        GuardLog(hm)
    }
}

impl From<&str> for GuardLog {
    fn from(s: &str) -> Self {
        GuardLog::from(DateLog::from(s))
    }
}
