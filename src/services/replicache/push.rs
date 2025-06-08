use anyhow::{Context, Result};
use diesel::prelude::*;

use crate::{
    app::AppState,
    mutations::handler::{RawMutation, parse_mutation},
    repositories::{
        replicache_client::ReplicacheClientRepository,
        replicache_client_group::ReplicacheClientGroupRepository,
    },
};

pub fn process_mutations(
    state: AppState,
    client_group_id: &str,
    current_user_id: &str,
    mutations: &[RawMutation],
) -> Result<()> {
    let mut conn = state.db_pool.get().context("Failed to get DB connection")?;

    for mutation in mutations {
        let result: Result<()> = conn.transaction(|conn| {
            let normal_result = process_single_mutation(
                state.clone(),
                conn,
                mutation,
                client_group_id,
                current_user_id,
                false, // error_mode = false
            );

            if let Err(e) = normal_result {
                tracing::warn!(
                    "Mutation {} failed: {:?}. Retrying in error_mode to advance mutation ID.",
                    mutation.id,
                    e
                );
                process_single_mutation(
                    state.clone(),
                    conn,
                    mutation,
                    client_group_id,
                    current_user_id,
                    true, // error_mode = true
                )?;
            }

            Ok(())
        });

        if let Err(e) = result {
            tracing::error!(
                mutation_id = mutation.id,
                "Transaction for mutation failed completely: {:?}",
                e
            );
        }
    }

    Ok(())
}

pub fn process_single_mutation(
    state: AppState,
    conn: &mut MysqlConnection,
    mutation: &RawMutation,
    client_group_id: &str,
    current_user_id: &str,
    error_mode: bool,
) -> Result<()> {
    tracing::info!(
        "Processing mutation {} (error_mode: {})",
        mutation.id,
        error_mode
    );

    ReplicacheClientGroupRepository.find_or_create(conn, client_group_id, current_user_id)?;
    let client =
        ReplicacheClientRepository.find_or_create(conn, &mutation.client_id, client_group_id)?;

    let next_mutation_id = client.last_mutation_id + 1;

    if mutation.id < next_mutation_id {
        tracing::info!("Mutation {} already processed, skipping", mutation.id);
        return Ok(());
    }

    if mutation.id > next_mutation_id {
        return Err(anyhow::anyhow!(
            "Out-of-order mutation. Expected {}, got {}.",
            next_mutation_id,
            mutation.id
        ));
    }

    if !error_mode {
        tracing::info!("Applying mutation business logic");
        let parsed_mutation =
            parse_mutation(mutation.clone()).context("Failed to parse raw mutation")?;

        parsed_mutation.process(state, conn, current_user_id)?;
    }

    ReplicacheClientRepository.update_last_mutation_id(conn, &client.id, next_mutation_id)?;

    Ok(())
}
