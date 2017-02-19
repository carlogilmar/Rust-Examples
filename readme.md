# My First Steps on Rust Lang

Installation on Ubuntu

1. Apply the curl command on Rust Web
2. Check that you have a .cargo dir on your home
3. Add this on your configuration shell .zshrc / .bashrc

> source $HOME/.cargo/env

For run a simple program with Rust

> rustc hola_mundo.rs

> ./hola_mundo

For create a project with Cargo

> cargo new hello_world --bin

> .

> ├── Cargo.toml

> └── src

>     └── main.rs

And for run your new project

> cargo run

Cargo Web Project with Iron

First you have to add on your Cargo.toml:

> [dependencies]

> iron = "0.4.*"

Replace your main.rs with this:

> extern crate iron;

> use iron::prelude::*;

> use iron::status;

> fn main() {

>     Iron::new(|_: &mut Request| {

>         Ok(Response::with((status::Ok, "Hello world!")))

>     }).http("localhost:3000").unwrap();

> }

Then Build, and Run with Cargo, and check your localhost:3000
