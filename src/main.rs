#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

#[get("/<name>")]
fn hello(name: String) -> String {
    format!("Hola {}!", name)
}

#[get("/")]
fn index() -> &'static str {
    "APP online papirrin! 🚀 <RUST + ROCKET> "
}

#[catch(404)]
fn not_found(req: &rocket::Request<'_>) -> content::Html<String>{
    content::Html(format!("<div>404 Aqui no hay nada papirrin, usshcale</div> {} <div><a href='/pitonisio'<a/>GO HOME</div>", req.uri()))
}

fn main() {
    rocket::ignite().mount("/",routes![hello, index]).register(catchers![not_found]).launch();
}
