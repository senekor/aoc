use core::str;
use std::{cmp, str::FromStr};

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn next_value(split: &mut str::Split<&str>) -> i32 {
    split
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap()
}

impl FromStr for Ingredient {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.next().unwrap();
        let rest = parts.next().unwrap();
        let properties = &mut rest.split(", ");
        Ok(Ingredient {
            name: name.to_string(),
            capacity: next_value(properties),
            durability: next_value(properties),
            flavor: next_value(properties),
            texture: next_value(properties),
            calories: next_value(properties),
        })
    }
}

fn score(v: &[(i32, Ingredient)], dbg: bool) -> i32 {
    let multipl_ingredients = v
        .iter()
        .map(|(count, ingredient)| Ingredient {
            name: ingredient.name.clone(),
            capacity: count * ingredient.capacity,
            durability: count * ingredient.durability,
            flavor: count * ingredient.flavor,
            texture: count * ingredient.texture,
            calories: count * ingredient.calories,
        })
        .collect::<Vec<Ingredient>>();
    let reduced_ingredient = multipl_ingredients.iter().fold(
        Ingredient {
            name: "".to_string(),
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        },
        |acc, ingredient| Ingredient {
            name: ingredient.name.clone(),
            capacity: acc.capacity + ingredient.capacity,
            durability: acc.durability + ingredient.durability,
            flavor: acc.flavor + ingredient.flavor,
            texture: acc.texture + ingredient.texture,
            calories: acc.calories + ingredient.calories,
        },
    );
    let non_neg_ingredient = Ingredient {
        name: reduced_ingredient.name.clone(),
        capacity: cmp::max(reduced_ingredient.capacity, 0),
        durability: cmp::max(reduced_ingredient.durability, 0),
        flavor: cmp::max(reduced_ingredient.flavor, 0),
        texture: cmp::max(reduced_ingredient.texture, 0),
        calories: cmp::max(reduced_ingredient.calories, 0),
    };
    let res = non_neg_ingredient.capacity
        * non_neg_ingredient.durability
        * non_neg_ingredient.flavor
        * non_neg_ingredient.texture;
    if dbg {
        dbg!(multipl_ingredients);
        dbg!(reduced_ingredient);
        dbg!(non_neg_ingredient);
        dbg!(res);
    }
    res
}

struct MyIterator {
    num_ingredients: i32,
    teaspoons: i32,
    counter: i32,
    rec_iter: Option<Box<MyIterator>>,
}

impl MyIterator {
    fn new(num_ingredients: i32, teaspoons: i32) -> MyIterator {
        assert!(num_ingredients > 0);
        let rec_iter = match num_ingredients > 1 {
            true => Some(Box::new(MyIterator::new(num_ingredients - 1, teaspoons))),
            false => None,
        };
        let counter = match num_ingredients > 1 {
            true => 0,
            false => teaspoons,
        };
        MyIterator {
            num_ingredients,
            teaspoons,
            counter,
            rec_iter,
        }
    }
}

impl Iterator for MyIterator {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.rec_iter.is_none() {
            if self.counter > self.teaspoons {
                return None;
            };
            let res = self.counter;
            self.counter += 1;
            return Some(vec![res]);
        }

        let prev_res = self.rec_iter.as_deref_mut().unwrap().next();
        let mut smol = match prev_res {
            Some(v) => v,
            None => {
                self.counter += 1;
                if self.counter > self.teaspoons {
                    return None;
                };
                self.rec_iter = Some(Box::new(MyIterator::new(
                    self.num_ingredients - 1,
                    self.teaspoons - self.counter,
                )));
                self.rec_iter.as_deref_mut().unwrap().next().unwrap()
            }
        };
        smol.push(self.counter);
        Some(smol)
    }
}

#[test]
fn test_score() {
    assert_eq!(
        62842880,
        score(
            &[
                (
                    44,
                    Ingredient {
                        name: String::from("Butterscotch"),
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8,
                    },
                ),
                (
                    56,
                    Ingredient {
                        name: String::from("Cinnamon"),
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3,
                    },
                ),
            ],
            false
        )
    )
}

fn calc_calories(v: &[(i32, Ingredient)]) -> i32 {
    v.iter().map(|(count, ing)| count * ing.calories).sum()
}

fn calc_max_cookie_score(input: &str, calories: Option<i32>) {
    let ingredients: Vec<Ingredient> = input.lines().map(str::parse).map(Result::unwrap).collect();

    let mut recipe = Vec::with_capacity(ingredients.len());
    for ing in ingredients {
        recipe.push((0, ing))
    }

    let my_iter = MyIterator::new(recipe.len() as i32, 100);

    let mut max = 0;
    for perm in my_iter {
        for i in 0..recipe.len() {
            recipe[i].0 = perm[i];
        }
        let calorie_constraint_met = if let Some(c) = calories {
            c == calc_calories(&recipe)
        } else {
            true
        };
        if calorie_constraint_met {
            let cur_score = score(&recipe, false);
            if cur_score > max {
                max = cur_score
            }
        }
    }
    println!("optimal cookie score: {}", max)
}

fn part1(input: &str) {
    calc_max_cookie_score(input, None);
}

fn part2(input: &str) {
    calc_max_cookie_score(input, Some(500));
}

fn main() {
    let input = include_str!("../input/input.txt");

    part1(input);

    part2(input);
}
