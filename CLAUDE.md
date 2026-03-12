# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
# Build
cargo build

# Run with defaults (10 people, 1 door, 10x10 room, 5s duration, 200ms interval)
cargo run

# Run with custom parameters
cargo run -- --people 50 --room-size 20 --doors 5 --seconds 10 --interval 150

# See all CLI options
cargo run -- --help

# Install as a system-wide CLI tool
cargo install --path .

# Cross-compile for Windows (requires x86_64-w64-mingw32-gcc)
cargo build --target x86_64-pc-windows-gnu
```

## Architecture

Three source files:

- `src/main.rs` — Parses CLI arguments via `clap` (derive API), validates input constraints, then calls `room::start()`.
- `src/room.rs` — Simulation logic. Exposes one public function: `start(qnty_people, qnty_doors, room_size, seconds, interval)`. Manages terminal lifecycle and the render thread internally.
- `src/ui.rs` — All ratatui rendering. Exports `setup_terminal`, `restore_terminal`, `render`, `SimStats`, and `SimPhase`.

### Simulation model (`src/room.rs`)

The room is an `Arc<Mutex<Vec<Vec<i32>>>>` matrix shared across threads. Each person runs in its own thread. A separate dedicated render thread owns the `Terminal` (ratatui requires single-thread draw calls).

**Matrix values:**
- `0` — empty cell
- `-1` — door position
- positive integer — person identifier

**Two-phase movement per person thread:**
1. **Random phase** (duration controlled by `seconds`): each person moves randomly in 8 directions, blocking if the target cell is occupied.
2. **Exit phase**: person moves deterministically toward its assigned door (diagonal steps allowed), then marks itself `-1` upon reaching the door.

Doors are placed randomly on the border tiles (`x=0`, `x=room_size-1`, `y=0`, `y=room_size-1`). Each person is assigned one door at spawn and starts from that door's position.

**Shared state between threads:**
- `Arc<Mutex<Vec<Vec<i32>>>>` — the room matrix (mutated by person threads, cloned by render thread each tick)
- `Arc<Mutex<SimStats>>` — scalar counters (people remaining, phase, elapsed time) updated by person threads
- `Arc<AtomicBool>` (`should_quit`) — set by render thread on `q`/`Esc`, or by `start()` after all person threads finish; person threads check it each tick for early exit

**`start()` lifecycle:** setup terminal → spawn render thread → spawn person threads → join person threads → set `should_quit` → join render thread (which calls `restore_terminal`).

**Key constraint**: `people < room_size²` and `doors < room_size²` (validated in `main.rs` before calling `start`).
