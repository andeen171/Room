use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

struct Person {
    value: i32,
    position: Position,
}

fn random_move(matrix: &Arc<Mutex<Vec<Vec<i32>>>>, person: Person) -> Person {
    let mut target_x = person.position.x;
    let mut target_y = person.position.y;

    let mut rng = rand::thread_rng();

    let mut matrix = matrix.lock().unwrap();

    match rng.gen_range(0..8) {
        0 => {
            if (target_x != 0) && (matrix[target_x - 1][target_y] == 0) {
                target_x -= 1;
            }
        }
        1 => {
            if (target_y != 0) && (matrix[target_x][target_y - 1] == 0) {
                target_y -= 1;
            }
        }
        2 => {
            if (target_x + 1 <= matrix.len() - 1) && (matrix[target_x + 1][target_y] == 0) {
                target_x += 1;
            }
        }
        3 => {
            if (target_y + 1 <= matrix.len() - 1) && (matrix[target_x][target_y + 1] == 0) {
                target_y += 1;
            }
        }
        4 => {
            if ((target_y + 1 <= matrix.len() - 1) && (target_x + 1 <= matrix.len() - 1))
                && (matrix[target_x + 1][target_y + 1] == 0)
            {
                target_y += 1;
                target_x += 1;
            }
        }
        5 => {
            if ((target_y != 0) && (target_x != 0)) && (matrix[target_x - 1][target_y - 1] == 0) {
                target_y -= 1;
                target_x -= 1;
            }
        }
        6 => {
            if ((target_x != 0) && (target_y + 1 <= matrix.len() - 1))
                && (matrix[target_x - 1][target_y + 1] == 0)
            {
                target_y += 1;
                target_x -= 1;
            }
        }
        7 => {
            if ((target_y != 0) && (target_x + 1 <= matrix.len() - 1))
                && (matrix[target_x + 1][target_y - 1] == 0)
            {
                target_y -= 1;
                target_x += 1;
            }
        }
        _ => (),
    }
    (*matrix)[person.position.x][person.position.y] = 0;
    (*matrix)[target_x][target_y] = person.value;

    Person {
        value: person.value,
        position: Position {
            x: target_x,
            y: target_y,
        },
    }
}

fn move_to_door() {
    // TODO: Exiting movement
}

fn print_room(room: &Arc<Mutex<Vec<Vec<i32>>>>) {
    let matrix = room.lock().unwrap();
    let mut line = String::new();

    for (_, row) in (*matrix).iter().enumerate() {
        for (_, col) in row.iter().enumerate() {
            line = format!("{line} {col}");
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
        possible_doors.push(Position { x: i, y: room_size - 1 });
        possible_doors.push(Position { x: 0, y: i });
        possible_doors.push(Position { x: room_size - 1, y: i });
    }
    possible_doors
        .choose_multiple(&mut rand::thread_rng(), qnty_doors)
        .map(|value| Position {
            x: value.x,
            y: value.y,
        })
        .collect()
}

pub fn start(qnty_people: i32, qnty_doors: usize, room_size: usize, seconds: u64) {
    let room_matrix = Arc::new(Mutex::new(vec![vec![0; room_size]; room_size]));
    let doors = possible_doors(room_size, qnty_doors);

    let mut threads = vec![];

    for i in 0..qnty_people {
        let room_matrix = Arc::clone(&room_matrix);
        let doors = doors.clone();
        let handle = thread::spawn(move || {
            let selected_door = doors.choose(&mut rand::thread_rng()).unwrap();
            let mut person: Person = Person {
                value: i + 1,
                position: Position { x: selected_door.x, y: selected_door.y },
            };
            let now = Instant::now();
            loop {
                person = random_move(&room_matrix, person);
                print_room(&room_matrix);
                if now.elapsed().as_secs() > seconds {
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
