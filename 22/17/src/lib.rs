use jet::Jet;
use rock::get_falling_rocks;
use row::Row;

mod jet;
mod rock;
mod row;

fn _print_room(room: &[Row]) {
    let mut room = room.to_vec();
    room.reverse();
    for row in room {
        println!("|{row}|");
    }
    println!("+-------+");
}

pub fn part1(input: &str) -> usize {
    let mut jets = input.trim().chars().map(Jet::from).cycle();
    let mut room: Vec<Row> = vec![];
    for mut rock in get_falling_rocks().take(2022) {
        let mut offset = room.len() + 3;
        for jet in &mut jets {
            rock = match rock.apply(jet) {
                Some(new_rock) if new_rock.collides_with(&room, offset) => rock,
                Some(new_rock) => new_rock,
                None => rock,
            };
            if offset == 0 {
                break;
            }
            if rock.collides_with(&room, offset - 1) {
                break;
            }
            offset -= 1;
        }
        rock.place_into(&mut room, offset);
    }
    room.len()
}

pub fn part2(_input: &str) -> usize {
    0
}
