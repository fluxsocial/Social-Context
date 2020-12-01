#[macro_use]
extern crate lazy_static;

use hdk3::hash_path::anchor::Anchor;
use hdk3::prelude::*;
use holo_hash::DnaHash;

use meta_traits::{GlobalEntryRef, SocialContextDao};

mod inputs;
mod methods;
mod out;

use inputs::*;
use out::*;

/// Right now this social context does not employ any kind of validation for incoming references or allow any permissioned posting
/// of either entry ref's, communication methods or agents allowed to join.
///
/// This is serving as a POC for now with the hopes to add above functionality as when it makes sense to and hdk3 support & design patterns
/// materialze further.
///
/// TODO:
/// Write and run some tests
/// Add validation logic for posting references; at the very least here we would want to check that target reference is a real thing
/// Validation logic for permissioned based posting
/// Allow for membraining of network
/// Make use of cap grants or some other pattern to design in optional permissioning for agents
/// Allow the use of other index methods listed here

/// Entry refs
#[hdk_entry(id = "dna_address_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
struct EntryRefPublic(GlobalEntryRef);

#[hdk_entry(id = "dna_address_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
struct EntryRefPrivate(GlobalEntryRef);

#[hdk_entry(id = "dna_address_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct DnaAddressReference {
    pub address: DnaHash,
}

#[hdk_entry(id = "user_ref", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct UserReference {
    pub address: agent_info::AgentInfo,
}

#[hdk_extern]
fn entry_defs(_: ()) -> ExternResult<EntryDefsCallbackResult> {
    Ok(vec![
        DnaAddressReference::entry_def(),
        EntryRefPublic::entry_def(),
        EntryRefPrivate::entry_def(),
        Anchor::entry_def(),
    ]
    .into())
}

/// Extern zome functions

#[hdk_extern]
pub fn post(expression_ref: GlobalEntryRef) -> ExternResult<()> {
    //let expression_ref: GlobalEntryRef = expression_ref.try_into()?;
    SocialContextDNA::post(expression_ref)
}

#[hdk_extern]
pub fn register_communication_method(dna_address: DnaAddress) -> ExternResult<()> {
    //Right now we need this because hdk_extern is failing in parsing incoming string to DnaHash even though string is confirmed to be valid
    // let dna_address: DnaHash = dna_address.dna_address.try_into().map_err(|_err| {
    //     HdkError::Wasm(WasmError::Zome(String::from(
    //         "Incoming dna_address not valid dna hash",
    //     )))
    // })?;
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
        args.count,
        args.page,
    )?))
}

#[hdk_extern]
pub fn get_communication_methods(args: PaginationArguments) -> ExternResult<DnaListOutput> {
    Ok(DnaListOutput(SocialContextDNA::get_communication_methods(
        args.count, args.page,
    )?))
}

#[hdk_extern]
pub fn members(args: PaginationArguments) -> ExternResult<IdentityListOutput> {
    Ok(IdentityListOutput(SocialContextDNA::members(
        args.count, args.page,
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
    /// Local bucket based indexing
    /// Involves spreading links across many fixed or dynamic entries to avoid hotspots
    LocalBuckets,
    /// Uses some external persistence mediated through node2node communication with agents at given addresses
    RemoteIndex {
        target_index_agents: Vec<agent_info::AgentInfo>,
    },
}

lazy_static! {
    //Set the indexing strategy; this would ideally be configured somehow for each social context clone being made
    pub static ref INDEX_STRATEGY: Vec<IndexStrategies> = vec![IndexStrategies::LocalAnchor];
    //Set the membrane list for this DNA
    pub static ref MEMBRANE: Option<Vec<AgentInfo>> = None;
}

pub struct SocialContextDNA();
