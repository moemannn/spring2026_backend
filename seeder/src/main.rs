use dotenvy::dotenv;
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::{FirstName, LastName};
use fake::Fake;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::sea_query::prelude::{Utc};
use uuid::Uuid;

use spring2026::{entity::users, db::connect_db};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_db().await;

    for _ in 0..20 {
        let user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            first_name: Set(FirstName().fake()),
            last_name: Set(LastName().fake()),
            email: Set(SafeEmail().fake()),
            password: Set("default".to_string()),

            admin: Set(false),
            created_at: Set(Utc::now().naive_utc()),
            changed_at: Set(Option::from(Utc::now().naive_utc())),
            deleted_at: Set(None),
        };

        user.insert(&db).await.unwrap();
    }

    println!("Seed complete");
}