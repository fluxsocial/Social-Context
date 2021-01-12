use hdk3::prelude::*;
use holo_hash::hash_type::Dna;
use holo_hash::HoloHash;

use meta_traits::{GlobalEntryRef, Identity};

/// Here we need some wrapper structs around the core types we are trying to return as TryFrom SerializedBytes is not implemented on std types such as Vec or bool

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct BoolOutput(pub bool);

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct EntryRefListOut(pub Vec<GlobalEntryRef>);

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct DnaListOutput(pub Vec<HoloHash<Dna>>);

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct IdentityListOutput(pub Option<Vec<Identity>>);
