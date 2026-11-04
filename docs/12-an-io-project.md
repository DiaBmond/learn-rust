# Chapter 12: An I/O Project — Building a Command Line Program

## Overview

This chapter combines concepts from previous chapters to build `minigrep`, a simplified version of the command-line search tool `grep`.

The program accepts a search query and a file path, reads the file, searches for matching lines, and prints the results.

The project demonstrates:

- Reading command-line arguments
- Reading files from the filesystem
- Handling errors with `Result`
- Organizing code into binary and library crates
- Writing automated tests
- Working with environment variables
- Separating standard output from standard error

## 12.1 Accepting Command Line Arguments

Rust provides `std::env::args()` to access arguments passed to a program.

```rust
use std::env;

let args: Vec<String> = env::args().collect();
```

`env::args()` returns an iterator over command-line arguments. The `collect()` method converts these values into a `Vec<String>`.

For example:

```bash
cargo run -- frog poem.txt
```

The arguments are approximately:

```text
args[0] = "target/debug/mini_project_04_minigrep"
args[1] = "frog"
args[2] = "poem.txt"
```

The first argument is the executable path, followed by the search query and file path.

The `--` separator tells Cargo that subsequent arguments belong to the program rather than Cargo itself.

### Borrowing Arguments

```rust
let query = &args[1];
let file_path = &args[2];
```

The `&` operator borrows values from the vector without transferring ownership.

Directly moving a `String` out of a vector through indexing is not allowed because `String` does not implement `Copy`.

Indexing also requires care: accessing an index that does not exist causes a panic.

## 12.2 Reading a File

Rust provides `std::fs::read_to_string()` to read an entire UTF-8 file into a `String`.

```rust
use std::fs;

let contents = fs::read_to_string(file_path)?;
```

The function returns `Result<String, std::io::Error>`.

- `Ok(contents)` indicates that the file was read successfully.
- `Err(error)` indicates that the file could not be read or decoded as UTF-8.

The `?` operator propagates an error to the calling function instead of requiring an explicit `match`.

## 12.3 Refactoring to Improve Modularity and Error Handling

As a program grows, separating responsibilities makes it easier to understand, maintain, and test.

The `minigrep` exercise introduces a `Config` struct to group the program's configuration.

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
```

### Building the Configuration

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

`Config::build()` validates the arguments and returns a `Result`.

The `clone()` method creates owned `String` values for the configuration because the original arguments are borrowed.

### Handling Configuration Errors

```rust
let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

`unwrap_or_else()` extracts the configuration when the result is `Ok`. When the result is `Err`, it executes the provided closure.

`process::exit(1)` terminates the program with a nonzero exit status, indicating failure.

### Running the Application

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
```

The `run()` function performs the application's main operations and returns a `Result` instead of terminating the process itself.

`Box<dyn Error>` allows the function to return different error types through a trait object.

This separates normal application operations from error reporting and process termination.

### Project Organization

The exercise uses both `src/main.rs` and `src/lib.rs`.

In the current implementation:

- `main.rs` handles arguments, configuration, file reading, output, and application-level errors.
- `lib.rs` contains the search functions and their unit tests.

The general principle is to keep reusable, testable logic separate from command-line application setup.

## 12.4 Developing the Library's Functionality with Test-Driven Development

Test-Driven Development (TDD) is an approach in which tests are written before the corresponding implementation.

A common workflow is:

1. Write a test describing the expected behavior.
2. Run the test and confirm that it fails.
3. Implement the functionality.
4. Run the test again and confirm that it passes.
5. Refactor while keeping the tests passing.

### Case-Sensitive Search

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

The function uses:

- `lines()` to iterate through the file content line by line
- `contains()` to check whether a line contains the query
- `Vec` to collect matching lines

The search is case-sensitive, so `"Rust"` and `"rust"` are different.

### Lifetime Annotations

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
```

The lifetime `'a` connects the returned string slices to `contents`.

This tells the compiler that the returned references cannot outlive the content they borrow from.

The returned lines are borrowed slices of the original content rather than newly allocated strings.

### Testing the Search Function

```rust
#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}
```

The test verifies that the search returns only lines containing the exact query.

## 12.5 Working with Environment Variables

Environment variables allow users to configure a program without changing its source code.

The `minigrep` project uses `IGNORE_CASE` to enable case-insensitive searching.

```rust
let ignore_case = env::var("IGNORE_CASE").is_ok();
```

`env::var()` returns a `Result`.

The `is_ok()` method returns `true` when the variable exists and its value can be read successfully as a string.

This checks whether the variable is available, not whether its value is `"true"` or `"1"`.

### Case-Insensitive Search

```rust
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

The function converts the query and each line to lowercase for comparison.

However, it returns the original lines, preserving their original capitalization.

Example on macOS or Linux:

```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```

## 12.6 Writing Error Messages to Standard Error Instead of Standard Output

Command-line programs commonly use two output streams:

- **Standard output (`stdout`)** for normal results
- **Standard error (`stderr`)** for error messages

Rust provides separate macros for these streams.

```rust
println!("Search result");
eprintln!("Application error");
```

Using `stderr` for errors allows users to redirect normal output without mixing error messages into the output file.

For example:

```bash
cargo run -- rust poem.txt > output.txt
```

Normal search results are written to `output.txt`, while errors remain visible in the terminal.

### Exit Status

```rust
process::exit(1);
```

By convention:

- Exit status `0` indicates success.
- A nonzero exit status indicates failure.

A command-line program should report errors clearly and return an appropriate exit status.

## Running the Project

Run a case-sensitive search:

```bash
cargo run -- duct poem.txt
```

Run a case-insensitive search on macOS or Linux:

```bash
IGNORE_CASE=1 cargo run -- rust poem.txt
```

Run automated tests:

```bash
cargo test
```

Check code formatting:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy
```

## Key Takeaways

- `env::args()` provides command-line arguments.
- `collect()` can convert an iterator into a vector.
- `fs::read_to_string()` reads UTF-8 file contents into a `String`.
- `Config` groups related application settings.
- `Result` and `?` support recoverable error handling.
- `unwrap_or_else()` handles errors using a closure.
- `Box<dyn Error>` enables returning different error types through a common trait object.
- `lines()` and `contains()` provide simple text-search functionality.
- Lifetime annotations ensure returned string slices remain valid.
- Automated tests verify search behavior and help prevent regressions.
- Environment variables configure program behavior externally.
- `stdout` and `stderr` should be used for normal output and errors respectively.
- A nonzero process exit status communicates failure to the operating system.