#![allow(dead_code)]

use std::cmp::max;


//structs
struct Person {
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

struct Point<T> {
    x: T,
    y: T,
}


// structs are init by struct literals


struct Number {
    odd: bool,
    value: i32,
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
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
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
    println!("The length of the word '{}': {}", long_word, long_word.len());
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
    let person1 = Person{name: "Happy".to_owned(), age: 18};
    let person2 = Person{name: "Amar".to_owned(), age: 30};
    // destructing structs
    let Person {name, age} = person1;
    println!("{} {}", name, age);
    println!("{}", person2.age());
    println!("{}", person2.is_legal_age());
    // primitive types and some non-primitive types are scoped by default

    // patterns and destructuring - if let
    let one = Number{odd: true, value: 1};
    let two = Number{odd: false, value: 2};
    let _three = Number{odd: true, ..two};
    let _two = Number{..two};
    get_number_detail_if_let(one);
    get_number_detail_if_let(two);
    let one = Number{odd: true, value: 1};
    let two = Number{odd: false, value: 2};
    print_match(one);
    print_match(two);

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
    // 
}

fn get_number_detail_if_let(n: Number) {
    if let Number{odd: true, value} = n {
        println!("Odd number: {}", value);
    }else if let Number{odd: false, value} = n {
        println!("Even number: {}", value);
    }
}

fn print_match(n: Number) {
    //match arm pattern
    match n {
        Number{odd: true, value} => println!("Odd: {}", value),
        Number{odd: false, value} => println!("Even: {}", value),
    }
}

//functions

fn get_person() -> Person {
    let name = String::from("Vikas");
    let age = 24;
    let person = Person { name, age };
    person
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