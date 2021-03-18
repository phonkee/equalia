#![allow(dead_code)]

use equalia::*;
use std::time::Duration;


#[derive(Equalia)]
struct Example {
    id: u8,
    name: String,

    #[equalia(skip)]
    duration: Duration,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}


