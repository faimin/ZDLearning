/*
需先mod再use
*/
mod person;
pub mod collection_learn;
pub mod mut_object;
mod leet_code;

use collection_learn::map_test;

// use self::person::{Person, Protocol};
use crate::person::{Person, Protocol};

fn main() {
    println!("Hello, world!");

    let person = Person {
        name: "Zero.D.Saber".to_string(),
        age: 120,
    };
    println!("person = {:#?}", person);

    person.say("三千院凪");
    let like = person.like_what();
    let hate = person.hate_what();

    println!("like: {} , hate: {}", like, hate);

    hash_map_sample();

    mut_object::mut_string();
}

fn hash_map_sample() {
    map_test();
}