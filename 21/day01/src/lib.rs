fn parse_i32(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn add_three(a: &str, b: &str, c: &str) -> i32 {
    parse_i32(a) + parse_i32(b) + parse_i32(c)
}

pub fn lib_main(input: &str) -> (i32, i32) {
    let mut input_split_on_lines = input.split('\n');
    let mut first_measurement = input_split_on_lines.next().unwrap();
    let mut second_measurement = input_split_on_lines.next().unwrap();
    let mut count = 0;
    loop {
        if parse_i32(first_measurement) < parse_i32(second_measurement) {
            count += 1;
        }
        first_measurement = second_measurement;
        let next_measurement = input_split_on_lines.next();
        match next_measurement {
            None => break,
            Some(some_next_measurement) => second_measurement = some_next_measurement,
        }
    }
    input_split_on_lines = input.split('\n');
    first_measurement = input_split_on_lines.next().unwrap();
    second_measurement = input_split_on_lines.next().unwrap();
    let mut third_measurement = input_split_on_lines.next().unwrap();
    let mut forth_measurement = input_split_on_lines.next().unwrap();
    let mut count_part2 = 0;
    loop {
        if add_three(first_measurement, second_measurement, third_measurement)
            < add_three(second_measurement, third_measurement, forth_measurement)
        {
            count_part2 += 1;
        }
        first_measurement = second_measurement;
        second_measurement = third_measurement;
        third_measurement = forth_measurement;
        let n = input_split_on_lines.next();
        match n {
            None => break,
            Some(some_next_measurement) => forth_measurement = some_next_measurement,
        }
    }
    (count, count_part2)
}
