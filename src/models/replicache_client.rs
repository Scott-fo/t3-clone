use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::models::replicache_client_group::ReplicacheClientGroup;

#[derive(Queryable, Insertable, Associations, PartialEq, Debug, Identifiable)]
#[diesel(belongs_to(ReplicacheClientGroup, foreign_key = client_group_id))]
#[diesel(table_name = crate::schema::replicache_clients)]
pub struct ReplicacheClient {
    pub id: String,
    pub client_group_id: String,
    pub last_mutation_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ReplicacheClient {
    pub fn new(id: String, client_group_id: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id,
            client_group_id,
            last_mutation_id: 0,
            created_at: now,
            updated_at: now,
        }
    }
}
