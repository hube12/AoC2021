use std::process::Output;

pub trait Test: TestR + TestL {}

pub trait TestR {
    type Output: std::fmt::Display;
    fn run(&self) -> Self::Output;
}

pub trait TestL {
    type Output: std::fmt::Display;
    fn run(&self) -> Self::Output;
}

pub struct Test1;

impl Test for Test1 {}

impl TestL for Test1 {
    type Output = String;

    fn run(&self) -> Self::Output {
        String::from("1L")
    }
}

impl TestR for Test1 {
    type Output = String;

    fn run(&self) -> Self::Output {
        String::from("1R")
    }
}

pub struct Test2;

impl Test for Test2 {}

impl TestL for Test2 {
    type Output = String;

    fn run(&self) -> Self::Output {
        String::from("2L")
    }
}

impl TestR for Test2 {
    type Output = String;

    fn run(&self) -> Self::Output {
        String::from("2R")
    }
}

pub fn match_it<T>(x: &str) -> Box<T>
where
    T: Test,
    T::TestR::Output: std::fmt::Display,
    T::TestL::Output: std::fmt::Display,
{
    match x {
        "1" => Box::new(Test1 {}),
        "2" => Box::new(Test2 {}),
        _ => unreachable!(),
    }
}

fn main() {
    match_it("1").run()
}
