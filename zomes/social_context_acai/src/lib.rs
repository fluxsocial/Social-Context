#[macro_use]
extern crate lazy_static;

use hdk3::prelude::*;

mod inputs;
mod methods;

use inputs::*;

#[hdk_entry(id = "link_acai_data", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct LinkExpression {
    pub author: Agent,
    pub data: Triple,
    pub timestamp: String,
    pub proof: ExpressionProof,
}

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(vec![Path::entry_def(), LinkExpression::entry_def(), Anchor::entry_def(), Agent::entry_def()].into())
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

// #[hdk_extern]
// pub fn validate_create_link(_: ()) -> ExternResult<ValidateLinkCallbackResult> {
//     Ok(ValidateLinkCallbackResult::Valid)
// }

#[hdk_extern]
pub fn add_link(link: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::add_link(link)
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetOthers(pub Vec<Agent>);

#[hdk_extern]
pub fn get_others(_: ()) -> ExternResult<GetOthers> {
    Ok(GetOthers(SocialContextDNA::get_others()?))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetLinksResponse(pub Vec<LinkExpression>);

#[hdk_extern]
pub fn get_links(input: Triple) -> ExternResult<GetLinksResponse> {
    Ok(GetLinksResponse(SocialContextDNA::get_links(input)?))
}

/// Configuration

/// Possible methods of indexing social context data
/// Some applications may wish to only use local DHT storage at the cost of performance due to DHT hot-spotting
/// others may be happy to use some remote indexing network or machine to handle the index
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IndexStrategies {
    /// Local simple Anchor based indexing
    LocalAnchor,
    /// Local chunk based indexing/anchoring
    LocalChunks,
    /// Uses some external persistence mediated through node2node communication with agents at given addresses
    RemoteIndex {
        target_index_agents: Vec<agent_info::AgentInfo>,
    },
}

lazy_static! {
    //Set the indexing strategy; this would ideally be configured somehow for each social context clone being made
    pub static ref INDEX_STRATEGY: Vec<IndexStrategies> = vec![IndexStrategies::LocalChunks];
    //Set the membrane list for this DNA
    pub static ref MEMBRANE: Option<Vec<AgentInfo>> = None;
    //Limits for the max number of links that are allowed on a given chunk
    //Zome logic will evaluate on soft limit as to try and reduce cases where lawful actors
    //commit links on some chunk which they read to be free but actually by validation time is full due to async operations
    //or (consistency concerns?)
    pub static ref SOFT_CHUNK_LIMIT: usize = 30;
    pub static ref HARD_CHUNK_LIMIT: usize = 50;
}

pub struct SocialContextDNA();
