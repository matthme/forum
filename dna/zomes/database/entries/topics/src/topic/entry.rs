use hdk::prelude::*;





#[hdk_entry(id = "topic")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Topic {
  pub title: String,
  pub content: String,
}