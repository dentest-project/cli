use serde::Deserialize;

#[derive(Deserialize)]
pub struct FeatureListItem {
    pub path: String,
    pub feature: String,
}
