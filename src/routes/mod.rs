use rocket::{routes, Route};

mod posts;

pub fn routes() -> Vec<Route> {
    routes![posts::new, posts::get]
}
