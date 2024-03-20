#![allow(dead_code)]
#[derive(Debug)]


enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress (char),
    Paste(String),
    Click{x: i64, y: i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("PageLoad Event"),
        WebEvent::PageUnload => println!("PageUnload Event"),
        WebEvent::KeyPress(c) => println!("Key Pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted String: {}", s),
        WebEvent::Click {x, y} => {
            println!("clicked on: X: {}, Y: {}", x, y);
        },
    }
}

fn main() {

    type W = WebEvent;

    let pressed = W::KeyPress('x');
    let pasted = W::Paste(String::from("Hello World"));

    inspect(pressed);
    inspect(pasted);
}

