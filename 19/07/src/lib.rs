use aoc_19_05::{InstrOutput, Program};
use itertools::Itertools as _;

fn execute_until_first_output(prog: &mut Program) -> Option<i32> {
    loop {
        match prog.execute_one_instr() {
            InstrOutput::Nothing => {}
            InstrOutput::Something(val) => return Some(val),
            InstrOutput::Stop => return None,
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut prog: Program = input.parse().unwrap();
    (0..5)
        .permutations(5)
        .map(|phase_setting_sequence| {
            phase_setting_sequence
                .into_iter()
                .fold(0, |input, phase_setting| {
                    prog.reset();
                    prog.add_input(phase_setting);
                    prog.add_input(input);
                    prog.execute_one_instr();
                    execute_until_first_output(&mut prog).unwrap()
                })
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut progs = {
        let prog: Program = input.parse().unwrap();
        [prog.clone(), prog.clone(), prog.clone(), prog.clone(), prog]
    };

    (5..10)
        .permutations(5)
        .map(|phase_setting_sequence| {
            for (prog, phase_setting) in progs.iter_mut().zip(phase_setting_sequence) {
                prog.reset();
                prog.add_input(phase_setting);
            }
            progs[0].add_input(0);

            let mut last_output = 0;

            for i in (0..5).cycle() {
                match execute_until_first_output(&mut progs[i]) {
                    Some(val) => {
                        progs[(i + 1) % 5].add_input(val);
                        if i == 4 {
                            last_output = val;
                        }
                    }
                    _ => break,
                }
            }

            last_output
        })
        .max()
        .unwrap()
}
