use num::BigUint;

macro_rules! periodic_table {
    ($($idx:literal $element:ident $repr:literal $($next:literal),+);+) => {
        mod repr {
            $(
                pub const $element: &str = $repr;
            )+
            // custom transuranic elements. last, variable digit in string omitted
            pub const NP: &str = "1311222113321132211221121332211";
            pub const PU: &str = "31221132221222112112322211";
        }
        mod next {
            $(
                pub const $element: &[usize] = &[$($next),+];
            )+
            // custom transuranic elements.
            pub const NP: &[usize] = &[71, 90, 0, 19, 93];
            pub const PU: &[usize] = &[92];
        }
        pub fn str_to_nr(elem: &str) -> usize {
            match elem {
                $(
                    repr::$element => $idx,
                )+
                // custom cases for transuranic elements
                _ => match elem.get(0..elem.len() - 1).unwrap() {
                    repr::NP => 92,
                    repr::PU => 93,
                    _ => panic!("element not recognized: {elem}"),
                },
            }
        }
        pub fn nr_to_len(elem_nr: usize) -> usize {
            match elem_nr {
                $(
                    $idx => repr::$element.len(),
                )+
                // custom cases for transuranic elements
                92 => repr::NP.len() + 1,
                93 => repr::PU.len() + 1,
                _ => panic!("unknown element number: {elem_nr}"),
            }
        }
        pub fn nr_to_next(elem_nr: usize) -> &'static [usize] {
            match elem_nr {
                $(
                    $idx => next::$element,
                )+
                // custom cases for transuranic elements
                92 => next::NP,
                93 => next::PU,
                _ => panic!("unknown element number: {elem_nr}"),
            }
        }
    };
}

periodic_table! {
91 U "3" 90;
90 PA "13" 89;
89 TH "1113" 88;
88 AC "3113" 87;
87 RA "132113" 86;
86 FR "1113122113" 85;
85 RN "311311222113" 66, 84;
84 AT "1322113" 83;
83 PO "1113222113" 82;
82 BI "3113322113" 60, 81;
81 PB "123222113" 80;
80 TL "111213322113" 79;
79 HG "31121123222113" 78;
78 AU "132112211213322113" 77;
77 PT "111312212221121123222113" 76;
76 IR "3113112211322112211213322113" 75;
75 OS "1321132122211322212221121123222113" 74;
74 RE "111312211312113221133211322112211213322113" 31, 19, 73;
73 W "312211322212221121123222113" 72;
72 TA "13112221133211322112211213322113" 71, 90, 0, 19, 73;
71 HF "11132" 70;
70 LU "311312" 69;
69 YB "1321131112" 68;
68 TM "11131221133112" 67, 19, 26;
67 ER "311311222" 66, 60;
66 HO "1321132" 65;
65 DY "111312211312" 64;
64 TB "3113112221131112" 66, 63;
63 GD "13221133112" 62, 19, 26;
62 EU "1113222" 61;
61 SM "311332" 60, 19, 29;
60 PM "132" 59;
59 ND "111312" 58;
58 PR "31131112" 57;
57 CE "1321133112" 56, 0, 19, 26;
56 LA "11131" 55;
55 BA "311311" 54;
54 CS "13211321" 53;
53 XE "11131221131211" 52;
52 I "311311222113111221" 66, 51;
51 TE "1322113312211" 62, 19, 50;
50 SB "3112221" 60, 49;
49 SN "13211" 48;
48 IN "11131221" 47;
47 CD "3113112211" 46;
46 AG "132113212221" 45;
45 PD "111312211312113211" 44;
44 RH "311311222113111221131221" 66, 43;
43 RU "132211331222113112211" 62, 19, 42;
42 TC "311322113212221" 41;
41 MO "13211322211312113211" 40;
40 NB "1113122113322113111221131221" 67, 39;
39 ZR "12322211331222113112211" 38, 0, 19, 42;
38 Y "1112133" 37, 91;
37 SR "3112112" 36;
36 RB "1321122112" 35;
35 KR "11131221222112" 34;
34 BR "3113112211322112" 33;
33 SE "13211321222113222112" 32;
32 AS "11131221131211322113322112" 31, 10;
31 GE "31131122211311122113222" 66, 30;
30 GA "13221133122211332" 62, 19, 88, 0, 19, 29;
29 ZN "312" 28;
28 CU "131112" 27;
27 NI "11133112" 29, 26;
26 CO "32112" 25;
25 FE "13122112" 24;
24 MN "111311222112" 23, 13;
23 CR "31132" 22;
22 V "13211312" 21;
21 TI "11131221131112" 20;
20 SC "3113112221133112" 66, 90, 0, 19, 26;
19 CA "12" 18;
18 K "1112" 17;
17 AR "3112" 16;
16 CL "132112" 15;
15 S "1113122112" 14;
14 P "311311222112" 66, 13;
13 SI "1322112" 12;
12 AL "1113222112" 11;
11 MG "3113322112" 60, 10;
10 NA "123222112" 9;
9 NE "111213322112" 8;
8 F "31121123222112" 7;
7 O "132112211213322112" 6;
6 N "111312212221121123222112" 5;
5 C "3113112211322112211213322112" 4;
4 B "1321132122211322212221121123222112" 3;
3 BE "111312211312113221133211322112211213322112" 31, 19, 2;
2 LI "312211322212221121123222112" 1;
1 HE "13112221133211322112211213322112" 71, 90, 0, 19, 2;
0 H "22" 0
}

