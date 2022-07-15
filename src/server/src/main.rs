use rocket::fs::{relative, FileServer};
use rocket::response::Redirect;
use std::env::current_dir;

#[macro_use]
extern crate rocket;

#[get("/login")]
fn pseudo_login() -> Redirect {
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    let mut cwd = current_dir().unwrap();
    cwd.pop();
    cwd.pop();

    rocket::build()
        .mount("/", routes![pseudo_login])
        .mount("/", FileServer::from(format!("{}/public", cwd.display())))
}
