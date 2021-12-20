use itertools::*;

#[macro_export]
macro_rules! parse {
    ( $line:expr, $( $t:ty, $sep:expr ),* ; $lt:ty ) => {{
        let mut rest = $line.to_string();
        (
            $({
                let mut iter = rest.split($sep);
                let elem = iter.next().unwrap().parse::<$t>().unwrap();
                rest = iter.join($sep);
                elem
            },)*
            rest.parse::<$lt>().unwrap(),
        )
    }};
}

// ---------- adjust these to customize parsing ---------- //
type Line = (i32, i32);
fn parse(line: &str) -> Line {
    parse!(line, i32, "," ; i32)
}
// ------------------------------------------------------- //

fn part1(input: Vec<Line>) {
    println!("{:?}", "bruh")
}

fn part2(input: Vec<Line>) {
    println!("{:?}", "bruh")
}

fn main() {
    let input = include_str!("../input/input.txt");
    let input = input.lines().map(parse).collect_vec();

    part1(input.clone());

    part2(input.clone());
}
