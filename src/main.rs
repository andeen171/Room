#![allow(unused)]
mod room;
use clap::Parser;
use room::start;

/// Simulation of a room with persons that then leave though a door
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Quantity of persons in the room, limited by the room size
    #[arg(short, long, default_value_t = 10)]
    people: i32,

    /// Quantity of doors in the room, limited by the room size
    #[arg(short, long, default_value_t = 1)]
    doors: usize,

    /// Size of the room or the matrix, a room size of 10 will have 100 elements
    #[arg(short, long, default_value_t = 10)]
    room_size: usize,

    /// Duration of the simulation before the people start heading out
    #[arg(short, long, default_value_t = 5)]
    seconds: u64,
}

fn main() {
    let args = Cli::parse();

    if args.people >= args.room_size.pow(2).try_into().unwrap() {
        eprintln!("People must be less than the room size squared! It is recommended half or less for a good visualization.");
        return;
    }

    if args.doors >= args.room_size.pow(2) {
        eprintln!("Not enough border tiles around the room for the specified door number!");
        return;
    }

    start(args.people, args.doors, args.room_size, args.seconds);
}
