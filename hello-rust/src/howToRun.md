Good question 👍

By default, `cargo run` looks for **`src/main.rs`** in your current project.
But you have a few options to run a different `.rs` file:

---

## 1. **Compile & run a single file (no Cargo)**

If you just want to run *any* `.rs` file without creating a project:

```bash
rustc your_file.rs -o your_program
./your_program
```

Example:

```bash
rustc main.rs -o main
./main
```

This is like compiling C with `gcc`.

---

## 2. **Use Cargo with multiple binaries**

Cargo expects a default `src/main.rs`, but you can add **extra binaries** in `src/bin/`.
Each `.rs` file inside `src/bin/` is treated as a standalone program.

For example:

```
my_project/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  └─ bin/
│     ├─ two_sum.rs
│     └─ hello.rs
```

Then you can run:

```bash
cargo run --bin two_sum
cargo run --bin hello
```

---

## 3. **Run a different file inside the same project**

If you want to quickly test another file (say `other.rs`) without moving it to `src/bin`, you can run:

```bash
cargo run --bin other
```

…but Cargo will only recognize it if it’s inside `src/bin/`.

---

✅ So:

* **Quick test** → `rustc file.rs && ./file`
* **Project with multiple programs** → put them in `src/bin/` and use `cargo run --bin name`

---

Do you want me to show you how to **turn your Two Sum program into a `src/bin/two_sum.rs` file** so you can keep multiple Rust exercises in one project and run whichever you want?
