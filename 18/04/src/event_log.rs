#[cfg(test)]
mod unit_tests {
    use time::{Date, Month};

    use super::*;

    #[test]
    fn parse_event_shift_start() {
        assert_eq!(
            Event::from("[1518-06-09 00:04] Guard #2393 begins shift"),
            Event {
                date: Date::from_calendar_date(1518, Month::June, 9).unwrap(),
                minute: 4,
                kind: EventKind::ShiftStart { guard_id: 2393 }
            }
        );
    }

    #[test]
    fn parse_event_fall_asleep() {
        assert_eq!(
            Event::from("[1518-11-14 00:12] falls asleep"),
            Event {
                date: Date::from_calendar_date(1518, Month::November, 14).unwrap(),
                minute: 12,
                kind: EventKind::FallAsleep
            }
        );
    }

    #[test]
    fn parse_event_wake_up() {
        assert_eq!(
            Event::from("[1518-02-04 00:58] wakes up"),
            Event {
                date: Date::from_calendar_date(1518, Month::February, 4).unwrap(),
                minute: 58,
                kind: EventKind::WakeUp
            }
        );
    }

    #[test]
    fn parse_before_midnight() {
        assert_eq!(
            Event::from("[1518-05-04 23:56] Guard #523 begins shift"),
            Event {
                date: Date::from_calendar_date(1518, Month::May, 5).unwrap(),
                minute: 0,
                kind: EventKind::ShiftStart { guard_id: 523 }
            }
        );
    }

    #[test]
    #[should_panic]
    fn parse_invalid_date() {
        #![allow(unused_must_use)]
        Event::from("[1518-02-04 12:58] wakes up");
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    ShiftStart { guard_id: u32 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub date: time::Date,
    pub minute: u8,
    pub kind: EventKind,
}

#[derive(Debug)]
pub struct EventLog(pub Vec<Event>);

fn u8_to_month(x: u8) -> time::Month {
    match x {
        1 => time::Month::January,
        2 => time::Month::February,
        3 => time::Month::March,
        4 => time::Month::April,
        5 => time::Month::May,
        6 => time::Month::June,
        7 => time::Month::July,
        8 => time::Month::August,
        9 => time::Month::September,
        10 => time::Month::October,
        11 => time::Month::November,
        12 => time::Month::December,
        _ => panic!("unknown month"),
    }
}

impl From<&str> for Event {
    fn from(s: &str) -> Self {
        // example:
        // [1518-06-09 00:04] Guard #2393 begins shift
        let mut iter = s.split(['[', '-', ' ', ':', ']', '#']);
        iter.next();
        let year = iter.next().unwrap().parse::<i32>().unwrap();
        let month = u8_to_month(iter.next().unwrap().parse::<u8>().unwrap());
        let day = iter.next().unwrap().parse::<u8>().unwrap();
        let hour = iter.next().unwrap().parse::<u8>().unwrap();

        let input_minute = iter.next().unwrap().parse::<u8>().unwrap();
        let input_date = time::Date::from_calendar_date(year, month, day).unwrap();

        let (date, minute) = match () {
            _ if hour == 0 => (input_date, input_minute),
            _ if hour == 23 => (input_date.next_day().unwrap(), 0),
            _ => panic!("invalid hour: {hour}"),
        };

        iter.next(); // "] " in "04] Guard" is split twice

        let kind = match iter.next().unwrap() {
            "Guard" => EventKind::ShiftStart {
                guard_id: iter.nth(1).unwrap().parse::<u32>().unwrap(),
            },
            "falls" => EventKind::FallAsleep,
            "wakes" => EventKind::WakeUp,
            _ => panic!("unknown event kind"),
        };

        Event { date, minute, kind }
    }
}

impl From<&str> for EventLog {
    fn from(input: &str) -> Self {
        EventLog(input.lines().map(Event::from).collect())
    }
}
