# todo-json-rs

A small Rust CLI todo app backed by a JSON file.

I made this as a learning project while getting more comfortable with Rust. The goal was to build something simple and useful while practicing:

- CLI argument parsing with `clap`
- JSON serialization with `serde` and `serde_json`
- file storage and basic app structure
- splitting code into small modules

## What it can do

- initialize or reset a local todo database
- add, show, edit, complete, uncomplete, search, and remove todos
- list all todos or filter by completed and pending status
- print output as a simple table or as JSON
- store data in a default app data directory or a custom directory

## Tech

- Rust
- `clap`
- `serde`
- `serde_json`
- `time`
- `anyhow`

## Running it

```bash
cargo run -- --help
```

## Example usage

Initialize the database:

```bash
cargo run -- db init
```

Add a todo:

```bash
cargo run -- todo add "learn rust ownership"
```

List todos:

```bash
cargo run -- todo ls
```

List only completed todos:

```bash
cargo run -- todo ls --done
```

List only pending todos:

```bash
cargo run -- todo ls --pending
```

Show one todo:

```bash
cargo run -- todo show 1
```

Search todos:

```bash
cargo run -- todo search rust
```

Mark a todo as done:

```bash
cargo run -- todo done 1
```

Mark a todo as not done:

```bash
cargo run -- todo undone 1
```

Edit a todo:

```bash
cargo run -- todo edit 1 "learn rust ownership and borrowing"
```

Remove a todo:

```bash
cargo run -- todo rm 1
```

JSON output:

```bash
cargo run -- --json todo ls
```

## Database location

By default, the app stores its data in your platform's data directory under `todo-json-rs`.

You can override that with either:

- `--db-dir <PATH>`
- `TODO_JSON_RS_DIR=<PATH>`

Example:

```bash
cargo run -- --db-dir ./data db init
```

## Commands

```text
todo-json-rs [OPTIONS] <COMMAND>

Commands:
  todo
  db

Options:
  --json
  --db-dir <DB_DIR>
```

Todo subcommands:

```text
add
rm
show
ls
search
done
undone
edit
```

## Why this project

This is intentionally small. It was mainly a way to learn Rust by building a complete little CLI app with argument parsing, persistence, and basic output formatting.
