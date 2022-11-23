use rocket::{routes, Route};

mod boards;
mod fetch_image;
mod index;
mod posts;

pub fn routes() -> Vec<Route> {
    routes![
        index::index,
        posts::new,
        posts::get,
        boards::info,
        boards::feed,
        fetch_image::fetch_image
    ]
}
