use sea_orm::entity::prelude::*;
use sea_orm_migration::async_trait::async_trait;
use serde::{Serialize, Deserialize};

use crate::errors::ErrorResponse;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub text: String,
    pub created_at : DateTime,
    pub updated_at : DateTime,
}

#[derive(DeriveIntoActiveModel, Serialize, Deserialize)]
pub struct NewModel {
    pub title: String,
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListModel {
    pub total_count: u64,
    pub posts: Vec<Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


fn check_title(title : &str) -> Result<(), &str>{
    if title.is_empty(){
        return Err("title can't be empty");
    } else if title.len() >256 {
        return Err("title can't be longer than 256 characters");
    } else {
        Ok(())
    }
}

fn check_text(text : &str) -> Result<(), &str> {
    if text.is_empty() {
        return Err("text can't be empty");
    } else if text.len() > 65535 {
        return Err("text can't be longer than 65535 characters");
    } else {
        Ok(())
    }
}


#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C:ConnectionTrait,
    {
        let mut errors: Vec<String> = vec![];

        let title = self.title.as_ref();
        let text = self.text.as_ref();

        
        if let Err(err) = check_title(title) {
            errors.push(ErrorResponse::new(422, err));
        }

        if let Err(err) = check_text(text) {
            errors.push(ErrorResponse::new(422, err))
        }

        if errors.is_empty(){
            Ok(self)
        }else {
            let errors = errors.join(", ");
            Err(DbErr::Custom(format!("[{errors}]")))
        }

    }
}