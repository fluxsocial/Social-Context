use chrono::{DateTime, Utc};
use hc_time_index::IndexableEntry;
use hdk::prelude::*;

use crate::{AgentReference, LinkExpression};

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
