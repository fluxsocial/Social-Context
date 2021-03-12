use chrono::{DateTime, Utc};
use hdk3::prelude::*;

#[hdk_entry(id = "acai_agent", visibility = "public")]
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
    #[serde(rename(serialize = "source", deserialize = "source"))]
    pub subject: Option<String>,
    #[serde(rename(serialize = "target", deserialize = "target"))]
    pub object: Option<String>,
    pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetLinks {
    #[serde(flatten)]
    pub triple: Triple,
    pub from: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UriTag(pub String);

impl Triple {
    pub fn num_entities(&self) -> usize {
        let mut num = 0;
        if self.subject.is_some() {
            num += 1;
        };
        if self.object.is_some() {
            num += 1;
        };
        if self.predicate.is_some() {
            num += 1;
        };

        num
    }
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct TripleParsed {
    pub subject: Option<AcaiUrl>,
    pub object: Option<AcaiUrl>,
    pub predicate: Option<AcaiUrl>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct AcaiUrl {
    pub language: String,
    pub expression: String,
}

impl std::fmt::Display for AcaiUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}", self.language, self.expression)
    }
}

impl TryFrom<String> for AcaiUrl {
    type Error = WasmError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split: Vec<&str> = value.split("://").into_iter().collect();
        if split.len() == 2 {
            Ok(AcaiUrl {
                language: split[0].to_owned(),
                expression: split[1].to_owned(),
            })
        } else if split.len() == 3 {
            Ok(AcaiUrl {
                language: split[0].to_owned(),
                expression: format!("{}://{}", split[1], split[2]),
            })
        } else {
            Err(WasmError::Zome(String::from(
                "Expected maximum 3 & minimum 2 segments for subject url in form lang://expression",
            )))
        }
    }
}

fn extract_acai_url(val: Option<String>) -> ExternResult<Option<AcaiUrl>> {
    if let Some(inner_val) = val {
        let split: Vec<&str> = inner_val.split("://").into_iter().collect();
        if split.len() == 2 {
            Ok(Some(AcaiUrl {
                language: split[0].to_owned(),
                expression: split[1].to_owned(),
            }))
        } else {
            Err(WasmError::Zome(String::from(
                "Expected two segments for subject url in form lang://expression",
            )))
        }
    } else {
        Ok(None)
    }
}

impl TryFrom<Triple> for TripleParsed {
    type Error = WasmError;

    fn try_from(value: Triple) -> Result<Self, Self::Error> {
        Ok(TripleParsed {
            subject: extract_acai_url(value.subject)?,
            object: extract_acai_url(value.object)?,
            predicate: extract_acai_url(value.predicate)?,
        })
    }
}
