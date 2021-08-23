use chrono::{DateTime, NaiveDateTime, Utc};
use hc_time_index::{IndexableEntry, SearchStrategy};
use hdk::prelude::*;

use crate::utils::{generate_link_path_permutations, LinkPermutation};
use crate::errors::{SocialContextError, SocialContextResult};
use crate::{
    GetLinks, LinkExpression, SocialContextDNA, UpdateLink, ACTIVE_AGENT_DURATION,
    ENABLE_SIGNALS, ENABLE_TIME_INDEX, INDEX_STRAT, IndexStrategy, get_wildcard, AgentReference
};

impl SocialContextDNA {
    pub fn add_link(link: LinkExpression) -> SocialContextResult<()> {
        //Create the LinkExpression entry
        create_entry(&link)?;

        //If signals are enabled from the dna properties
        if *ENABLE_SIGNALS {
            let now = sys_time()?;
            let now = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(now.0, now.1),
                Utc,
            );
            //Get recent agents (agents which have marked themselves online in time period now -> ACTIVE_AGENT_DURATION as derived from DNA properties)
            let recent_agents = hc_time_index::get_links_and_load_for_time_span::<AgentReference>(
                String::from("active_agent"),
                now - *ACTIVE_AGENT_DURATION,
                now,
                Some(LinkTag::new("")),
                SearchStrategy::Bfs,
                None,
            )?;
            let mut recent_agents = recent_agents
                .into_iter()
                .map(|val| val.agent)
                .collect::<Vec<AgentPubKey>>();
            recent_agents.dedup();
            debug!("Social-Context.add_link: Sending signal to agents: {:#?}", recent_agents);
            remote_signal(link.clone().get_sb()?, recent_agents)?;
        };
        //Index the LinkExpression so its discoverable by source, predicate, target queries
        SocialContextDNA::index_link(link)?;
        Ok(())
    }

    pub fn add_active_agent_link() -> SocialContextResult<Option<DateTime<Utc>>> {
        let now = sys_time()?;
        let now = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(now.0, now.1),
            Utc,
        );
        //Get the recent agents so we can check that the current agent is not already 
        let recent_agents = hc_time_index::get_links_and_load_for_time_span::<AgentReference>(
            String::from("active_agent"),
            now - *ACTIVE_AGENT_DURATION,
            now,
            Some(LinkTag::new("")),
            SearchStrategy::Bfs,
            None,
        )?;
        debug!("Social-Context.add_active_agent_link: Got recent agents: {:#?}", recent_agents);

        let current_agent_online = recent_agents
            .iter()
            .find(|agent| {
                agent.agent
                    == agent_info()
                        .expect("Could not get agent info")
                        .agent_latest_pubkey
            });
        match current_agent_online {
            Some(agent_ref) => {
                //If the agent is already marked online then return the timestamp of them being online so the zome caller can add another active_agent link at the correct time in the future
                //But for now this is TODO and we will just add an agent reference anyway
                let new_agent_ref = AgentReference {
                    agent: agent_info()?.agent_initial_pubkey,
                    timestamp: now,
                };
                create_entry(&new_agent_ref)?;
                hc_time_index::index_entry(
                    String::from("active_agent"),
                    new_agent_ref,
                    LinkTag::new(""),
                )?;
                Ok(Some(agent_ref.timestamp))
            },
            None => {
                //Agent is not marked online so lets add an online agent reference
                let agent_ref = AgentReference {
                    agent: agent_info()?.agent_initial_pubkey,
                    timestamp: now,
                };
                create_entry(&agent_ref)?;
                hc_time_index::index_entry(
                    String::from("active_agent"),
                    agent_ref,
                    LinkTag::new(""),
                )?;
                Ok(None)
            }
        }
    }

    pub fn index_link(link: LinkExpression) -> SocialContextResult<()> {
        //Check the INDEX_STRATEGY defined in the DNA properties and generate appropriate number of link permutations
        //TODO: link strategy should be defined per zome call and not derived from DNA properties
        let link_indexes = match *INDEX_STRAT {
            //Index strategy is full so we generate all possible indexes to fufill all query possibilities +
            //add another wildcard index to make this discoverable when querying with no source, predicate or target 
            IndexStrategy::FullWithWildCard => {
                let mut perm = generate_link_path_permutations(&link.data)?;
                let wildcard = get_wildcard();
                perm.push(LinkPermutation::new(wildcard.to_string(), wildcard.to_string()));
                perm
            },
            IndexStrategy::Full => generate_link_path_permutations(&link.data)?,
            //Index strategy is simple so we only index using source + predicate meaning this LinkExpression will only be discoverable if a query with 
            //source + predicate matching that of the LinkExpression
            IndexStrategy::Simple => vec![LinkPermutation::new(
                link.data
                    .source
                    .clone()
                    .ok_or(SocialContextError::RequestError(
                        "Expected source with simple index strategy",
                    ))?,
                link.data
                    .predicate
                    .clone()
                    .ok_or(SocialContextError::RequestError(
                        "Expected predicate with simple index strategy",
                    ))?,
            )]
        };

        //For each LinkPermutation index the entry
        for link_index in link_indexes {
            if *ENABLE_TIME_INDEX {
                //Create index using hc_time_index crate and put it into a time tree to allow for retreival of links by time as well as source, predicate, target (IndexStrategy dependant)
                hc_time_index::index_entry(link_index.root_index, link.clone(), link_index.tag)?;
            } else {
                //Create basic index (link) which links from Path() entry -> link_index.tag -> LinkExpression 
                let link_hash = hash_entry(&link)?;
                let path_source = Path::from(link_index.root_index);
                path_source.ensure()?;
                create_link(path_source.hash()?, link_hash.clone(), link_index.tag)?;
            };
        }
        Ok(())
    }

    pub fn get_links(mut get_links: GetLinks) -> SocialContextResult<Vec<LinkExpression>> {
        let wildcard = get_wildcard();
        //No elements were supplied in the triple so we use wildcards as source + predicate to simulate a getAllLinks query 
        //(note for this to work the FullWithWildCard index needs to be enabled)
        if  get_links.triple.num_entities() == 0 {
            get_links.triple.source = Some(wildcard.to_string());
            get_links.triple.predicate = Some(wildcard.to_string());
        };

        //Derive the source link index value + link tag value to query with based on the values passed in GetLinks.triple
        //Note we are only looking for two or one elements in the triple since if you have three you already have the LinkExpression! 
        let link_query_elements = if get_links.triple.source.is_some() {
            if get_links.triple.target.is_some() {
                //Query with source + target; will match all LinkExpression with same source + target
                //In this case the predicate unknown here and thus the value zome caller is interested in
                LinkPermutation::new(
                    get_links.triple.source.unwrap(),
                    get_links.triple.target.unwrap(),
                )
            } else if get_links.triple.predicate.is_some() {
                //Query with source + predicate
                //Here target is unknown and thus the value the zome caller is looking for
                LinkPermutation::new(
                    get_links.triple.source.unwrap(),
                    get_links.triple.predicate.unwrap(),
                )
            } else {
                //Look for all links with the given source
                LinkPermutation::new(get_links.triple.source.unwrap(), wildcard)
            }
        } else if get_links.triple.target.is_some() {
            if get_links.triple.predicate.is_some() {
                LinkPermutation::new(
                    get_links.triple.target.unwrap(),
                    get_links.triple.predicate.unwrap(),
                )
            } else {
                LinkPermutation::new(get_links.triple.target.unwrap(), wildcard)
            }
        } else {
            LinkPermutation::new(get_links.triple.predicate.unwrap(), wildcard)
        };

        //TODO: this should be specified by the zome caller and not in DNA props
        if *ENABLE_TIME_INDEX {
            //If fromDate & untilDate have been supplied then call hc_time_index looking for LinkExpression(s) in date range with given root_index + tag
            if get_links.from_date.is_some() && get_links.until_date.is_some() {
                Ok(hc_time_index::get_links_and_load_for_time_span::<
                    LinkExpression,
                >(
                    link_query_elements.root_index,
                    get_links.from_date.unwrap(),
                    get_links.until_date.unwrap(),
                    Some(link_query_elements.tag),
                    SearchStrategy::Dfs,
                    Some(get_links.limit),
                )?)
            } else {
                //fromDate & untilDate not supplied so we will try to get all LinkExpression(s) from now -> unix epoch
                //This will return all links since the hc_time_index crate does not support indexing before unix epoch currently
                let now = sys_time()?;
                let now = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(now.0, now.1),
                    Utc,
                );
                let unix = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(0, 0),
                    Utc,
                );
                Ok(hc_time_index::get_links_and_load_for_time_span::<
                    LinkExpression,
                >(
                    link_query_elements.root_index,
                    unix,
                    now,
                    Some(link_query_elements.tag),
                    SearchStrategy::Bfs,
                    None,
                )?)
            }
        } else {
            //Time index not enabled so just make a simple query
            SocialContextDNA::make_simple_link_query(Path::from(link_query_elements.root_index).hash()?, Some(link_query_elements.tag))
        }
    }

    fn make_simple_link_query(
        base: EntryHash,
        link_tag: Option<LinkTag>,
    ) -> SocialContextResult<Vec<LinkExpression>> {
        Ok(hdk::link::get_links(base, link_tag)?
            .into_inner()
            .into_iter()
            .map(|link| match get(link.target, GetOptions::latest())? {
                Some(chunk) => Ok(Some(
                    chunk.entry().to_app_option::<LinkExpression>()?.ok_or(
                        SocialContextError::InternalError(
                            "Expected element to contain app entry data",
                        ),
                    )?,
                )),
                None => Ok(None),
            })
            .filter_map(|val| {
                if val.is_ok() {
                    let val = val.unwrap();
                    if val.is_some() {
                        Some(Ok(val.unwrap()))
                    } else {
                        None
                    }
                } else {
                    Some(Err(val.err().unwrap()))
                }
            })
            .collect::<SocialContextResult<Vec<LinkExpression>>>()?)
    }

    pub fn get_others() -> SocialContextResult<Vec<String>> {
        Ok(vec![])
    }

    pub fn remove_link(link: LinkExpression) -> SocialContextResult<()> {
        //Get the LinkExpression entry to be deleted
        let entry =
            get(link.hash()?, GetOptions::latest())?.ok_or(SocialContextError::RequestError(
                "Could not find link expression that was requested for deletion",
            ))?;

        if *ENABLE_TIME_INDEX {
            //TODO: check that the deletion here is exhastive and deletes all index permutations
            hc_time_index::remove_index(link.hash()?)?;
        } else {
            //Generate the link indexes that are possible for this LinkExpression
            let link_indexes = generate_link_path_permutations(&link.data)?;
            let link_hash = link.hash()?;
            //For each permutation get links on source and if exists then delete where target of link == target LinkExpression to be deleted
            for link_index in link_indexes {
                let path_source = Path::from(link_index.root_index);
                hdk::link::get_links(path_source.hash()?, Some(link_index.tag))?
                    .into_inner()
                    .into_iter()
                    .filter(|link| link.target == link_hash)
                    .map(|val| {
                        delete_link(val.create_link_hash.to_owned())?;
                        Ok(())
                    })
                    .collect::<SocialContextResult<()>>()?;
            }
        }
        delete_entry(entry.header_address().to_owned())?;
        Ok(())
    }

    pub fn update_link(update_link: UpdateLink) -> SocialContextResult<()> {
        SocialContextDNA::remove_link(update_link.source)?;
        SocialContextDNA::add_link(update_link.target)?;
        Ok(())
    }
}
