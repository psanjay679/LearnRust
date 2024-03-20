#![allow(dead_code)]

struct Person {
    name: String,
    age: u8
}

struct Unit;
struct Pair(i32, i32);

struct Point {
    x: i32,
    y: i32
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn main() {
    
    let top_left: Point = Point {x: 1, y: 5};
    let bottom_right: Point = Point {x: 5, y: 6};

    let rect: Rectangle = Rectangle {top_left, bottom_right};

    let name = String::from("Sanjay");
    let age = 28;

    let sanjay: Person = Person{ name, age };

    println!("Name: {}\nAge: {}", sanjay.name, sanjay.age);

}