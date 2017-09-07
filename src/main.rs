#[macro_use] extern crate nickel;
extern crate hyper;


use hyper::method::Method;
use nickel::{Nickel, StaticFilesHandler, HttpRouter};
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

mod Emulator;
mod Translation_Cache;
mod Emitter;
mod Interpreter;

fn main() {

    let Emulator = Arc::new(RwLock::new(Emulator::Emulator::new()));
    let mut server = Nickel::new();

    let EmulatorRom = Emulator.clone();
    server.add_route(Method::Post, "/sendRom", middleware!{|req|
        let mut Emulator = EmulatorRom.write().unwrap();
        req.origin.read_to_end(&mut Emulator.ROM).unwrap();
    });

    let mut EmulatorBlock = Emulator.clone();
    server.add_route(Method::Get, "/runBlock", middleware!{
        let mut Emulator = EmulatorBlock.write().unwrap();
        Emulator.runBlock();
    });

    server.utilize(StaticFilesHandler::new("./Client"));
    server.listen("10.0.0.4:8080").unwrap();

}
