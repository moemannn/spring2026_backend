use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db = users::database::connect().await.unwrap();

    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        first_name: Set("John".to_string()),
        last_name: Set("Doe".to_string()),
        email: Set("john@test.com".to_string()),
        password: Set("default".to_string()),
    };

    user.insert(&db).await.unwrap();

    println!("Seed complete");
}