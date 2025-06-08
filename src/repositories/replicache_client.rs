use anyhow::{Context, Result, bail};
use chrono::Utc;
use diesel::prelude::*;

use crate::models::replicache_client::ReplicacheClient;

#[derive(Debug, Clone)]
pub struct ReplicacheClientRepository;

impl ReplicacheClientRepository {
    pub fn find(
        &self,
        conn: &mut MysqlConnection,
        client_id: &str,
    ) -> Result<Option<ReplicacheClient>> {
        use crate::schema::replicache_clients::dsl::*;

        replicache_clients
            .find(client_id)
            .first::<ReplicacheClient>(conn)
            .optional()
            .context(format!(
                "Database error while finding replicache client {}",
                client_id
            ))
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        client_id: &str,
        client_group_id_param: &str,
    ) -> Result<ReplicacheClient> {
        use crate::schema::replicache_clients::dsl::*;

        let new_client =
            ReplicacheClient::new(client_id.to_string(), client_group_id_param.to_string());

        diesel::insert_into(replicache_clients)
            .values(&new_client)
            .execute(conn)
            .context(format!("Failed to create replicache client {}", client_id))?;

        Ok(new_client)
    }

    pub fn find_or_create(
        &self,
        conn: &mut MysqlConnection,
        client_id: &str,
        client_group_id: &str,
    ) -> Result<ReplicacheClient> {
        conn.transaction(|conn| {
            if let Some(client) = self.find(conn, client_id)? {
                if client.client_group_id != client_group_id {
                    bail!(
                        "Client {} already exists but belongs to a different group.",
                        client_id
                    );
                }
                Ok(client)
            } else {
                self.create(conn, client_id, client_group_id)
            }
        })
    }

    pub fn update_last_mutation_id(
        &self,
        conn: &mut MysqlConnection,
        client_id: &str,
        new_mutation_id: i32,
    ) -> Result<()> {
        use crate::schema::replicache_clients::dsl::{
            last_mutation_id, replicache_clients, updated_at,
        };

        let rows_affected = diesel::update(replicache_clients.find(client_id))
            .set((
                last_mutation_id.eq(new_mutation_id),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)
            .context(format!(
                "Failed to update last_mutation_id for client {}",
                client_id
            ))?;

        if rows_affected == 0 {
            bail!("Client {} not found for update.", client_id);
        }

        Ok(())
    }
}
