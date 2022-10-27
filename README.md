# Struct-To-Bytes
Implementation of struct to bytes conversion and vice versa using [serde](https://serde.rs/) framework

## Notes
Before using, make sure to add [serde](https://docs.rs/serde/latest/serde/) in your `Cargo.toml`
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## Struct To Bytes Conversion
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

## Bytes To Struct Conversion
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

## Format
The produced output will be in a packed struct format. For strings or sequences, there will be 2 bytes of padding indicating their size or length. However, you can customize your own padding format by using `FormatOptions`.
```rust
pub struct FormatOptions {
    pub short_length: bool,
    pub short_variant_index: bool,
}
```
`short_length`:
* If `true`, strings or sequences size padding will be in 2 bytes
* If `false`, strings or sequences size padding will be in 4 bytes

`short_variant_index`:
* If `true`, enum index padding will be in 2 bytes
* If `false`, enum index padding will be in 4 bytes

Note that you need to call `se::to_bytes_with_options` / `de::from_bytes_with_options` instead of `se::to_bytes` / `de::from_bytes`

## Developers
* [kevz](https://github.com/zKevz)
