use anyhow::{Context, Result};
use diesel::prelude::*;
use std::collections::HashMap;

use super::{cvr::CvrRecord, entity_registry::EntityRegistry, types::PatchOperation};

pub struct PatchGenerator {
    entity_registry: EntityRegistry,
}

impl PatchGenerator {
    pub fn new() -> Self {
        Self {
            entity_registry: EntityRegistry::new(),
        }
    }

    pub fn generate_patch(
        &self,
        base_cvr: &CvrRecord,
        next_cvr: &CvrRecord,
        conn: &mut MysqlConnection,
    ) -> Result<Vec<PatchOperation>> {
        tracing::info!("Generating patch");
        let mut patch = Vec::new();

        if base_cvr.entities.is_empty() {
            tracing::debug!("Empty base CVR, adding clear operation");
            patch.push(PatchOperation::Clear);
        }

        let diff = next_cvr.diff(base_cvr);
        tracing::debug!("Calculated diff: {:?}", diff);

        patch.extend(
            diff.dels
                .into_iter()
                .map(|key| PatchOperation::Delete { key }),
        );

        let combined_keys: Vec<&String> = diff.puts.iter().chain(diff.changed.iter()).collect();

        let groups: HashMap<&str, Vec<&str>> = combined_keys
            .iter()
            .filter_map(|&key| key.split_once('/'))
            .fold(HashMap::new(), |mut acc, (entity_type, obj_id)| {
                acc.entry(entity_type).or_default().push(obj_id);
                acc
            });

        let query_map: Result<HashMap<&str, HashMap<String, serde_json::Value>>> = groups
            .iter()
            .map(|(&entity_type, ids)| {
                let patch_fn = self
                    .entity_registry
                    .get_patch_fn(entity_type)
                    .with_context(|| format!("Unknown entity type: {}", entity_type))?;

                let data = patch_fn(conn, ids)?;
                Ok((entity_type, data))
            })
            .collect();

        let query_map = query_map?;

        let put_operations: Result<Vec<PatchOperation>> = combined_keys
            .iter()
            .map(|&key| {
                let (entity_type, obj_id) =
                    key.split_once('/').expect("Already filtered invalid keys");

                let entity_map = query_map
                    .get(entity_type)
                    .with_context(|| format!("Entity map for '{}' should exist", entity_type))?;

                let value = entity_map
                    .get(obj_id)
                    .with_context(|| format!("'{}' not found for id: {}", entity_type, obj_id))?;

                Ok(PatchOperation::Put {
                    key: key.clone(),
                    value: value.clone(),
                })
            })
            .collect();

        patch.extend(put_operations?);

        tracing::debug!("Generated patch with {} operations", patch.len());
        Ok(patch)
    }
}
