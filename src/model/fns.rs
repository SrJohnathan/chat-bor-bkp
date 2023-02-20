use super::{db::PgAsyncConnection, models::{new_users, User}};
use crate::schema::users;
use tokio_diesel::*;

pub async fn create(refc: &PgAsyncConnection, value: new_users) -> Result<bool, String> {
  let va =  diesel::insert_into(users::table)
        .values(value)
        .execute_async(refc)
        .await;
        match va {
            Ok(x) => if x == 1 { Ok( true ) } else {  Ok(false ) },
            Err(e) => Err(e.to_string())
        }
}
pub async fn list(refc: &PgAsyncConnection) -> Result<Vec<User>, String> {
    let va =  users::dsl::users
          .get_results_async(refc)
          .await;
          match  va {
            Ok(x) => {
                Ok(x)
            }
            Err(e) => {Err(e.to_string())}
        }
  }

  pub async fn get_user(refc: &PgAsyncConnection) -> Result<User, String> {
    let va =  users::dsl::users
          .get_result_async(refc)
          .await;
          match  va {
            Ok(x) => {
                Ok(x)
            }
            Err(e) => {Err(e.to_string())}
        }
  }
