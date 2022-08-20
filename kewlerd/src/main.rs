#[macro_use]
extern crate rocket;

mod controller;
mod model;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            controller::git::add,
            controller::git::remove,
            controller::git::update
        ],
    )
}
