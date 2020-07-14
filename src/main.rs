#![feature(proc_macro_hygiene, decl_macro)]
use std::process::Command;
use std::env;
use dotenv;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    dotenv::dotenv().ok();
    let command;
    match env::var("UPDATE_ACTIVITY_COMMAND") {
        Ok(val) => command = val,
        Err(_e) => command = "none".to_string(),
    }
    command
}

#[post("/exec/crawler")]
fn exec_crawler() {
    Command::new("ls")
        .arg("-l")
        .arg("-a")
        .spawn()
        .expect("ls command failed to start");
}

fn main() {
    rocket::ignite().mount("/", routes![index, exec_crawler]).launch();
}