# Tutorial Source
Youtube  
Introduction to Rust Part 1
Introduction to Rust Part 2
Ryan Levick
Nov 15, 2020

# Progress
- NOV-27-2022: Stopped at Part 1 01:22:08/02:04:04

# Notes
## Create a new rust program
```
./$ cargo new <new_project_folder_name>
```

## Check the source (doesn't build a binary)
```
./$ cargo check
```

## Build the binary for debug mode (doesn't run)
```
./$ cargo build

NOTE: Binary can be found:
./target/debug/<program_name>

NOTE: Binary can be manually run:
./$ ./target/debug/<program_name>
```

## Build the binary for release mode (doesn't run)
```
./$ cargo build --release

NOTE: Binary can be found:
./target/release/<program_name>

NOTE: Binary can be manually run:
./$ ./target/release/<program_name>
```

## Build and Run
```
./$ cargo run
```

## Build and Run with arguments
```
./$ cargo run -- <first_arg> <second_arg>
```

## Check if rust is installed, and what version
```
./$ rustc --version
```

## Update rust
```
./$ rustup update
```

