use hdk3::hash_path::anchor::Anchor;
use hdk3::prelude::*;
use holo_hash::DnaHash;

use meta_traits::{GlobalEntryRef, Identity, SocialContextDao};

use crate::{DnaAddressReference, EntryRefPublic, SocialContextDNA, UserReference};

impl SocialContextDao for SocialContextDNA {
    /// Persist to social context that you have made an entry at expression_ref.dna_address/@expression_ref.entry_address
    /// which is most likely contextual to the collective of host social context
    fn post(expression_ref: GlobalEntryRef) -> ExternResult<()> {
        let dna_ref = DnaAddressReference {
            address: expression_ref.dna.clone(),
        };
        let dna_ref_entry_hash = hash_entry(&dna_ref)?;
        let dna_reference = create_entry(&dna_ref)?;
        debug!(format!(
            "Created dna ref with header: {:?} & entry: {:?}\n\n",
            dna_reference, dna_ref_entry_hash
        ));

        let entry_ref = EntryRefPublic(GlobalEntryRef {
            dna: expression_ref.dna,
            entry_address: expression_ref.entry_address,
        });
        let entry_ref_hash = hash_entry(&entry_ref)?;
        let entry_reference = create_entry(&entry_ref)?;
        debug!(format!("Created entry ref: {:?}\n\n", entry_reference));

        create_link(
            dna_ref_entry_hash,
            entry_ref_hash.clone(),
            LinkTag::new("communication"),
        )?;

        let agent_address: AnyDhtHash = agent_info()?.agent_initial_pubkey.clone().into();
        create_link(
            agent_address.into(),
            entry_ref_hash,
            LinkTag::new("agent_communication"),
        )?;
        Ok(())
    }

    /// Register that there is some dna at dna_address that you are communicating in.
    /// Others in collective can use this to join you in new DNA's
    fn register_communication_method(dna_address: DnaHash) -> ExternResult<()> {
        //Maybe here was want some provenenace support such that only certain members can make new communication DNA's known
        let dna_ref = DnaAddressReference {
            address: dna_address,
        };
        let dna_ref_entry_hash = hash_entry(&dna_ref)?;
        let dna_reference = create_entry(&dna_ref)?;
        debug!(format!("Created dna ref: {:?}\n\n", dna_reference));

        let dna_anchor = Anchor {
            anchor_type: String::from("dna"),
            anchor_text: None,
        };
        let dna_anchor_entry_hash = hash_entry(&dna_anchor)?;
        let dna_anchor_reference = create_entry(&dna_anchor)?;
        debug!(format!(
            "Created dna ref anchor: {:?}\n\n",
            dna_anchor_reference
        ));

        create_link(
            dna_anchor_entry_hash,
            dna_ref_entry_hash,
            LinkTag::new("communication_method"),
        )?;
        Ok(())
    }

    /// Is current agent allowed to write to this DNA
    fn writable() -> bool {
        //Public open Social Context so this is always true
        //Private social context may call some Permissions trait to validate this information
        true
    }

