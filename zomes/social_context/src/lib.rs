use chrono::{DateTime, Utc};
use hdk3::prelude::*;
use lazy_static::lazy_static;

mod errors;
mod impls;
mod inputs;
mod methods;
mod utils;

use inputs::*;

#[hdk_entry(id = "link_data", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct LinkExpression {
    pub author: Agent,
    pub data: Triple,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(vec![
        Path::entry_def(),
        LinkExpression::entry_def(),
        Anchor::entry_def(),
        Agent::entry_def(),
    ]
    .into())
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    let sig: LinkExpression = LinkExpression::try_from(signal.clone())?;
    Ok(emit_signal(&sig)?)
}

#[hdk_extern]
pub fn add_link(link: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::add_link(link).map_err(|err| WasmError::Zome(err.to_string()))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetOthers(pub Vec<Agent>);

#[hdk_extern]
pub fn get_others(_: ()) -> ExternResult<GetOthers> {
    Ok(GetOthers(
        SocialContextDNA::get_others().map_err(|err| WasmError::Zome(err.to_string()))?,
    ))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetLinksResponse(pub Vec<LinkExpression>);

#[hdk_extern]
pub fn get_links(input: GetLinks) -> ExternResult<GetLinksResponse> {
    Ok(GetLinksResponse(
        SocialContextDNA::get_links(input).map_err(|err| WasmError::Zome(err.to_string()))?,
    ))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UpdateLink {
    pub source: LinkExpression,
    pub target: LinkExpression,
}

#[hdk_extern]
pub fn update_link(update_link: UpdateLink) -> ExternResult<()> {
    SocialContextDNA::update_link(update_link).map_err(|err| WasmError::Zome(err.to_string()))
}

#[hdk_extern]
pub fn remove_link(remove_link: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::remove_link(remove_link).map_err(|err| WasmError::Zome(err.to_string()))
}

pub struct SocialContextDNA();

//TODO: this should be derived from DNA properties so can be set for each social context based on projected size
lazy_static! {
    pub static ref ACTIVE_AGENT_DURATION: chrono::Duration = chrono::Duration::hours(2);
    pub static ref ACTIVE_AGENT_INDEX_TAG: String = String::from("active_agent");
    //Number of agents to send signal at once
    pub static ref EMIT_RS_BATCH_SIZE: usize = 10;
}
