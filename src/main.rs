extern crate rustbox;

use std::error::Error;
use std::default::Default;
use std::time::Duration;

use rustbox::{Color, RustBox};
use rustbox::Key;

pub struct Cursor {
    x: usize,
    y: usize
}

impl Cursor {
    fn new(x: usize, y: usize) -> Self {
        Cursor { x: x, y: y }
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut cursor = Cursor::new(0, 0);

    loop {
        match rustbox.peek_event(Duration::from_secs(0), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc => { break; }
                    Key::Backspace => {
                        if cursor.x > 0 {
                            cursor.x -= 1;
                        }
                        rustbox.print(cursor.x, cursor.y, rustbox::RB_NORMAL, Color::Black, Color::Default, " ");
                    }
                    Key::Char(c) => {
                        let s = &c.to_string();
                        rustbox.print(cursor.x, cursor.y, rustbox::RB_BOLD, Color::Black, Color::Red, s);

                        if cursor.x < (rustbox.width() - 1) {
                            cursor.x += 1;
                        }
                    }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }

        rustbox.present();
    }
}