fn look_and_say_once(input: String) -> String {
    let mut chars = input.chars();
    let mut digit = chars.next().unwrap();
    let mut count = 1;
    let mut res = "".to_string();
    for c in chars {
        if c == digit {
            count += 1;
        } else {
            res = format!("{}{}{}", res, count, digit);
            digit = c;
            count = 1;
        }
    }
    format!("{res}{count}{digit}")
}

mod rules {
    const B1: u8 = b'1';
    const B2: u8 = b'2';
    const B3: u8 = b'3';
    // single rules assume that b[i] == b2
    fn single_rule_1(b: &[u8], l: usize, i: usize) -> bool {
        i + 2 < l && b[i + 1] == B1 && b[i + 1] != b[i + 2] && (i + 3 >= l || b[i + 2] != b[i + 3])
    }
    fn single_rule_2(b: &[u8], l: usize, i: usize) -> bool {
        i + 3 < l && b[i + 1] == B1 && b[i + 2] == B1 && b[i + 3] == B1
    }
    fn single_rule_3(b: &[u8], l: usize, i: usize) -> bool {
        i + 1 < l
            && b[i + 1] == B3
            && (i + 2 >= l
                || (b[i + 2] != B3 && (i + 4 >= l || b[i + 3] != b[i + 2] || b[i + 4] != b[i + 2])))
    }
    pub fn single_rule(b: &[u8], l: usize, i: usize) -> bool {
        single_rule_1(b, l, i) || single_rule_2(b, l, i) || single_rule_3(b, l, i)
    }
    // double rule assumes that b[i] != b2
    pub fn double_rule(b: &[u8], l: usize, i: usize) -> bool {
        i + 3 < l && b[i + 1] == B2 && b[i + 2] == B2 && single_rule(b, l, i + 2)
    }
    pub fn end_rule(b: &[u8], l: usize) -> bool {
        b[l - 3] != B2 && b[l - 2] == B2 && b[l - 1] == B2
    }
}

fn parse_elems(only_elements: &str) -> Vec<BigUint> {
    // find all splits according to "The Splitting Theorem", see
    // https://web.archive.org/web/20061224154744/http://www.uam.es/personal_pdi/ciencias/omartin/Biochem.PDF
    let b = only_elements.as_bytes();
    let l = b.len();
    let mut splits: Vec<usize> = Vec::new();
    for i in 0..b.len() {
        if b[i] == b'2' && rules::single_rule(b, l, i)
            || b[i] != b'2' && rules::double_rule(b, l, i)
        {
            splits.push(i + 1);
        }
    }
    if rules::end_rule(b, l) {
        splits.push(l - 2);
    }

    // records how many times an element was found among the splits
    let mut arr = Vec::new();
    arr.resize(94, BigUint::new(Vec::new()));

    // first split
    arr[str_to_nr(only_elements.get(0..splits[0]).unwrap())] += 1_u32;

    for i in 0..(splits.len() - 1) {
        let s = only_elements.get(splits[i]..splits[i + 1]).unwrap();
        arr[str_to_nr(s)] += 1_u32;
    }
    // last split
    arr[str_to_nr(only_elements.get(splits[splits.len() - 1]..).unwrap())] += 1_u32;

    arr
}

fn smart_look_and_say(elems: Vec<BigUint>) -> Vec<BigUint> {
    let mut new_elems = Vec::new();
    new_elems.resize(94, BigUint::new(Vec::new()));
    for (i, elem) in elems.iter().enumerate() {
        for next_elem in nr_to_next(i) {
            new_elems[*next_elem] += elem;
        }
    }
    new_elems
}

fn repeat_smart_las(elems: Vec<BigUint>, num_reps: i32) -> Vec<BigUint> {
    let mut new_elems = elems;
    for _ in 0..num_reps {
        new_elems = smart_look_and_say(new_elems);
    }
    new_elems
}

fn calc_len(elems: &[BigUint]) -> BigUint {
    let mut count = BigUint::new(Vec::new());
    for (i, elem) in elems.iter().enumerate() {
        count += elem * nr_to_len(i);
    }
    count
}

pub fn look_and_say(input: &str, n: i32) -> BigUint {
    let mut only_elements = input.to_string();
    for _ in 0..24 {
        only_elements = look_and_say_once(only_elements)
    }
    let elems = repeat_smart_las(parse_elems(&only_elements), n - 24);
    calc_len(&elems)
}

pub fn part1(input: &str) -> BigUint {
    look_and_say(input, 40)
}

pub fn part2(input: &str) -> BigUint {
    look_and_say(input, 50)
}
