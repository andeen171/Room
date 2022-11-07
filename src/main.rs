use rand::Rng;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

struct Position {
    x: usize,
    y: usize,
}

struct Person {
    value: i32,
    position: Position,
}

fn do_move(matrix: &Arc<Mutex<[[i32; 10]; 10]>>, person: Person) -> Person {
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

fn print_room(room: &Arc<Mutex<[[i32; 10]; 10]>>) {
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

fn main() {
    let room_matrix = Arc::new(Mutex::new([[0; 10]; 10]));
    let mut threads = vec![];

    for i in 1..10 {
        let room_matrix = Arc::clone(&room_matrix);
        let handle = thread::spawn(move || {
            let mut person: Person = Person {
                value: i,
                position: Position { x: 0, y: 0 },
            };
            loop {
                person = do_move(&room_matrix, person);
                print_room(&room_matrix);
                thread::sleep(Duration::from_millis(100));
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

}
