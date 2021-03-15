#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, Utc};
use hdk3::prelude::*;

mod inputs;
mod methods;
mod utils;
mod impls;

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
pub fn add_link(link: LinkExpression) -> ExternResult<()> {
    debug!("Adding a link from lib");
    SocialContextDNA::add_link(link)
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetOthers(pub Vec<Agent>);

#[hdk_extern]
pub fn get_others(_: ()) -> ExternResult<GetOthers> {
    Ok(GetOthers(SocialContextDNA::get_others()?))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetLinksResponse(pub Vec<LinkExpression>);

#[hdk_extern]
pub fn get_links(input: GetLinks) -> ExternResult<GetLinksResponse> {
    Ok(GetLinksResponse(SocialContextDNA::get_links(input)?))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UpdateLink {
    pub source: LinkExpression,
    pub target: LinkExpression,
}

#[hdk_extern]
pub fn update_link(update_link: UpdateLink) -> ExternResult<()> {
    SocialContextDNA::update_link(update_link)
}

#[hdk_extern]
pub fn remove_link(remove_link: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::remove_link(remove_link)
}

lazy_static! {
    //Set the membrane list for this DNA
    pub static ref MEMBRANE: Option<Vec<AgentInfo>> = None;
}

pub struct SocialContextDNA();
