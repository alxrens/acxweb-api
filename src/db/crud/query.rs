use sea_orm::{DatabaseConnection, EntityTrait, DbErr, PaginatorTrait, QueryOrder, Statement, DbBackend};

use crate::db::entities::{post, prelude::Post};

pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DatabaseConnection, id: i32) -> Result<post::Model, DbErr> {
        let post: Option<post::Model> = Post::find()
            .from_raw_sql(
                Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"SELECT * FROM post WHERE id = $1"#,
                    vec![id.into()],
                ),
            )
            .one(db)
            .await?;
    
        Ok(post.ok_or(DbErr::RecordNotFound(String::from("No post found with the given ID")))?)
    }
    
    pub async fn find_post_in_page(db : &DatabaseConnection, page : u64, posts_per_page : u64) -> Result<post::ListModel, DbErr> {
        let paginator = post::Entity::find()
            .order_by_asc(post::Column::Id)
            .paginate(db, posts_per_page);
        let total_count = paginator.num_pages().await?;

        paginator
            .fetch_page(page -1)
            .await
            .map(|posts| post::ListModel { total_count, posts })
        
    }
}