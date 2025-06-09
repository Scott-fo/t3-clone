use anyhow::{Context, Result};
use axum::{Json, http::StatusCode};
use diesel::{Connection, prelude::*};
use std::collections::HashMap;
use tower_sessions_redis_store::fred::prelude::{KeysInterface, Pool};
use tracing::{debug, info};
use uuid::Uuid;

use crate::{
    infra::db::DbPool,
    models::{
        replicache::ReplicachePullModel, replicache_client::ReplicacheClient,
        replicache_client_group::ReplicacheClientGroup,
    },
    repositories::replicache_client_group::ReplicacheClientGroupRepository,
    services::{
        container::ServiceContainer,
        replicache::{
            patch_generator::PatchGenerator,
            types::{Cookie, PullResponse, PullResult},
        },
    },
};

use super::cvr::CvrRecord;

pub fn map_anyhow_error_to_response(err: anyhow::Error) -> (StatusCode, String) {
    tracing::error!("Request failed: {:?}", err);

    if err.to_string().contains("Unauthorized") {
        (StatusCode::UNAUTHORIZED, err.to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal server error occurred".to_string(),
        )
    }
}

pub async fn retrieve_base_cvr(cookie: &Option<Cookie>, pool: Pool) -> Result<CvrRecord> {
    let cookie = match cookie {
        Some(c) => c,
        None => {
            info!("No cookie in request, using empty CVR");
            return Ok(CvrRecord::empty());
        }
    };

    let cache_key = format!("cvr/{}", cookie.cvr_id);
    info!(cache_key, "Attempting to read from cache");

    let cached_data: Option<String> = pool
        .get(&cache_key)
        .await
        .context("Failed to retrieve from Redis cache")?;

    match cached_data {
        Some(data) => {
            debug!(data_length = data.len(), "Retrieved data from cache");
            let json_data =
                serde_json::from_str(&data).context("Failed to parse cached CVR data")?;
            Ok(CvrRecord::from_hash(Some(json_data)))
        }
        None => {
            info!(cache_key, "No data found in cache, using empty CVR");
            Ok(CvrRecord::empty())
        }
    }
}

pub fn process_pull_request(
    pool: &DbPool,
    base_cvr: CvrRecord,
    user_id: &str,
    client_group_id: &str,
    cookie_order: i32,
    services: &ServiceContainer,
) -> Result<PullResult> {
    let mut conn = pool
        .get()
        .context("Failed to get DB connection from pool")?;

    conn.transaction(|conn| {
        let group =
            ReplicacheClientGroupRepository.find_or_create(conn, client_group_id, user_id)?;

        let next_cvr = build_next_cvr(conn, &group, user_id.to_string(), services)?;

        if next_cvr.to_hash() == base_cvr.to_hash() {
            info!("CVR unchanged, returning early");
            return Ok(PullResult::Unchanged {
                cvr_version: group.cvr_version,
            });
        }

        let patch_generator = PatchGenerator::new();

        let patch = patch_generator.generate_patch(&base_cvr, &next_cvr, conn)?;

        let new_version = std::cmp::max(cookie_order, group.cvr_version) + 1;
        ReplicacheClientGroupRepository.update_cvr_version(conn, &group.id, new_version)?;

        Ok(PullResult::Changed {
            next_cvr,
            patch,
            cvr_version: new_version,
        })
    })
}

pub async fn build_response(
    pull_result: PullResult,
    cookie: Option<Cookie>,
    pool: Pool,
) -> Result<Json<PullResponse>> {
    match pull_result {
        PullResult::Unchanged { cvr_version } => {
            let cookie = cookie.unwrap_or_else(|| Cookie {
                order: cvr_version,
                cvr_id: Uuid::new_v4().to_string(),
            });

            info!("CVR unchanged, returning empty patch");
            Ok(Json(PullResponse {
                cookie,
                last_mutation_id_changes: HashMap::new(),
                patch: vec![],
            }))
        }
        PullResult::Changed {
            next_cvr,
            patch,
            cvr_version,
        } => {
            let cvr_id = Uuid::new_v4().to_string();
            let cache_key = format!("cvr/{}", cvr_id);

            let json_data = serde_json::to_string(&next_cvr.to_hash())?;

            let _: () = pool
                .set(&cache_key, &json_data, None, None, false)
                .await
                .context("Failed to cache new CVR")?;

            Ok(Json(PullResponse {
                patch,
                cookie: Cookie {
                    order: cvr_version,
                    cvr_id,
                },
                last_mutation_id_changes: next_cvr.last_mutation_ids,
            }))
        }
    }
}

fn build_next_cvr(
    conn: &mut MysqlConnection,
    client_group: &ReplicacheClientGroup,
    current_user_id: String,
    services: &ServiceContainer,
) -> Result<CvrRecord> {
    info!("Building next CVR for client group: {}", client_group.id);

    let entities_map = collect_all_entities(conn, &current_user_id, services)?;

    let clients: Vec<ReplicacheClient> = ReplicacheClient::belonging_to(client_group).load(conn)?;

    let last_mutation_ids: HashMap<String, i32> = clients
        .into_iter()
        .map(|client| (client.id, client.last_mutation_id))
        .collect();

    debug!(
        entity_count = entities_map.len(),
        client_count = last_mutation_ids.len(),
        "Built next CVR"
    );

    Ok(CvrRecord::new(entities_map, last_mutation_ids))
}

fn collect_all_entities(
    conn: &mut MysqlConnection,
    user_id: &str,
    services: &ServiceContainer,
) -> Result<HashMap<String, i32>> {
    let mut map = HashMap::new();

    let chats_with_messages = services
        .chat_service
        .list_with_messages_for_user(conn, user_id)?;

    let mut chats = Vec::new();
    let mut messages = Vec::new();

    for cwm in chats_with_messages {
        let chat = crate::models::chat::Chat {
            id: cwm.id,
            user_id: cwm.user_id,
            title: cwm.title,
            pinned: cwm.pinned,
            archived: cwm.archived,
            version: cwm.version,
            created_at: cwm.created_at,
            updated_at: cwm.updated_at,
        };
        chats.push(chat);
        messages.extend(cwm.messages);
    }

    collect_entity_type(&mut map, chats);
    collect_entity_type(&mut map, messages);

    Ok(map)
}

fn collect_entity_type<T>(map: &mut HashMap<String, i32>, entities: Vec<T>)
where
    T: ReplicachePullModel,
{
    map.extend(T::into_replicache(entities));
}
