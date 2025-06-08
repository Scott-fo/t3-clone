use std::collections::HashMap;

pub trait ReplicachePullModel: serde::Serialize {
    fn resource_prefix() -> &'static str;

    fn get_id(&self) -> &str;

    fn get_version(&self) -> i32;

    fn into_replicache(list: Vec<Self>) -> HashMap<String, i32>
    where
        Self: Sized,
    {
        list.into_iter()
            .map(|item| {
                (
                    format!("{}/{}", Self::resource_prefix(), item.get_id()),
                    item.get_version(),
                )
            })
            .collect()
    }
}
