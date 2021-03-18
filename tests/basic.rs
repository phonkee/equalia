#![allow(dead_code)]

use equalia::*;
use std::time::Duration;


#[derive(Equalia)]
#[equalia(hash)]
struct Example {
    id: u8,

    #[equalia(only)]
    name: String,

    #[equalia(only, map = "map_func")]
    address: String,

    #[equalia(skip)]
    duration: Duration,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


