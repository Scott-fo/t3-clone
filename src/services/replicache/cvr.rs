use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvrDiff {
    pub puts: Vec<String>,
    pub dels: Vec<String>,
    pub changed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvrRecord {
    #[serde(rename = "entities")]
    pub entities: HashMap<String, i32>,
    #[serde(rename = "lastMutationIDs")]
    pub last_mutation_ids: HashMap<String, i32>,
}

impl CvrRecord {
    pub fn new(entities: HashMap<String, i32>, last_mutation_ids: HashMap<String, i32>) -> Self {
        Self {
            entities,
            last_mutation_ids,
        }
    }

    pub fn empty() -> Self {
        Self::new(HashMap::new(), HashMap::new())
    }

    // TODO crap. fix this later
    pub fn from_hash(hash: Option<serde_json::Value>) -> Self {
        match hash {
            None => Self::empty(),
            Some(value) => {
                let entities = value
                    .get("entities")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .filter_map(|(k, v)| v.as_i64().map(|v| (k.clone(), v as i32)))
                            .collect()
                    })
                    .unwrap_or_default();

                let last_mutation_ids = value
                    .get("lastMutationIDs")
                    .and_then(|v| v.as_object())
                    .map(|obj| {
                        obj.iter()
                            .filter_map(|(k, v)| v.as_i64().map(|v| (k.clone(), v as i32)))
                            .collect()
                    })
                    .unwrap_or_default();

                Self::new(entities, last_mutation_ids)
            }
        }
    }

    pub fn to_hash(&self) -> serde_json::Value {
        serde_json::json!({
            "entities": self.entities,
            "lastMutationIDs": self.last_mutation_ids,
        })
    }

    pub fn diff(&self, other: &CvrRecord) -> CvrDiff {
        let self_entities = &self.entities;
        let other_entities = &other.entities;

        let self_keys: HashSet<_> = self_entities.keys().cloned().collect();
        let other_keys: HashSet<_> = other_entities.keys().cloned().collect();

        let puts: Vec<String> = self_keys
            .difference(&other_keys)
            .into_iter()
            .cloned()
            .collect();

        let dels: Vec<String> = other_keys
            .difference(&self_keys)
            .into_iter()
            .cloned()
            .collect();

        let changed: Vec<String> = self_keys
            .intersection(&other_keys)
            .cloned()
            .filter(|k| self_entities.get(k) != other_entities.get(k))
            .collect();

        CvrDiff {
            puts,
            dels,
            changed,
        }
    }
}
