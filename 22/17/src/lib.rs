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

fn f<const NUM_ROCKS: usize>(input: &str) -> usize {
    let mut jets = input.trim().chars().map(Jet::from).cycle();
    let mut room: Vec<Row> = vec![];
    for mut rock in get_falling_rocks().take(NUM_ROCKS) {
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

#[test]
fn vec_drain_shifts() {
    let mut v = vec![1, 2, 3, 4];
    v.drain(..2);
    assert_eq!(v, [3, 4])
}

pub fn part1(input: &str) -> usize {
    f::<2022>(input)
}

pub fn part2(input: &str) -> usize {
    f::<1_000_000_000_000>(input)
}
