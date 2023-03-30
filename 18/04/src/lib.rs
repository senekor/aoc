mod date_log;
mod event_log;
mod guard_log;

use std::cmp::Reverse;

use guard_log::GuardEntry;

impl GuardEntry {
    fn count_sleep_minutes(&self) -> usize {
        self.0
            .iter()
            .map(|shift| {
                shift
                    .sleep_sessions
                    .iter()
                    .map(|sleep_session| {
                        (sleep_session.wake_up - sleep_session.fall_asleep) as usize
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn get_sleepiest_minute(&self) -> (u32, usize) {
        let mut sleep_counts = [0; 60];
        for guard_shift in self.0.iter() {
            for sleep_session in &guard_shift.sleep_sessions {
                for i in sleep_session.fall_asleep..sleep_session.wake_up {
                    sleep_counts[i as usize] += 1;
                }
            }
        }
        sleep_counts
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as u32, c))
            .max_by_key(|(_, sleep_count)| *sleep_count)
            .unwrap()
    }
}

pub fn part1(input: &str) -> u32 {
    let guard_log = guard_log::GuardLog::from(input);

    let mut guard_vec: Vec<_> = guard_log.0.into_iter().collect();
    guard_vec.sort_by_cached_key(|(_, guard_entry)| Reverse(guard_entry.count_sleep_minutes()));

    guard_vec[0].0 * guard_vec[0].1.get_sleepiest_minute().0
}

pub fn part2(input: &str) -> u32 {
    let guard_log = guard_log::GuardLog::from(input);

    let mut guards: Vec<_> = guard_log.0.into_iter().collect();

    guards.sort_by_cached_key(|(_, guard_entry)| Reverse(guard_entry.get_sleepiest_minute().1));

    guards[0].0 * guards[0].1.get_sleepiest_minute().0
}
