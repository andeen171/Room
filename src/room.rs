use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct Person {
    value: i32,
    position: Position,
}

fn random_move(matrix: &Arc<Mutex<Vec<Vec<i32>>>>, person: Person, door: &Position) -> Person {
    let mut rng = rand::thread_rng();
    let mut matrix = matrix.lock().unwrap();
    let mut target = Position {
        x: person.position.x,
        y: person.position.y,
    };

    match rng.gen_range(0..8) {
        0 => {
            if (target.x != 0) {
                target.x -= 1;
            }
        }
        1 => {
            if (target.y != 0) {
                target.y -= 1;
            }
        }
        2 => {
            if (target.x + 1 < matrix.len()) {
                target.x += 1;
            }
        }
        3 => {
            if (target.y + 1 < matrix.len()) {
                target.y += 1;
            }
        }
        4 => {
            if ((target.y + 1 < matrix.len()) && (target.x + 1 < matrix.len())) {
                target.y += 1;
                target.x += 1;
            }
        }
        5 => {
            if ((target.y != 0) && (target.x != 0)) {
                target.y -= 1;
                target.x -= 1;
            }
        }
        6 => {
            if ((target.x != 0) && (target.y + 1 < matrix.len())) {
                target.y += 1;
                target.x -= 1;
            }
        }
        7 => {
            if ((target.y != 0) && (target.x + 1 < matrix.len())) {
                target.y -= 1;
                target.x += 1;
            }
        }
        _ => (),
    }
    if matrix[target.x][target.y] != 0 {
        return person;
    }

    (*matrix)[target.x][target.y] = person.value;
    if matrix[person.position.x][person.position.y] == person.value {
        (*matrix)[person.position.x][person.position.y] = 0;
    }

    Person {
        value: person.value,
        position: target,
    }
}

fn move_to_door(room: &Arc<Mutex<Vec<Vec<i32>>>>, mut person: Person, door: &Position) -> Person {
    let mut matrix = room.lock().unwrap();
    let mut target = Position {
        x: person.position.x,
        y: person.position.y,
    };

    if (door.x > target.x) {
        target.x += 1;
    }
    if (door.y > target.y) {
        target.y += 1;
    }
    if (door.x < target.x) {
        target.x -= 1;
    }
    if (door.y < target.y) {
        target.y -= 1;
    }

    (*matrix)[person.position.x][person.position.y] = 0;

    if &target == door {
        person.value = -1;
    }

    (*matrix)[target.x][target.y] = person.value;

    Person {
        value: person.value,
        position: target,
    }
}

fn print_room(room: &Arc<Mutex<Vec<Vec<i32>>>>) {
    let matrix = room.lock().unwrap();
    let mut line = String::new();

    for (_, row) in (*matrix).iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            line = match col {
                0 => format!("{line} _"),
                -1 => format!("{line} *"),
                _ => format!("{line} {col}"),
            }
        }
        line = format!("{line} \n");
    }
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", line);
}

fn possible_doors(room_size: usize, qnty_doors: usize) -> Vec<Position> {
    let mut possible_doors = Vec::new();
    for i in 0..room_size {
        possible_doors.push(Position { x: i, y: 0 });
        possible_doors.push(Position {
            x: i,
            y: room_size - 1,
        });
        possible_doors.push(Position { x: 0, y: i });
        possible_doors.push(Position {
            x: room_size - 1,
            y: i,
        });
    }
    possible_doors
        .choose_multiple(&mut rand::thread_rng(), qnty_doors)
        .map(|value| Position {
            x: value.x,
            y: value.y,
        })
        .collect()
}

pub fn start(qnty_people: i32, qnty_doors: usize, room_size: usize, seconds: u64, interval: u64) {
    let room_matrix = Arc::new(Mutex::new(vec![vec![0; room_size]; room_size]));
    let doors = possible_doors(room_size, qnty_doors);

    for door in doors.clone() {
        let mut matrix = room_matrix.lock().unwrap();
        (*matrix)[door.x][door.y] = -1;
    }

    let mut threads = vec![];

    for i in 0..qnty_people {
        let room_matrix = Arc::clone(&room_matrix);
        let doors = doors.clone();
        let handle = thread::spawn(move || {
            let selected_door = doors.choose(&mut rand::thread_rng()).unwrap();
            let mut person: Person = Person {
                value: i,
                position: Position {
                    x: selected_door.x,
                    y: selected_door.y,
                },
            };
            let now = Instant::now();
            loop {
                if now.elapsed().as_secs() > seconds {
                    break;
                }
                person = random_move(&room_matrix, person, selected_door);
                print_room(&room_matrix);
                thread::sleep(Duration::from_millis(interval));
            }

            loop {
                if person.value == -1 {
                    break;
                }
                person = move_to_door(&room_matrix, person, selected_door);
                print_room(&room_matrix);
                thread::sleep(Duration::from_millis(interval));
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
