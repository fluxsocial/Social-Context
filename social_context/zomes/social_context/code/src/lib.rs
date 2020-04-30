#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

pub mod methods;

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::Address;
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};

use hdk_proc_macros::zome;
use meta_traits::{GlobalEntryRef, SocialContextDao};

pub struct SocialContextDNA();

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum AnchorTypes {
    /// Points to a DnaAddressReference
    DNA,
    /// Points to a Identity
    User,
    /// Points to a GlobalEntryRef
    CommunicationReference,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Anchor {
    pub r#type: AnchorTypes,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct DnaAddressReference {
    pub address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct UserReference {
    pub address: Address,
}

#[zome]
pub mod social_context {
    #[entry_def]
    pub fn anchor_entry_def() -> ValidatingEntryType {
        entry!(
            name: "anchor",
            description: "Anchor for DHT wide entry linking",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<Anchor>| {
                Ok(())
            },

            links: [
                to!(
                    "dna_address_reference",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                ),
                to!(
                    "&agent_id",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[entry_def]
    pub fn dna_address_ref_def() -> ValidatingEntryType {
        entry!(
            name: "dna_address_reference",
            description: "Entry for marking a DNA Address of communication",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<DnaAddressReference>| {
                Ok(())
            },

            links: [
                to!(
                    "global_entry_ref",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[entry_def]
    pub fn entry_ref_def() -> ValidatingEntryType {
        entry!(
            name: "global_entry_ref",
            description: "Entry for marking a piece of communication",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<GlobalEntryRef>| {
                Ok(())
            },

            links: [
                to!(
                    "dna_address_reference",
                    link_type: "source_dna",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                ),
                from!(
                    "&agent_id",
                    link_type: "",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData | {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[init]
    pub fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn post(expression_ref: GlobalEntryRef) -> ZomeApiResult<()> {
        SocialContextDNA::post(expression_ref)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn register_communication_method(dna_address: Address) -> ZomeApiResult<()> {
        SocialContextDNA::register_communication_method(dna_address)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn writable() -> bool {
        SocialContextDNA::writable()
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn read_communications(
        by_dna: Option<Address>,
        by_agent: Option<Address>,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        SocialContextDNA::read_communications(by_dna, by_agent, count, page)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn get_communication_methods(count: usize, page: usize) -> ZomeApiResult<Vec<Address>> {
        SocialContextDNA::get_communication_methods(count, page)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_context")]
    pub fn members(count: usize, page: usize) -> ZomeApiResult<Option<Vec<Address>>> {
        SocialContextDNA::members(count, page)
    }
}
