# The Room
Rust project to simulate the behavior of random movements inside a matrix, using multiple threads to make movements simuntaniously and Arc Mutexes to preserve and share the concurrent state of the matrix.
 
# Instructions

First make sure you have rustup installed, if you don't have it you can download it from the <a href="https://www.rust-lang.org/tools/install">rust website</a>
 
The website also have help for any possible problem you can ran into while setting things up

Then clone the repo

```sh
git clone https://github.com/andeen171/Room.git
cd room
```

Now inside the folder you can run the program with:

```sh
cargo run
```
or using parameters
```sh
cargo run -- --help
cargo run -- --people 50 --room 20 --doors 5 --seconds 10
```

Or the recommended way, installing it as a cli, running this command:
```sh
cargo install --path .
```

and then using it anywhere in the system via the 'room' keyword
```sh
room --help
room --people 50 --room 20 --doors 5 --seconds 10
```
