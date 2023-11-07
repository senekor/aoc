use std::collections::HashMap;

mod parse;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dependency: char,
    target: char,
}

#[derive(Debug, Clone, Default)]
struct DependencyTree {
    steps: HashMap<char, Vec<char>>,
}

impl From<Vec<Instruction>> for DependencyTree {
    fn from(instructions: Vec<Instruction>) -> Self {
        let mut deps = Self::default();
        for Instruction { dependency, target } in instructions {
            // ensure steps without dependencies appear as well
            deps.steps.entry(dependency).or_default();

            deps.steps.entry(target).or_default().push(dependency);
        }
        deps
    }
}

impl DependencyTree {
    fn remove_next_ready_step(&mut self) -> Option<char> {
        let res = self
            .steps
            .iter()
            .filter_map(|(&k, v)| v.is_empty().then_some(k))
            .min();
        if let Some(res) = res {
            self.steps.remove(&res);
        }
        res
    }

    fn mark_completed(&mut self, completed: char) {
        self.steps.remove(&completed);
        for (_, v) in self.steps.iter_mut() {
            v.retain(|&dep| dep != completed);
        }
    }

    fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

impl Iterator for DependencyTree {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps.is_empty() {
            return None;
        }
        let res = self.remove_next_ready_step().unwrap();
        self.mark_completed(res);
        Some(res)
    }
}

pub fn part1(input: &str) -> String {
    let (_, instructions) = parse::instructions(input).unwrap();
    let dep_tree = DependencyTree::from(instructions);
    dep_tree.collect()
}

pub fn part2_impl(input: &str, num_workers: usize, base_time: usize) -> usize {
    let (_, instructions) = parse::instructions(input).unwrap();
    let mut dep_tree = DependencyTree::from(instructions);

    let mut t = 0;
    let mut steps_in_progress = Vec::with_capacity(num_workers);
    while !dep_tree.is_empty() || !steps_in_progress.is_empty() {
        // start as many new steps as possible
        while steps_in_progress.len() < num_workers {
            if let Some(new_step) = dep_tree.remove_next_ready_step() {
                let execution_time = base_time + (new_step as usize - 'A' as usize + 1);
                steps_in_progress.push((new_step, execution_time));
            } else {
                break;
            }
        }

        for &mut (step, ref mut time) in steps_in_progress.iter_mut() {
            *time -= 1;
            if *time == 0 {
                dep_tree.mark_completed(step);
            }
        }
        steps_in_progress.retain(|&(_, time)| time != 0);

        t += 1;
    }
    t
}

pub fn part2(input: &str) -> usize {
    part2_impl(input, 6, 60)
}
