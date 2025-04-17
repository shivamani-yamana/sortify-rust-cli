# 📂 Sortify

> A blazing-fast Rust CLI tool to sort files in a directory using the Merge Sort algorithm.  
> Built with performance, low-level design, and DSA in mind.

---

## 🚀 Features

- 🔢 Sorts files by **size** using **merge sort**
- 📁 Recursively walks directories
- 🧪 Tested with built-in Rust test framework
- ⚡ Fast and memory-efficient
- 🧱 Beginner-friendly code structure

---

## 📦 Installation

```bash
git clone https://github.com/your-username/sortify.git
cd sortify
cargo build --release
```

---

## 🛠️ Usage

```bash
cargo run -- --path <directory-path>
```

### Example:

```bash
cargo run -- --path ./Downloads
```

This will print the list of files sorted by size.

---

## 🧪 Run Tests

```bash
cargo test
```

---

## 🧩 Project Structure

```
sortify/
├── src/
│   ├── main.rs        # CLI entry point
│   ├── sorter.rs      # Merge sort logic
│   └── file_utils.rs  # File info utilities
├── Cargo.toml
└── README.md
```

---

## 📚 Concepts Used

- 🧠 Merge Sort (DSA)
- 🗃️ File metadata handling
- 📜 CLI parsing with [`clap`](https://crates.io/crates/clap)
- 🏃‍♂️ Directory traversal with [`walkdir`](https://crates.io/crates/walkdir)
- 🧪 Unit testing

---

## 🧰 Dependencies

| Crate     | Description                |
|-----------|----------------------------|
| `clap`    | CLI argument parsing       |
| `walkdir` | Recursive directory walker |

---

## 💡 Future Plans

- [ ] Sorting by name, extension, or last modified
- [ ] TUI mode with `ratatui`
- [ ] Export sorted list to file
- [ ] Real-time folder watching
- [ ] Multi-threaded file processing


---

## 📝 License

MIT License. Feel free to use, contribute, and learn!
