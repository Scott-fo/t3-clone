use anyhow::{Context, Result, bail};
use diesel::prelude::*;

use crate::models::replicache_client_group::ReplicacheClientGroup;

#[derive(Debug, Clone)]
pub struct ReplicacheClientGroupRepository;

impl ReplicacheClientGroupRepository {
    pub fn find(
        &self,
        conn: &mut MysqlConnection,
        client_group_id: &str,
    ) -> Result<Option<ReplicacheClientGroup>> {
        use crate::schema::replicache_client_groups::dsl::*;

        replicache_client_groups
            .find(client_group_id)
            .first::<ReplicacheClientGroup>(conn)
            .optional()
            .context(format!(
                "Database error while finding replicache client group {}",
                client_group_id
            ))
    }

    pub fn create(
        &self,
        conn: &mut MysqlConnection,
        client_group_id: &str,
        user_id_param: &str,
    ) -> Result<ReplicacheClientGroup> {
        use crate::schema::replicache_client_groups::dsl::*;

        let new_group =
            ReplicacheClientGroup::new(client_group_id.to_string(), user_id_param.to_string());

        diesel::insert_into(replicache_client_groups)
            .values(&new_group)
            .execute(conn)
            .context(format!(
                "Failed to create replicache client group {}",
                client_group_id
            ))?;

        Ok(new_group)
    }

    pub fn find_or_create(
        &self,
        conn: &mut MysqlConnection,
        client_group_id: &str,
        user_id: &str,
    ) -> Result<ReplicacheClientGroup> {
        if let Some(group) = self.find(conn, client_group_id)? {
            if group.user_id != user_id {
                bail!(
                    "Unauthorized: ClientGroup {} does not belong to the specified user.",
                    client_group_id
                );
            }
            Ok(group)
        } else {
            self.create(conn, client_group_id, user_id)
        }
    }

    pub fn update_cvr_version(
        &self,
        conn: &mut MysqlConnection,
        client_group_id: &str,
        new_version: i32,
    ) -> Result<()> {
        use crate::schema::replicache_client_groups::dsl::{cvr_version, replicache_client_groups};

        let rows_affected = diesel::update(replicache_client_groups.find(client_group_id))
            .set(cvr_version.eq(new_version))
            .execute(conn)
            .context(format!(
                "Failed to update cvr_version for group {}",
                client_group_id
            ))?;

        if rows_affected == 0 {
            bail!("ClientGroup {} not found for update.", client_group_id);
        }

        Ok(())
    }
}
