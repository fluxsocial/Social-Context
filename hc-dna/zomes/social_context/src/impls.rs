use chrono::{DateTime, Utc};
use hc_time_index::IndexableEntry;
use hdk::prelude::*;

use crate::{AgentReference, LinkExpression, IndexStrategy, errors::SocialContextError};

impl IndexableEntry for LinkExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp.to_owned()
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}

impl IndexableEntry for AgentReference {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp.to_owned()
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}

impl LinkExpression {
    pub fn get_sb(self) -> ExternResult<SerializedBytes> {
        Ok(self.try_into()?)
    }
}

impl TryFrom<String> for IndexStrategy {
    type Error = SocialContextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "Full" => Ok(IndexStrategy::Full),
            "full" => Ok(IndexStrategy::Full),
            "Simple" => Ok(IndexStrategy::Simple),
            "simple" => Ok(IndexStrategy::Simple),
            "FullWithWildCard" => Ok(IndexStrategy::FullWithWildCard),
            "fullwithwildcard" => Ok(IndexStrategy::FullWithWildCard),
            _ => Err(SocialContextError::InternalError("could not convert string to IndexStrategy enum"))
        }    
    }
}