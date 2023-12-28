use actix_web::{get, web, HttpResponse};
use serde::{self, Serialize};
use serde_json::to_string_pretty;

use std::env;

use crate::errors::CustomError;

mod posts;