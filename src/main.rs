#[macro_use] extern crate nickel;
extern crate hyper;


use hyper::method::Method;
use nickel::{Nickel, StaticFilesHandler, HttpRouter};
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

mod Emulator;
mod Translation_Cache;

fn main() {

    let mut Emulator = Arc::new(RwLock::new(Emulator::Emulator::new()));
    let mut Shared_Memory
    let mut server = Nickel::new();

    server.add_route(Method::Post, "/sendRom", middleware!{|req|
        let mut Emulator = Emulator.write().unwrap();
        req.origin.read_to_end(&mut Emulator.ROM).unwrap();
    });

    server.add_route(Method::Get, "/runBlock", middleware!{
        Emulator.runBlock();
    });

    server.utilize(StaticFilesHandler::new("./Client"));
    server.listen("10.0.0.4:8080").unwrap();

}
