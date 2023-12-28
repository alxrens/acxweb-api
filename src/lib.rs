#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use sea_orm::DatabaseConnection;

use std::env;

pub mod config;
mod errors;
pub mod db;
pub mod routes;