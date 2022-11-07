use rand::Rng;
use std::sync::mpsc::{self, Sender};
use std::{thread, time::Duration};

struct Position {
    x: usize,
    y: usize,
}

struct Person {
    value: i32,
    position: Position,
}

fn do_move(matrix: &mut [[i32; 10]; 10], person: Person, ty: &Sender<Person>) {
    let mut target_x = person.position.x;
    let mut target_y = person.position.y;

    let mut rng = rand::thread_rng();

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
            if ((target_y + 1 <= matrix.len() - 1) && (target_x + 1 <= matrix.len() - 1)) && (matrix[target_x + 1][target_y + 1] == 0) {
                target_y += 1;
                target_x += 1;
            }
        },
        5 => {
            if ((target_y != 0) && (target_x != 0)) && (matrix[target_x - 1][target_y - 1] == 0) {
                target_y -= 1;
                target_x -= 1;
            }
        },
        6 => {
            if ((target_x != 0) && (target_y + 1 <= matrix.len() - 1)) && (matrix[target_x - 1][target_y + 1] == 0) {
                target_y += 1;
                target_x -= 1;
            }
        },
        7 => {
            if ((target_y != 0) && (target_x + 1 <= matrix.len() - 1)) && (matrix[target_x + 1][target_y - 1] == 0) {
                target_y -= 1;
                target_x += 1;
            }
        },
        _ => (),
    }
    matrix[person.position.x][person.position.y] = 0;
    matrix[target_x][target_y] = person.value;

    ty.send(Person {
        value: person.value,
        position: Position {
            x: target_x,
            y: target_y,
        },
    })
    .unwrap();
}

fn print_room(room: [[i32; 10]; 10]) {
    clearscreen::clear().expect("failed to clear screen");

    let mut line = String::new();

    for (_i, row) in room.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            line = format!("{line} {col}");
        }
        println!("{}", line);
        line = String::new();
    }
}

fn main() {
    let mut room_matrix: [[i32; 10]; 10] = [[0; 10]; 10];
    let (tx, rx) = mpsc::channel();
    let (ty, ry) = mpsc::channel();
    // let mut exits = ArrayVec::<[Position; 40]>::new();

    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let mut person: Person = Person {
            value: rng.gen_range(1..10),
            position: Position { x: 0, y: 0 },
        };
        loop {
            tx.send(person).unwrap();
            person = ry.recv().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    loop {
        for movement in &rx {
            do_move(&mut room_matrix, movement, &ty);
            print_room(room_matrix);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
