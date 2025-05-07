use UIEvent::*;

#[derive(Debug)]
enum Direction {
    Up, Down
}
#[derive(Debug)]
enum UIEvent {
    ButtonClicked,
    Scroll(Direction),
    KeyPressed(char)
}

impl UIEvent {
    fn describe(&self) {
        println!("{:?}", self);
    }
}

fn call(event : UIEvent) {
    match event {
        ButtonClicked => println!("Button clicked"), // simple match
        Scroll(x) => println!("Scroll {:?}", x), // attribute extraction
        KeyPressed(ch) => { // whole block
            let up_ch = ch.to_uppercase();
            println!("Key pressed: {}", up_ch);
        }
    }
}

fn main() {
    let clicked = ButtonClicked;
    let scroll = Scroll(Direction::Down);
    let key_pressed = KeyPressed('b');
    call(clicked);
    call(scroll);
    call(key_pressed);
}