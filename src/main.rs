#![crate_name = "quickstart_rs"]
#![allow(dead_code)]

mod collection;
mod ownership;

use collection::vector_demo;
use ownership::ownership_demo;
use quick_lib::person::Person as quick_lib_person;
use quick_lib::{indirect_access, person};
use std::cmp::max;

//structs
/// A human being is represented here
struct Person {
    /// A Person must have a name and age
    name: String,
    age: u8,
}

impl Person {
    fn age(&self) -> u8 {
        self.age
    }
}
impl Legal for Person {
    fn is_legal_age(&self) -> bool {
        println!("Is legal age...");
        self.age > 18
    }
}

trait Legal {
    fn is_legal_age(&self) -> bool;
}

trait Signed {
    fn is_negative(&self) -> bool;
}

struct Point<T> {
    x: T,
    y: T,
}

// structs are init by struct literals

#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

impl Signed for Number {
    fn is_negative(&self) -> bool {
        self.value < 0
    }
}

impl std::ops::Neg for Number {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }
    }
}

fn main() {
    //half hour of rust
    //variable binding: let introduce variable bindings
    // let <variable-name>: <type> = value;
    // int: i8, i16, i32, i64, i128
    // i32 is default
    // compiler will prevents form using uninitialized variable
    let x: i32 = 42;
    println!("{}", x);
    // trowing values away
    // let _ = value;
    // trow away as value: constant, function return etc.
    let _ = 56;
    // name starting with underscore: compiler won't warm about them begin unused
    let _x = 42;
    // shadow binding:
    let _x = 13;
    let _x = _x + 3;
    // tuple: fixed-length collections of values of different types
    let pair = ('a', 15);
    println!("{} {}", pair.0, pair.1);
    // annotated tuples
    let pair: (char, i32) = ('a', 17);
    // destructuring tuples
    let (some_char, some_int) = pair;
    println!("{} {}", some_char, some_int);
    // statements
    let x = 5;
    let y = 3;
    let _z = x + y;
    let x = [1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{}", x);
    // block: a pair of brackets
    // block are also expression
    let x = "out";
    {
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
    let x = {
        let y = 1;
        let z = 2;
        y + z //this is the *tail* - what the whole block will evaluate to
    };
    println!("{}", x);

    // everything is an expression - if, match etc.
    // fields access and method calling
    let vikas = get_person();
    println!("{}", vikas.name);
    let long_word = "Hippopotamus";
    println!(
        "The length of the word '{}': {}",
        long_word,
        long_word.len()
    );
    // modules and namespace scoping
    let least = std::cmp::min(10, 45);
    println!("The smallest number is {}", least);
    let max = max(10, 4);
    println!("The largest number is {}", max);
    // wild card imports from a namespace
    // types are namespaces too
    let _length = "vikas".len();
    let _length = str::len("vikas");
    // struct initialized using struct literal
    let person1 = Person {
        name: "Happy".to_owned(),
        age: 18,
    };
    let person2 = Person {
        name: "Amar".to_owned(),
        age: 30,
    };
    // destructing structs
    let Person { name, age } = person1;
    println!("{} {}", name, age);
    println!("{}", person2.age());
    println!("{}", person2.is_legal_age());
    // primitive types and some non-primitive types are scoped by default

    // patterns and destructuring - if let
    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    let _three = Number { odd: true, ..two };
    let _two = Number { ..two };
    get_number_detail_if_let(one);
    get_number_detail_if_let(two);
    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    print_match(one);
    print_match(two);
    let _name = "Peter";
    // exhaustive matches
    // "catch-all" pattern

    // immutability
    // variable binding are immutable by default: means their interior can't be mutated or resigned
    // making variable mutable
    let mut val = Number {
        odd: true,
        value: 17,
    };
    val.value = 13;

    // Struct --- Impl --- Traits
    // orphan rule
    // one of your traits on anyone’s type
    // anyone’s trait on one of your types
    // but not a foreign trait on a foreign type

    // foreign type are those type which originates outside the rust-ecosystem
    // accessed via FFI (foreign function interface)
    // wrapped in rust-type allowing safe interaction

    // Self type: impl block is always for a type

    // marker traits: certain method are called on a type, not type implements certain methods
    // deriving traits: some common traits can be implemented automatically used derive attribute
    // generics: function can be generic
    // foobar(&val);
    // foobar(&val);
    let first_name: String = String::from("Peter");
    let reference_name: &String = &first_name;
    println!("{}", reference_name);
    vector_demo();
    ownership_demo();
    indirect_access();
    external_person();
}

fn foobar<T>(_arg: T) {
    todo!("It will get implemented")
}

fn external_person() {
    let person: quick_lib_person =
        quick_lib_person::new("Vikas Poddar".to_owned(), 24, person::Gender::Male);
    println!("Hello, Mr. {}", person.get_name());
}

fn get_number_detail_if_let(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

fn print_match(n: Number) {
    //match arm pattern
    match n {
        Number { odd: true, value } => println!("Odd: {}", value),
        Number { odd: false, value } => println!("Even: {}", value),
    }
}

//functions

fn get_person() -> Person {
    let name = String::from("Vikas");
    let age = 24;
    Person { name, age }
}

fn greet() {
    println!("Hi there!");
}

// implicit return: omitting semicolon at the end

fn answer_of_life() -> i32 {
    42
}

fn new_answer_of_life() -> i32 {
    42
}

fn biased_dice() -> i32 {
    if answer_of_life() == 42 {
        132
    } else {
        42
    }
}

fn non_random_roll() -> i32 {
    match new_answer_of_life() == 42 {
        true => 5,
        false => 10,
    }
}
