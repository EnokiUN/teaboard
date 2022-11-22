use rocket::{routes, Route};

mod boards;
mod fetch_image;
mod posts;

pub fn routes() -> Vec<Route> {
    routes![
        posts::new,
        posts::get,
        boards::info,
        boards::feed,
        fetch_image::fetch_image
    ]
}
