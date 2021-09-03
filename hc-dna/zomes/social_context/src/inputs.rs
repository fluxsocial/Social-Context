use chrono::{DateTime, Utc};
use hdk::prelude::*;
use crate::LinkExpression;

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct ExpressionProof {
    pub signature: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct Triple {
    pub source: Option<String>,
    pub target: Option<String>,
    pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetLinks {
    #[serde(flatten)]
    pub triple: Triple,
    #[serde(rename(serialize = "fromDate", deserialize = "fromDate"))]
    pub from_date: Option<DateTime<Utc>>,
    #[serde(rename(serialize = "untilDate", deserialize = "untilDate"))]
    pub until_date: Option<DateTime<Utc>>,
    pub limit: usize
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum IndexStrategy {
    FullWithWildCard,
    Full,
    Simple,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddLinkInput {
    pub link_expression: LinkExpression,
    pub index_strategy: IndexStrategy,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UpdateLinkInput {
    pub source: LinkExpression,
    pub target: LinkExpression,
    pub index_strategy: IndexStrategy,
}
