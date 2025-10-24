# CLI Counter App (Rust)

A simple interactive **command-line counter** written in Rust.  
It lets you increment, decrement, reset, and view your counter — all from your terminal.

---

## Overview

This app demonstrates:
- Structs and methods in Rust
- Error handling with `Result`
- Mutable references (`&mut`)
- Basic CLI input/output with `std::io`
- Match control flow and loop logic

---

## Code Summary

The app defines a `Counter` struct with:
```rust
pub struct Counter {
    value: u64,
    max: u64,
}
```
It supports:
```

inc() → Increment the counter (with max cap)

dec() → Decrement the counter (with min cap)

reset() → Reset to zero

current_value() and max_value() → Read-only accessors
```
## The interactive CLI loop allows the user to choose actions using numeric input.

# Example Usage

1. Run the app
```

cargo run
```


2. Sample output
```

Counter created with the max value 5


=== CLI Counter App Menu ===
1. View Count
2. Increment Counter
3. Decrement Counter
4. Reset Counter
5. Exit 
6. Show Menu
==============================


Please enter your choice: 2
Counter Incremented!

Please enter your choice: 1
Current counter value: 1
```

3. Exit
```

Please enter your choice: 5
Exiting. Final count: 3
```

# Project Structure
```
counter/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

# Features
```
✅ Interactive CLI menu
✅ Input validation
✅ Error handling for max/min bounds
✅ Clean and modular struct-based design
```
