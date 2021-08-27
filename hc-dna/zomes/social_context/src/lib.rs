use chrono::{DateTime, Utc};
use hdk::prelude::*;
use lazy_static::lazy_static;

mod errors;
mod impls;
mod inputs;
mod methods;
mod utils;
mod validation;

use inputs::*;
pub use validation::*;


entry_defs![Path::entry_def(), LinkExpression::entry_def(), AgentReference::entry_def()];

#[hdk_entry(id = "link_data", visibility = "public")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct LinkExpression {
    pub author: String,
    pub data: Triple,
    pub timestamp: DateTime<Utc>,
    pub proof: ExpressionProof,
}

#[hdk_entry(id = "agent_reference", visbility = "public")]
#[derive(Clone)]
pub struct AgentReference {
    pub agent: AgentPubKey,
    pub timestamp: DateTime<Utc>,
}

pub struct SocialContextDNA;

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let mut functions: GrantedFunctions = BTreeSet::new();
    functions.insert((zome_info()?.zome_name, "recv_remote_signal".into()));

    //Create open cap grant to allow agents to send signals of links to each other
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
fn recv_remote_signal(signal: SerializedBytes) -> ExternResult<()> {
    let sig: LinkExpression = LinkExpression::try_from(signal.clone())?;
    Ok(emit_signal(&sig)?)
}

#[hdk_extern]
pub fn add_link(add_link_data: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::add_link(add_link_data).map_err(|err| WasmError::Host(err.to_string()))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct AddActiveAgentLinkResponse {
    pub existing: Option<DateTime<Utc>>,
}

#[hdk_extern]
pub fn add_active_agent_link(_: ()) -> ExternResult<AddActiveAgentLinkResponse> {
    let res = SocialContextDNA::add_active_agent_link().map_err(|err| WasmError::Host(err.to_string()))?;
    Ok(AddActiveAgentLinkResponse {
        existing: res
    })
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct GetLinksResponse(pub Vec<LinkExpression>);

#[hdk_extern]
pub fn get_links(input: GetLinks) -> ExternResult<GetLinksResponse> {
    Ok(GetLinksResponse(
        SocialContextDNA::get_links(input).map_err(|err| WasmError::Host(err.to_string()))?,
    ))
}

#[derive(Serialize, Deserialize, Clone, SerializedBytes, Debug)]
pub struct UpdateLink {
    pub source: LinkExpression,
    pub target: LinkExpression,
}

#[hdk_extern]
pub fn update_link(update_link: UpdateLink) -> ExternResult<()> {
    SocialContextDNA::update_link(update_link).map_err(|err| WasmError::Host(err.to_string()))
}

#[hdk_extern]
pub fn remove_link(remove_link: LinkExpression) -> ExternResult<()> {
    SocialContextDNA::remove_link(remove_link).map_err(|err| WasmError::Host(err.to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IndexStrategy {
    FullWithWildCard,
    Full, 
    Simple,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
pub struct SocialContextProperties {
    pub active_agent_duration_s: i64,
    pub enable_signals: bool,
    //TODO: lets set this per add_link zome call and not lock in each DNA to enabling or disabling this feature, there are cases where you want both time indexes and regular
    pub enable_time_index: bool,
    pub index_strategy: String,
}

lazy_static! {
    pub static ref ACTIVE_AGENT_DURATION: chrono::Duration = {
        let host_dna_config = zome_info()
            .expect("Could not get zome configuration")
            .properties;
        let properties = SocialContextProperties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to SocialContextProperties. Please ensure that your dna properties contains a SocialContextProperties field.");
        chrono::Duration::seconds(properties.active_agent_duration_s)
    };
    pub static ref ENABLE_SIGNALS: bool = {
        let host_dna_config = zome_info()
            .expect("Could not get zome configuration")
            .properties;
        let properties = SocialContextProperties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to SocialContextProperties. Please ensure that your dna properties contains a SocialContextProperties field.");
        properties.enable_signals
    };
    pub static ref ENABLE_TIME_INDEX: bool = {
        let host_dna_config = zome_info()
            .expect("Could not get zome configuration")
            .properties;
        let properties = SocialContextProperties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to SocialContextProperties. Please ensure that your dna properties contains a SocialContextProperties field.");
        properties.enable_time_index
    };
    pub static ref INDEX_STRAT: IndexStrategy = {
        let host_dna_config = zome_info()
            .expect("Could not get zome configuration")
            .properties;
        let properties = SocialContextProperties::try_from(host_dna_config)
            .expect("Could not convert zome dna properties to SocialContextProperties. Please ensure that your dna properties contains a SocialContextProperties field.");
        IndexStrategy::try_from(properties.index_strategy).expect("Could not get index strategy from string supplied in index_strategy property")
    };
}
