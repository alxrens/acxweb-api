use chrono::Utc;

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::db::crud::query::Query;
use crate::db::entities::{post, prelude::Post};


pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        db: &DatabaseConnection,
        form_data: post::NewModel,
    ) -> Result<post::Model>
}
