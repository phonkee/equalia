#![allow(dead_code)]

use equalia::*;
use std::time::Duration;


#[derive(Equalia)]
#[equalia(hash)]
struct Example {
    id: u8,

    #[equalia(only)]
    name: String,

    #[equalia(only)]
    address: String,

    #[equalia(skip)]
    duration: Duration,
}

#[derive(Debug, Default, Equalia)]
struct OnlyTest {
    #[equalia(only)]
    id: u8,
    name: String,
    #[equalia(only)]
    address: String,
}

fn hello(_: &u8) -> u8 {
    1
}


#[derive(Debug, Default, Equalia)]
struct MapTest {
    #[equalia(map = "hello")]
    id: u8,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s1 = OnlyTest::default();
        let mut s2 = OnlyTest::default();
        s1.id = 1;
        s1.name = "other".to_owned();
        s2.id = 1;
        assert_eq!(s1, s2);
    }
}


