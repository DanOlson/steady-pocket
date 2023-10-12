use actix_web::{ResponseError, HttpResponse};
use thiserror::Error;
use derive_more::Display;

#[derive(Error, Display, Debug)]
pub enum Error {
    NotFound,
    DB(String),
    Migration(#[from] sqlx::migrate::MigrateError),
    IO(#[from] std::io::Error),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::DB(format!("Database error {error}"))
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound => HttpResponse::NotFound().finish(),
            Error::DB(ref err) => {
                let body = format!("Error::DB {err}");
                println!("{body}");
                HttpResponse::InternalServerError().body(body)
            },
            Error::IO(ref err) => {
                let body = format!("Error::IO {err}");
                println!("{body}");
                HttpResponse::InternalServerError().body(body)
            },
            Error::Migration(ref err) => {
                let body = format!("Error::Migration {err}");
                println!("{body}");
                HttpResponse::InternalServerError().body(body)
            }
        }
    }
}
