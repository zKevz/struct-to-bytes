# Struct-To-Bytes
Implementation of struct to bytes conversion and vice versa using [serde](https://serde.rs/) framework

## Notes
Before using, make sure to add [serde](https://docs.rs/serde/latest/serde/) in your `Cargo.toml`
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## Struct to bytes conversion:
```rust
#[derive(Serialize, Deserialize)]
struct Employee {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Company {
    name: String,
    employees: Vec<Employee>,
}

let company = Company {
    name: String::from("Walter White Company"),
    employees: vec![Employee {
        name: String::from("Jesse Pinkman"),
    }],
};

let bytes = ser::to_bytes(&company).unwrap();
```

## Bytes to struct conversion:
```rust
#[derive(Serialize, Deserialize)]
struct Employee {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Company {
    name: String,
    employees: Vec<Employee>,
}

// Suppose you have a chunk of bytes here
let company: Company = de::from_bytes(&bytes).unwrap();
```

## Developers
* [kevz](https://github.com/zKevz)
