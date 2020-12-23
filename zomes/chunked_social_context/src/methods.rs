use hdk3::hash_path::{anchor::Anchor, path::Component};
use hdk3::prelude::*;
use holo_hash::DnaHash;

use meta_traits::{GlobalEntryRefChunked, Identity, GlobalEntryRef};

use crate::{EntryRefPublic, SocialContextDNA, UserReference, SOFT_CHUNK_LIMIT};

impl SocialContextDNA {
    /// Persist to social context that you have made an entry at expression_ref.dna_address/@expression_ref.entry_address
    /// which is most likely contextual to the collective of host social context
    pub fn post(expression_ref: GlobalEntryRefChunked) -> ExternResult<()> {
        //Get dna path with chunk
        let component = Component::from(expression_ref.dna.get_raw_36().to_owned());
        let dna_path = Path::from(vec![component]);
        let dna_path = add_chunk_path(dna_path, expression_ref.chunk);

        // Ensure the path exists
        dna_path.ensure()?;

        //Get the hash of this path
        let dna_ref_path_hash = hash_entry(&dna_path)?;

        //Create entry ref
        let entry_ref = EntryRefPublic(GlobalEntryRef {
            dna: expression_ref.dna,
            entry_address: expression_ref.entry_address,
        });
        let entry_ref_hash = hash_entry(&entry_ref)?;
        let entry_reference = create_entry(&entry_ref)?;
        debug!(format!("Created entry ref: {:?}\n\n", entry_reference));

        //Create link between path and entry ref
        create_link(
            dna_ref_path_hash,
            entry_ref_hash.clone(),
            LinkTag::new("communication"),
        )?;

        //Also create link on agent pub key; this is fine not to be chunked from performance perspective
        //But if pagination will not be added then chunking can be a good way to add pagination like behaviour
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
    pub fn register_communication_method(dna_address: DnaHash) -> ExternResult<()> {
        //Get dna path
        let dna_path = Path::from(dna_address.to_string());
        dna_path.ensure()?;
        let dna_ref_path_hash = hash_entry(&dna_path)?;

        //Create root dna path "anchor"
        let root_dna_path = Path::from("dna_anchor");
        root_dna_path.ensure()?;

        //Look for a chunk which does not have many links on it
        let chunk = get_free_chunk(&root_dna_path, LinkTag::from("communication_method".as_bytes().to_owned()))?;

        let root_dna_path = add_chunk_path(root_dna_path, chunk);
        root_dna_path.ensure()?;

        let root_dna_path = hash_entry(&root_dna_path)?;

        create_link(
            root_dna_path,
            dna_ref_path_hash,
            LinkTag::new("communication_method"),
        )?;
        Ok(())
    }

    /// Is current agent allowed to write to this DNA
    pub fn writable() -> bool {
        //Public open Social Context so this is always true
        //Private social context may call some Permissions trait to validate this information
        true
    }

    /// Get GlobalEntryRef for collective; queryable by dna or agent or all. DHT hotspotting @Nico?
    pub fn read_communications(
        by_dna: Option<DnaHash>,
        by_agent: Option<Identity>,
        from_chunk: u32,
        to_chunk: u32,
    ) -> ExternResult<Vec<GlobalEntryRef>> {
        if by_dna.is_some() && by_agent.is_some() {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "You must pass either by_dna or by_agent - not both",
            ))));
        };

        let ref_links = if let Some(dna_address) = by_dna {
            let component = Component::from(dna_address.get_raw_39().to_owned());
            let dna_path = Path::from(vec![component]);
            let mut links: Vec<Link> = Vec::new();
            let mut counter = from_chunk as u32;
            loop {        
                // Add the chunk component
                let chunked_dna_path = add_chunk_path(dna_path.clone(), counter);
        
                // Ensure the path exists
                chunked_dna_path.ensure()?;
        
                // Get the actual hash we are going to pull the messages from
                let channel_entry_hash = chunked_dna_path.hash()?;
        
                // Get the message links on this channel
                links.append(&mut get_links(channel_entry_hash.clone(), Some(LinkTag::new("communication")))?.into_inner());
                if counter == to_chunk {
                    break;
                }
                counter += 1
            };
            links
        } else if let Some(agent_ident) = by_agent {
            let agent_hash: AnyDhtHash = agent_ident.into();
            debug!("getting communications for agent at {:?}", agent_hash);
            get_links(agent_hash.into(), Some(LinkTag::new("agent_communication")))?.into_inner()
        } else {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "by_dna or by_agent must be passed",
            ))));
        };

        let refs = ref_links
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
    pub fn get_communication_methods(from_chunk: u32, to_chunk: u32) -> ExternResult<Vec<DnaHash>> {
        //Create root dna path "anchor"
        let root_dna_path = Path::from("dna_anchor");

        let mut communication_links: Vec<Link> = Vec::new();
        let mut counter = from_chunk as u32;
        loop {
            // Add the chunk component
            let root_dna_path = add_chunk_path(root_dna_path.clone(), counter);
    
            // Ensure the path exists
            root_dna_path.ensure()?;
    
            // Get the actual hash we are going to pull the messages from
            let channel_entry_hash = root_dna_path.hash()?;
    
            // Get the message communication_links on this channel
            communication_links.append(&mut get_links(channel_entry_hash.clone(), Some(LinkTag::new("communication_method")))?.into_inner());
            if counter == to_chunk {
                break;
            }
            counter += 1
        };
        
        let communication_links = communication_links
            .into_iter()
            .map(|val| {
                let element = get(val.target.clone(), GetOptions::default())?.ok_or(
                    HdkError::Wasm(WasmError::Zome(format!(
                        "Could not get entry for link with target: {}",
                        val.target
                    ))),
                )?;
                //Not sure about this...
                let mut component: Vec<Component> = try_from_entry::<Path>(
                    element
                        .entry()
                        .as_option()
                        .ok_or(HdkError::Wasm(WasmError::Zome(format!(
                            "Could not get entry for link with target: {}",
                            val.target
                        ))))?
                        .to_owned(),
                )?.into(); 
                Ok(DnaHash::from_raw_36(component.remove(0).into()))
            })
            .collect::<ExternResult<Vec<DnaHash>>>()?;
        Ok(communication_links)
    }

    /// Get agents who are a part of this social context
    /// optional to not force every implementation to create a global list of members - might be ok for small DHTs
    pub fn members(_count: usize, _page: usize) -> ExternResult<Option<Vec<Identity>>> {
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

/// Add the chunk index from the Date type to this path
fn add_chunk_path(path: Path, chunk: u32) -> Path {
    let mut components: Vec<_> = path.into();

    components.push(format!("{}", chunk).into());
    components.into()
}

//TODO: this can be cleaned up
fn get_free_chunk(path: &Path, tag: LinkTag) -> ExternResult<u32> {
    let mut current_chunk = 0;
    let chunked_path = add_chunk_path(path.clone(), current_chunk);
    let mut current_path = hash_entry(&chunked_path)?;

    let chunk_val = loop {
        if get_links(current_path.clone(), Some(tag.clone()))?.into_inner().len() < *SOFT_CHUNK_LIMIT {
            break current_chunk;
        } else {
            current_chunk = current_chunk + 1;
            let chunked_path = add_chunk_path(path.clone(), current_chunk.clone());
            current_path = hash_entry(&chunked_path)?;
        };
    };
    Ok(chunk_val)
}