    /// Get GlobalEntryRef for collective; queryable by dna or agent or all. DHT hotspotting @Nico?
    fn read_communications(
        by_dna: Option<DnaHash>,
        by_agent: Option<Identity>,
        _count: usize,
        _page: usize,
    ) -> ExternResult<Vec<GlobalEntryRef>> {
        if by_dna.is_some() && by_agent.is_some() {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "You must pass either by_dna or by_agent - not both",
            ))));
        };

        let ref_links = if let Some(dna_address) = by_dna {
            let dna_ref = DnaAddressReference {
                address: dna_address.clone(),
            };
            let dna_ref_entry_hash = hash_entry(&dna_ref)?;
            debug!("getting communications for dna at {:?}", dna_ref_entry_hash);
            get_links(dna_ref_entry_hash, Some(LinkTag::new("communication")))?
        } else if let Some(agent_ident) = by_agent {
            let agent_hash: AnyDhtHash = agent_ident.into();
            debug!("getting communications for agent at {:?}", agent_hash);
            get_links(agent_hash.into(), Some(LinkTag::new("agent_communication")))?
        } else {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "by_dna or by_agent must be passed",
            ))));
        };

        let refs = ref_links
            .into_inner()
            .into_iter()
            .map(|val| {
                let element = get(val.target.clone(), GetOptions::default())?.ok_or(
                    HdkError::Wasm(WasmError::Zome(format!(
                        "Could not get entry for link with target: {}",
                        val.target
                    ))),
                )?;
                let entry_ref = try_from_entry::<EntryRefPublic>(
                    element
                        .entry()
                        .as_option()
                        .ok_or(HdkError::Wasm(WasmError::Zome(format!(
                            "Could not get entry for link with target: {}",
                            val.target
                        ))))?
                        .to_owned(),
                )?
                .0;
                Ok(GlobalEntryRef {
                    dna: entry_ref.dna,
                    entry_address: entry_ref.entry_address,
                })
            })
            .collect::<ExternResult<Vec<GlobalEntryRef>>>()?;

        Ok(refs)
    }

    /// Get DNA's this social context is communicating in
    fn get_communication_methods(_count: usize, _page: usize) -> ExternResult<Vec<DnaHash>> {
        let dna_anchor = Anchor {
            anchor_type: String::from("dna"),
            anchor_text: None,
        };
        let dna_anchor_entry_hash = hash_entry(&dna_anchor)?;

        let communication_links = get_links(
            dna_anchor_entry_hash.into(),
            Some(LinkTag::new("communication_method")),
        )?;

        let communication_links = communication_links
            .into_inner()
            .into_iter()
            .map(|val| {
                let element = get(val.target.clone(), GetOptions::default())?.ok_or(
                    HdkError::Wasm(WasmError::Zome(format!(
                        "Could not get entry for link with target: {}",
                        val.target
                    ))),
                )?;
                Ok(try_from_entry::<DnaAddressReference>(
                    element
                        .entry()
                        .as_option()
                        .ok_or(HdkError::Wasm(WasmError::Zome(format!(
                            "Could not get entry for link with target: {}",
                            val.target
                        ))))?
                        .to_owned(),
                )?
                .address)
            })
            .collect::<ExternResult<Vec<DnaHash>>>()?;
        Ok(communication_links)
    }

    /// Get agents who are a part of this social context
    /// optional to not force every implementation to create a global list of members - might be ok for small DHTs
    fn members(_count: usize, _page: usize) -> ExternResult<Option<Vec<Identity>>> {
        let user_anchor = Anchor {
            anchor_type: String::from("global_user_anchor"),
            anchor_text: None,
        };
        let anchor_hash = hash_entry(&user_anchor)?;
        Ok(Some(
            get_links(anchor_hash, Some(LinkTag::new("member")))?
                .into_inner()
                .into_iter()
                .map(|link| {
                    let user_entry = get(link.target.clone(), GetOptions::default())?.ok_or(
                        HdkError::Wasm(WasmError::Zome(format!(
                            "Could not get entry for link with target: {}",
                            link.target
                        ))),
                    )?;
                    Ok(try_from_entry::<UserReference>(
                        user_entry
                            .entry()
                            .as_option()
                            .ok_or(HdkError::Wasm(WasmError::Zome(format!(
                                "Could not get entry for link with target: {}",
                                link.target
                            ))))?
                            .to_owned(),
                    )?
                    .address)
                })
                .collect::<ExternResult<Vec<AgentPubKey>>>()?,
        ))
    }
}

pub fn try_from_entry<T: TryFrom<SerializedBytes>>(entry: Entry) -> ExternResult<T> {
    match entry {
        Entry::App(content) => match T::try_from(content.into_sb()) {
            Ok(e) => Ok(e),
            Err(_) => Err(HdkError::Wasm(WasmError::Zome(String::from(
                "Could not create entry",
            )))),
        },
        _ => Err(HdkError::Wasm(WasmError::Zome(String::from(
            "Could not create Entry::App variant from incoming Entry",
        )))),
    }
}
