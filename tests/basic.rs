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

#[derive(Debug, Default, Equalia)]
struct OnlyTest {
    #[equalia(only)]
    id: u8,
    name: String,
    #[equalia(only)]
    address: String,
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


