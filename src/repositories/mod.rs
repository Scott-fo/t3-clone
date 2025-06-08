pub mod session;

use anyhow::Result;
use diesel::prelude::*;

pub trait Repository<T, C>
where
    T: Clone + Send + Sync,
    C: AsChangeset,
{
    fn find_by_id(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<T>>;

    fn find_by_id_for_update(&self, conn: &mut MysqlConnection, id: &str) -> Result<Option<T>>;

    fn find_by_user(&self, conn: &mut MysqlConnection, user_id: &str) -> Result<Vec<T>>;

    fn create(&self, conn: &mut MysqlConnection, entity: &T) -> Result<T>;

    fn update(&self, conn: &mut MysqlConnection, id: &str, changeset: C) -> Result<T>;

    fn delete(&self, conn: &mut MysqlConnection, id: &str) -> Result<()>;
}
