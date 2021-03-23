use hdk3::prelude::*;
//use holo_hash::error::HoloHashError;
use std::convert::Infallible;

use hc_time_index::errors::IndexError;

#[derive(thiserror::Error, Debug)]
pub enum SocialContextError {
    #[error(transparent)]
    Serialization(#[from] SerializedBytesError),
    #[error(transparent)]
    Infallible(#[from] Infallible),
    #[error(transparent)]
    EntryError(#[from] EntryError),
    #[error("Failed to convert an agent link tag to an agent pub key")]
    AgentTag,
    #[error(transparent)]
    Wasm(#[from] WasmError),
    // #[error(transparent)]
    // HoloHashError(#[from] HoloHashError),
    #[error("Internal Error. Error: {0}")]
    InternalError(&'static str),
    // #[error(transparent)]
    // HdkError(#[from] HdkError),
    #[error("Invalid Request Data. Error: {0}")]
    RequestError(&'static str),
    #[error(transparent)]
    IndexError(#[from] IndexError),
}

pub type SocialContextResult<T> = Result<T, SocialContextError>;
