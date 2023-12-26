use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigrationTrait;

use std::env;

mod entities;
mod migrator;
mod crud;

use migrator::Migrator;