use hdk3::prelude::*;
use holo_hash::DnaHash;

use meta_traits::Identity;

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct DnaAddress {
    pub dna_address: DnaHash,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct PaginationArguments {
    pub count: usize,
    pub page: usize,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes, Debug)]
pub struct ReadCommunicationArguments {
    pub by_dna: Option<DnaHash>,
    pub by_agent: Option<Identity>,
    pub count: usize,
    pub page: usize,
}
