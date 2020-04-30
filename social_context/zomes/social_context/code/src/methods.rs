use hdk::holochain_persistence_api::hash::HashString;
use hdk::{
    api::AGENT_ADDRESS,
    error::{ZomeApiError, ZomeApiResult},
    holochain_json_api::json::JsonString,
    prelude::{Address, Entry, GetLinksOptions, LinkMatch, Pagination, SizePagination, SortOrder},
};
use meta_traits::{GlobalEntryRef, SocialContextDao};
use multihash::Hash;

use crate::{Anchor, AnchorTypes, DnaAddressReference, SocialContextDNA};

impl SocialContextDao for SocialContextDNA {
    fn post(expression_ref: GlobalEntryRef) -> ZomeApiResult<()> {
        let dna_reference = hdk::commit_entry(&Entry::App(
            "dna_address_reference".into(),
            DnaAddressReference {
                address: expression_ref.dna_address.clone(),
            }
            .into(),
        ))?;
        let entry_reference = hdk::commit_entry(&Entry::App(
            "global_entry_ref".into(),
            GlobalEntryRef {
                dna_address: expression_ref.dna_address,
                entry_address: expression_ref.entry_address,
            }
            .into(),
        ))?;
        hdk::link_entries(&dna_reference, &entry_reference, "", "")?;
        hdk::link_entries(&AGENT_ADDRESS, &entry_reference, "", "")?;
        Ok(())
    }

    fn register_communication_method(dna_address: Address) -> ZomeApiResult<()> {
        let dna_reference = hdk::commit_entry(&Entry::App(
            "dna_address_reference".into(),
            DnaAddressReference {
                address: dna_address,
            }
            .into(),
        ))?;
        let dna_anchor = hdk::commit_entry(&Entry::App(
            "anchor".into(),
            Anchor {
                r#type: AnchorTypes::DNA,
            }
            .into(),
        ))?;
        hdk::link_entries(&dna_anchor, &dna_reference, "", "")?;
        Ok(())
    }

    fn writable() -> bool {
        //Public open Social Context so this is always true
        //Private social context may call some Permissions trait to validate this information
        true
    }

    fn read_communications(
        by_dna: Option<Address>,
        by_agent: Option<Address>,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<Vec<GlobalEntryRef>> {
        if by_dna.is_none() && by_agent.is_none() {
            return Err(ZomeApiError::Internal(String::from(
                "You must pass either by_dna or by_agent",
            )));
        };
        if by_dna.is_some() && by_agent.is_some() {
            return Err(ZomeApiError::Internal(String::from(
                "You must pass either by_dna or by_agent - not both",
            )));
        };
        Ok(match by_dna {
            Some(address) => {
                let dna_reference_entry: Address = HashString::encode_from_json_string(
                    JsonString::from(Entry::App(
                        "dna_address_reference".into(),
                        DnaAddressReference { address: address }.into(),
                    )),
                    Hash::SHA2256,
                );
                hdk::get_links_with_options(
                    &dna_reference_entry,
                    LinkMatch::Any,
                    LinkMatch::Any,
                    GetLinksOptions {
                        status_request: Default::default(),
                        headers: false,
                        timeout: Default::default(),
                        pagination: Some(Pagination::Size(SizePagination {
                            page_number: page,
                            page_size: count,
                        })),
                        sort_order: Some(SortOrder::Descending),
                    },
                )?
                .addresses()
                .into_iter()
                .map(|link_target_address| {
                    hdk::utils::get_as_type::<GlobalEntryRef>(link_target_address)
                })
                .collect::<ZomeApiResult<Vec<GlobalEntryRef>>>()?
            }
            None => match by_agent {
                Some(identity) => hdk::get_links_with_options(
                    &identity,
                    LinkMatch::Any,
                    LinkMatch::Any,
                    GetLinksOptions {
                        status_request: Default::default(),
                        headers: false,
                        timeout: Default::default(),
                        pagination: Some(Pagination::Size(SizePagination {
                            page_number: page,
                            page_size: count,
                        })),
                        sort_order: Some(SortOrder::Descending),
                    },
                )?
                .addresses()
                .into_iter()
                .map(|link_target_address| {
                    hdk::utils::get_as_type::<GlobalEntryRef>(link_target_address)
                })
                .collect::<ZomeApiResult<Vec<GlobalEntryRef>>>()?,
                None => unreachable!(),
            },
        })
    }

    fn get_communication_methods(count: usize, page: usize) -> ZomeApiResult<Vec<Address>> {
        let dna_anchor: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "anchor".into(),
                Anchor {
                    r#type: AnchorTypes::DNA,
                }
                .into(),
            )),
            Hash::SHA2256,
        );
        Ok(hdk::get_links_with_options(
            &dna_anchor,
            LinkMatch::Any,
            LinkMatch::Any,
            GetLinksOptions {
                status_request: Default::default(),
                headers: false,
                timeout: Default::default(),
                pagination: Some(Pagination::Size(SizePagination {
                    page_number: page,
                    page_size: count,
                })),
                sort_order: Some(SortOrder::Descending),
            },
        )?
        .addresses()
        .into_iter()
        .map(|link_target_address| {
            hdk::utils::get_as_type::<DnaAddressReference>(link_target_address)
        })
        .collect::<ZomeApiResult<Vec<DnaAddressReference>>>()?
        .into_iter()
        .map(|dna_reference| dna_reference.address)
        .collect::<Vec<Address>>())
    }

    fn members(count: usize, page: usize) -> ZomeApiResult<Option<Vec<Address>>> {
        let user_anchor: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "anchor".into(),
                Anchor {
                    r#type: AnchorTypes::User,
                }
                .into(),
            )),
            Hash::SHA2256,
        );
        Ok(Some(
            hdk::get_links_with_options(
                &user_anchor,
                LinkMatch::Any,
                LinkMatch::Any,
                GetLinksOptions {
                    status_request: Default::default(),
                    headers: false,
                    timeout: Default::default(),
                    pagination: Some(Pagination::Size(SizePagination {
                        page_number: page,
                        page_size: count,
                    })),
                    sort_order: Some(SortOrder::Descending),
                },
            )?
            .addresses(),
        ))
    }
}
