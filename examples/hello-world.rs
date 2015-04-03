#![feature(io)]
#![feature(core)]
#![feature(old_io)]

extern crate rustbox;

use std::char;
use std::old_io::stdio;
use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox, InitOptions};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(InitOptions {
        buffer_stderr: stdio::stderr_raw().isatty(),
        ..Default::default()
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        rustbox.present();
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    _ => { }
                }
            },
            Err(s) => match s {
                //Some(e) => panic!("{}", e.description()),
                Some(e) => panic!("I can't even print the error"),
                None => { }
            },
            _ => { }
        }
    }
}
