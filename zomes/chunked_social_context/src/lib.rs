#[macro_use]
extern crate lazy_static;

use hdk3::hash_path::anchor::Anchor;
use hdk3::prelude::*;

use meta_traits::{GlobalEntryRef, GlobalEntryRefChunked};

mod inputs;
mod methods;
mod out;

use inputs::*;
use out::*;

/// Right now this social context does not employ any kind of validation for incoming references or allow any permissioned posting
/// of either entry ref's, communication methods or agents allowed to join.
///
/// This social context is identical in function to the other social_context zome but also employs chunk/path based indexing with the hopes to
/// reduce DHT hotspotting. This assumption is expected to be tested.

/// Entry refs
#[hdk_entry(id = "dna_address_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
struct EntryRefPublic(GlobalEntryRef);

#[hdk_entry(id = "dna_address_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
struct EntryRefPrivate(GlobalEntryRef);

#[hdk_entry(id = "user_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug)]
pub struct UserReference {
    pub address: AgentPubKey,
}

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(vec![
        EntryRefPublic::entry_def(),
        EntryRefPrivate::entry_def(),
        UserReference::entry_def(),
        Anchor::entry_def()
    ]
    .into())
}

/// Extern zome functions

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let user_anchor = Anchor {
        anchor_type: String::from("global_user_anchor"),
        anchor_text: None,
    };
    let anchor_hash = hash_entry(&user_anchor)?;
    create_entry(&user_anchor)?;

    //Here we actually dont need to store the address since it is already present with the entry
    //but for now this is fine
    let user_reference = UserReference {
        address: agent_info()?.agent_latest_pubkey,
    };
    let user_reference_hash = hash_entry(&user_reference)?;
    create_entry(&user_reference)?;

    create_link(anchor_hash, user_reference_hash, LinkTag::new("member"))?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn post(expression_ref: GlobalEntryRefChunked) -> ExternResult<()> {
    SocialContextDNA::post(expression_ref)
}

#[hdk_extern]
pub fn register_communication_method(dna_address: DnaAddress) -> ExternResult<()> {
    SocialContextDNA::register_communication_method(dna_address.dna_address)
}

#[hdk_extern]
pub fn writable(_: ()) -> ExternResult<BoolOutput> {
    Ok(BoolOutput(SocialContextDNA::writable()))
}

#[hdk_extern]
pub fn read_communications(args: ReadCommunicationArguments) -> ExternResult<EntryRefListOut> {
    //Again here we are using strings and then doing the conversion to rust types here vs it happening inside hdk_extern
    //let args: ReadCommunicationArguments = args.try_into()?;
    Ok(EntryRefListOut(SocialContextDNA::read_communications(
        args.by_dna,
        args.by_agent,
        args.from_chunk as u32,
        args.to_chunk as u32,
    )?))
}

#[hdk_extern]
pub fn get_communication_methods(args: PaginationArguments) -> ExternResult<DnaListOutput> {
    Ok(DnaListOutput(SocialContextDNA::get_communication_methods(
        args.from_chunk as u32, args.to_chunk as u32,
    )?))
}

#[hdk_extern]
pub fn members(args: PaginationArguments) -> ExternResult<IdentityListOutput> {
    Ok(IdentityListOutput(SocialContextDNA::members(
        args.from_chunk, args.to_chunk,
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
    pub static ref SOFT_CHUNK_LIMIT: usize = 50;
    pub static ref HARD_CHUNK_LIMIT: usize = 100;
}

pub struct SocialContextDNA();
