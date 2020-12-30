#[macro_use]
extern crate lazy_static;

use hdk3::prelude::*;

mod inputs;
mod methods;
mod out;

use inputs::*;

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(vec![Path::entry_def()].into())
}

/// Extern zome functions

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // let user_anchor_path = Path::from(&Anchor {
    //     anchor_type: String::from("global_user_anchor"),
    //     anchor_text: None,
    // });
    // //Get a chunk to use on this user anchor
    // let chunk = methods::get_free_chunk(&user_anchor_path, LinkTag::new("member"))?;

    // let user_anchor_path = methods::add_chunk_path(user_anchor_path, chunk);
    // let anchor_hash = hash_entry(&user_anchor_path)?;
    // user_anchor_path.ensure()?;

    // //Here we actually dont need to store the address since it is already present with the entry
    // //but for now this is fine
    // let user_reference = UserReference {
    //     address: agent_info()?.agent_latest_pubkey,
    // };
    // let user_reference_hash = hash_entry(&user_reference)?;
    // create_entry(&user_reference)?;

    // create_link(anchor_hash, user_reference_hash, LinkTag::new("member"))?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn add_link_auto_index(link: Triple) -> ExternResult<()> {
    let link = TripleParsed::try_from(link)?;
    SocialContextDNA::add_link_auto_index(link)
}

#[hdk_extern]
pub fn add_link(link: Triple) -> ExternResult<()> {
    let link = TripleParsed::try_from(link)?;
    SocialContextDNA::add_link(link)
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes)]
pub struct GetLinksResponse(pub Vec<Triple>);

#[hdk_extern]
pub fn get_links(input: GetLinks) -> ExternResult<GetLinksResponse> {
    Ok(GetLinksResponse(SocialContextDNA::get_links(
        input.subject,
        input.predicate,
    )?))
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
