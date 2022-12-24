use std::convert::Infallible;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

use crate::conf::Conf;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordAuth {
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrictPasswordAuth;

#[derive(Debug)]
pub enum AuthError {
    Unauthorised,
    AuthNotFound,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PasswordAuth {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(password) => Outcome::Success(PasswordAuth {
                admin: password
                    == &req
                        .rocket()
                        .state::<Conf>()
                        .expect("Could not find instance config in rocket state")
                        .password,
            }),
            None => Outcome::Success(PasswordAuth { admin: false }),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StrictPasswordAuth {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(password) => {
                if password
                    == &req
                        .rocket()
                        .state::<Conf>()
                        .expect("Could not find instance config in rocket state")
                        .password
                {
                    Outcome::Success(StrictPasswordAuth)
                } else {
                    Outcome::Failure((Status::Forbidden, AuthError::Unauthorised))
                }
            }
            None => Outcome::Failure((Status::BadRequest, AuthError::AuthNotFound)),
        }
    }
}
