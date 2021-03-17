# equalia

This package helps you with equal traits for structs. You can provide
which struct fields to compare and which not.

# example

```rust
#[derive(Equalia)]
pub struct Entity {
    
    #[equalia(skip)]
    value1: u8,

    #[equalia(value = "value_func")]
    value2: u8,
}
```

or when single field can identify equality

```rust
#[derive(Equalia)]
pub struct Entity {
    
    #[equalia(only)]
    id: u8,

    value2: u8,
}
```

# author
Peter Vrba <phonkee@pm.me>