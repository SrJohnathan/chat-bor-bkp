

use std::ops::Deref;


use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, PooledConnection};
//use diesel_migrations::embed_migrations;
pub use diesel::r2d2::Pool;
use dotenvy::dotenv;


#[warn(dead_code)]
pub type  PgAsyncConnection =  Pool<ConnectionManager<PgConnection>>;
pub struct   PoolPgAsyncConnection( pub PooledConnection<ConnectionManager<PgConnection>>);


impl Deref for PoolPgAsyncConnection  {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
impl Debug for PgAsyncConnection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for PgAsyncConnection {
    fn default() -> Self {
        connection("GRPC".to_string()).unwrap()
    }
}


*/
#[warn(dead_code)]
pub async  fn connection(str: String) -> Result<PgAsyncConnection,String> {
    dotenv().unwrap();
    let db_url = std::env::var(str).unwrap();
    println!("{}",db_url);
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager).unwrap();
   // embed_migrations!();
    Ok(pool)
}