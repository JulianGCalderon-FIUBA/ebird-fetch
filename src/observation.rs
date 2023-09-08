use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub com_name: String,
    pub sci_name: String,
    pub loc_name: String,
    pub how_many: usize,
}
