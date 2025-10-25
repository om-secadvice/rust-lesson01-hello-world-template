#[path = "../src/main.rs"]
mod main_file;
use main_file::say_hello;

#[test]
fn test_say_hello() {
    assert_eq!(say_hello(), "Hello, world!");
}

