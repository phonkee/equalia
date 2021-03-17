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

    #[equalia(from = "value_func")]
    value2: u8,
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

When single field ins struct can identify equality with help of function.

```rust
#[derive(Equalia)]
#[equalia(hash)]
pub struct Entity {
    
    #[equalia(only = "value_func")]
    id: u8,

    // this value will be ignored
    value2: u8,
}
```


When you provide `#[equalia(hash)]` for struct/enum equalia will automatically
implement `Hash` trait from given configuration.

#

#### Enums:

First define __Entity__ from previous example and add enum example __(Enumeration)__
§
```rust
#[derive(Equalia)]
#[equalia(hash)]
pub struct Entity {
    
    #[equalia(only)]
    id: u8,
    
    // this value will be ignored
    value2: u8,
}

#[derive(Equalia)]
#[equalia(hash)]
pub enum Enumeration {
    First,
    Second,
    Third(String),
    Fourth(Entity),
}
```

# author
Peter Vrba <phonkee@pm.me>