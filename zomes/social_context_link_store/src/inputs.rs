use hdk3::prelude::*;

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetLinks {
    pub subject: String,
    pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct Triple {
    pub subject: Option<String>,
    pub object: Option<String>,
    pub predicate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct TripleWithAuthor {
    pub subject: Option<String>,
    pub object: Option<String>,
    pub predicate: Option<String>,
    pub author: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct TripleParsed {
    pub subject: AcaiUrl,
    pub object: AcaiUrl,
    pub predicate: Option<AcaiUrl>,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct AcaiUrl {
    pub language: String,
    pub expression: String,
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct Subject {
    pub subject: String,
}

impl std::fmt::Display for AcaiUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}", self.language, self.expression)
    }
}

impl TryFrom<String> for AcaiUrl {
    type Error = HdkError;

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
            Err(HdkError::Wasm(WasmError::Zome(String::from(
                "Expected maximum 3 & minimum 2 segments for subject url in form lang://expression",
            ))))
        }
    }
}

fn extract_acai_url(
    val: Option<String>,
    require: bool,
    property: &str,
) -> ExternResult<Option<AcaiUrl>> {
    if let Some(inner_val) = val {
        let split: Vec<&str> = inner_val.split("://").into_iter().collect();
        if split.len() == 2 {
            Ok(Some(AcaiUrl {
                language: split[0].to_owned(),
                expression: split[1].to_owned(),
            }))
        } else {
            Err(HdkError::Wasm(WasmError::Zome(String::from(
                "Expected two segments for subject url in form lang://expression",
            ))))
        }
    } else {
        if require == false {
            Ok(None)
        } else {
            Err(HdkError::Wasm(WasmError::Zome(format!(
                "Missing a {}",
                property
            ))))
        }
    }
}

impl TryFrom<Triple> for TripleParsed {
    type Error = HdkError;

    fn try_from(value: Triple) -> Result<Self, Self::Error> {
        Ok(TripleParsed {
            subject: extract_acai_url(value.subject, true, "subject")?.unwrap(),
            object: extract_acai_url(value.object, true, "object")?.unwrap(),
            predicate: extract_acai_url(value.predicate, false, "predicate")?,
        })
    }
}
