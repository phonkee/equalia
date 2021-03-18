# Equalia

This package helps you with implementation of Eq and PartialEq for structs.
You can provide which struct fields to compare and which not.

#

#### Structs
When we want to define which fields are omitted, or we want to provide
custom function to return value to be compared.

```rust
#[derive(Equalia)]
#[equalia(hash)]
pub struct Entity {
    
    #[equalia(skip)]
    value1: u8,

    #[equalia(map = "map_func")]
    value2: u8,
}

// map function that changes value
fn map_func(input: &u8) -> u8 {
    input * 2
}

```

When single field ins struct can identify equality.

```rust
#[derive(Equalia)]
#[equalia(hash)]
pub struct Entity {
    
    #[equalia(only)]
    id: u8,

    // this value will be ignored
    value2: u8,
}
```

## Hash

When you provide `#[equalia(hash)]` for struct/enum equalia will automatically
implement `Hash` trait from given configuration.


# author
Peter Vrba <phonkee@pm.me>