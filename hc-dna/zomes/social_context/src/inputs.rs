use chrono::{DateTime, Utc};
use hdk::prelude::*;

#[hdk_entry(id = "did_agent", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Agent {
    pub did: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

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

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UriTag(pub String);

impl Triple {
    pub fn num_entities(&self) -> usize {
        let mut num = 0;
        if self.source.is_some() {
            num += 1;
        };
        if self.target.is_some() {
            num += 1;
        };
        if self.predicate.is_some() {
            num += 1;
        };

        num
    }
}
