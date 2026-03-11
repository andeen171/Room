use crate::ui::{restore_terminal, setup_terminal, render, SimPhase, SimStats};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
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

fn random_move(matrix: &Arc<Mutex<Vec<Vec<i32>>>>, person: Person, _door: &Position) -> Person {
    let mut rng = rand::thread_rng();
    let mut matrix = matrix.lock().unwrap();
    let mut target = Position {
        x: person.position.x,
        y: person.position.y,
    };

    match rng.gen_range(0..8) {
        0 => {
            if target.x != 0 {
                target.x -= 1;
            }
        }
        1 => {
            if target.y != 0 {
                target.y -= 1;
            }
        }
        2 => {
            if target.x + 1 < matrix.len() {
                target.x += 1;
            }
        }
        3 => {
            if target.y + 1 < matrix.len() {
                target.y += 1;
            }
        }
        4 => {
            if (target.y + 1 < matrix.len()) && (target.x + 1 < matrix.len()) {
                target.y += 1;
                target.x += 1;
            }
        }
        5 => {
            if target.y != 0 && target.x != 0 {
                target.y -= 1;
                target.x -= 1;
            }
        }
        6 => {
            if target.x != 0 && target.y + 1 < matrix.len() {
                target.y += 1;
                target.x -= 1;
            }
        }
        7 => {
            if target.y != 0 && target.x + 1 < matrix.len() {
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

    if door.x > target.x {
        target.x += 1;
    }
    if door.y > target.y {
        target.y += 1;
    }
    if door.x < target.x {
        target.x -= 1;
    }
    if door.y < target.y {
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

    let stats = Arc::new(Mutex::new(SimStats {
        people_remaining: qnty_people as usize,
        total_people: qnty_people as usize,
        total_doors: qnty_doors,
        room_size,
        elapsed_secs: 0,
        phase: SimPhase::Random,
    }));

    let should_quit = Arc::new(AtomicBool::new(false));

    // Spawn render thread
    let render_matrix = Arc::clone(&room_matrix);
    let render_stats = Arc::clone(&stats);
    let render_quit = Arc::clone(&should_quit);
    let sim_start = Instant::now();

    let render_handle = thread::spawn(move || {
        let mut terminal = match setup_terminal() {
            Ok(t) => t,
            Err(_) => return,
        };

        loop {
            if render_quit.load(Ordering::Relaxed) {
                break;
            }

            // Non-blocking keyboard poll
            if event::poll(Duration::ZERO).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    if key.kind == KeyEventKind::Press
                        && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                    {
                        render_quit.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }

            // Update elapsed time in stats
            {
                let mut s = render_stats.lock().unwrap();
                s.elapsed_secs = sim_start.elapsed().as_secs();
            }

            let snapshot = render_matrix.lock().unwrap().clone();
            let stats_snap = render_stats.lock().unwrap().clone();

            let _ = terminal.draw(|f| render(f, &snapshot, &stats_snap));

            thread::sleep(Duration::from_millis(interval));
        }

        let _ = restore_terminal(&mut terminal);
    });

    // Spawn person threads
    let mut threads = vec![];

    for i in 1..=qnty_people {
        let room_matrix = Arc::clone(&room_matrix);
        let doors = doors.clone();
        let stats = Arc::clone(&stats);
        let should_quit = Arc::clone(&should_quit);

        let handle = thread::spawn(move || {
            let selected_door = doors.choose(&mut rand::thread_rng()).unwrap();
            let mut person = Person {
                value: i,
                position: Position {
                    x: selected_door.x,
                    y: selected_door.y,
                },
            };
            let now = Instant::now();

            // Phase 1: random movement
            loop {
                if should_quit.load(Ordering::Relaxed) {
                    return;
                }
                if now.elapsed().as_secs() > seconds {
                    break;
                }
                person = random_move(&room_matrix, person, selected_door);
                thread::sleep(Duration::from_millis(interval));
            }

            // Phase 2: move toward door
            {
                let mut s = stats.lock().unwrap();
                s.phase = SimPhase::Exiting;
            }

            loop {
                if should_quit.load(Ordering::Relaxed) {
                    return;
                }
                if person.value == -1 {
                    break;
                }
                person = move_to_door(&room_matrix, person, selected_door);
                thread::sleep(Duration::from_millis(interval));
            }

            // Person has exited
            {
                let mut s = stats.lock().unwrap();
                if s.people_remaining > 0 {
                    s.people_remaining -= 1;
                }
            }
        });
        threads.push(handle);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    should_quit.store(true, Ordering::Relaxed);
    render_handle.join().unwrap();
}
