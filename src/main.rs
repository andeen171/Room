use std::thread;
use std::time::Duration;

fn main() {
    let my_int_matrix:[[i32;3];3] = [[0;3];3];

    let handle = thread::spawn(|| {
        for _i in 1..10 {
            // println!("second thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let mut line = String::new();

    for (_i, row) in my_int_matrix.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            
            line = format!("{line} {col}");
        }
        println!("{}", line);
        line = String::new();
    }

    handle.join().unwrap();
}