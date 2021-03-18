use chrono::{DateTime, Utc};
use hc_time_index::IndexableEntry;
use hdk3::prelude::*;

use crate::LinkExpression;

impl IndexableEntry for LinkExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp.to_owned()
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}
