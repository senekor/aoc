use json::{
    self,
    object::Object,
    JsonValue::{self, *},
};

trait Summable {
    fn sum_all(&self) -> i64;
}

impl Summable for Object {
    fn sum_all(&self) -> i64 {
        self.iter()
            .into_iter()
            .fold(0, |acc, (_, jv)| acc + sum(jv))
    }
}

impl Summable for Vec<JsonValue> {
    fn sum_all(&self) -> i64 {
        self.iter().fold(0, |acc, jv| acc + sum(jv))
    }
}

fn sum(obj: &JsonValue) -> i64 {
    match obj {
        Short(_s) => 0,
        String(_s) => 0,
        Number(n) => n.as_fixed_point_i64(0).unwrap(),
        Object(o) => o.sum_all(),
        Array(v) => v.sum_all(),
        x => {
            dbg!(x);
            panic!("unexpected json val")
        }
        // Null => 0,
        // Boolean(b) => 0,
    }
}

fn part1(input: &str) {
    let obj = json::parse(input).unwrap();

    println!("sum: {}", sum(&obj))
}

trait Summable2 {
    fn sum_all2(&self) -> i64;
}

impl Summable2 for Object {
    fn sum_all2(&self) -> i64 {
        self.iter()
            .into_iter()
            .fold(0, |acc, (_, jv)| acc + sum2(jv))
    }
}

impl Summable2 for Vec<JsonValue> {
    fn sum_all2(&self) -> i64 {
        self.iter().fold(0, |acc, jv| acc + sum2(jv))
    }
}

fn has_red(object: &Object) -> bool {
    object.iter().into_iter().any(|(_, jv)| match jv {
        Short(s) => s.as_str() == "red",
        String(s) => s.as_str() == "red",
        _ => false,
    })
}

fn sum2(obj: &JsonValue) -> i64 {
    match obj {
        Short(_s) => 0,
        String(_s) => 0,
        Number(n) => n.as_fixed_point_i64(0).unwrap(),
        Object(o) => if has_red(o) { 0 } else { o.sum_all2() },
        Array(v) => v.sum_all2(),
        x => {
            dbg!(x);
            panic!("unexpected json val")
        }
        // Null => 0,
        // Boolean(b) => 0,
    }
}

fn part2(input: &str) {
    let obj = json::parse(input).unwrap();

    println!("sum without red: {}", sum2(&obj))
}

fn main() {
    let input = include_str!("../input/input.txt");

    // let input = "[1,{\"c\":\"red\",\"b\":2},3]";

    part1(input);

    part2(input)
}
