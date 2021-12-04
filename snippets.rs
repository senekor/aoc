// todo

#[derive(Clone, Debug)]
enum MyEnum {
    One,
    Two,
    Three,
}
use MyEnum::*;

fn str_to_enum(s: &str) -> MyEnum {
    match s {
        "one" => One,
        "two" => Two,
        "three" => Three,
        x => panic!("unknown direction {}", x),
    }
}


#[derive(Clone, Debug)]
struct ParsedLine {
    enum_val: MyEnum,
    str: String,
    num: i32,
}
// or, if the line is just one primitive value:
// type ParsedLine = i32;

fn parse_line(line: &str) -> ParsedLine {
    let mut iter = line.split(" ");
    ParsedLine {
        enum_val: str_to_enum(iter.next().unwrap()),
        str: iter.next().unwrap().to_string(),
        num: iter.next().unwrap().parse().unwrap(),
    }
}
