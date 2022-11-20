use rocket::{routes, Route};

mod boards;
mod posts;

pub fn routes() -> Vec<Route> {
    routes![posts::new, posts::get, boards::info, boards::feed]
}
