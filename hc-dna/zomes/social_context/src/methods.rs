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
                NaiveDateTime::from_timestamp(now.as_secs_f64() as i64, now.subsec_nanos()),
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
            NaiveDateTime::from_timestamp(now.as_secs_f64() as i64, now.subsec_nanos()),
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
            IndexStrategy::Full => generate_link_path_permutations(&link)?,
            IndexStrategy::FullWithWildCard => {
                let mut perm = generate_link_path_permutations(&link)?;
                let wildcard = get_wildcard();
                perm.push(LinkPermutation::new(wildcard.to_string(), wildcard.to_string()));
                perm
            }
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

        for link_index in link_indexes {
            if *ENABLE_TIME_INDEX {
                hc_time_index::index_entry(link_index.source, link.clone(), link_index.tag)?;
            } else {
                let link_hash = hash_entry(&link)?;
                let path_source = Path::from(link_index.source);
                path_source.ensure()?;
                create_link(path_source.hash()?, link_hash.clone(), link_index.tag)?;
            };
        }
        Ok(())
    }

    pub fn get_links(mut get_links: GetLinks) -> SocialContextResult<Vec<LinkExpression>> {
        let num_entities = get_links.triple.num_entities();
        let wildcard = get_wildcard();
        if num_entities == 0 {
            get_links.triple.source = Some(wildcard.to_string());
            get_links.triple.predicate = Some(wildcard.to_string());
        };

        let link_query_elements = if get_links.triple.source.is_some() {
            if get_links.triple.target.is_some() {
                LinkPermutation::new(
                    get_links.triple.source.unwrap(),
                    get_links.triple.target.unwrap(),
                )
            } else if get_links.triple.predicate.is_some() {
                LinkPermutation::new(
                    get_links.triple.source.unwrap(),
                    get_links.triple.predicate.unwrap(),
                )
            } else {
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

        if *ENABLE_TIME_INDEX {
            if get_links.from_date.is_some() && get_links.until_date.is_some() {
                Ok(hc_time_index::get_links_and_load_for_time_span::<
                    LinkExpression,
                >(
                    link_query_elements.source,
                    get_links.from_date.unwrap(),
                    get_links.until_date.unwrap(),
                    Some(link_query_elements.tag),
                    SearchStrategy::Dfs,
                    Some(get_links.limit),
                )?)
            } else {
                let now = sys_time()?;
                let now = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(now.as_secs_f64() as i64, now.subsec_nanos()),
                    Utc,
                );
                let unix = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(0, 0),
                    Utc,
                );
                Ok(hc_time_index::get_links_and_load_for_time_span::<
                    LinkExpression,
                >(
                    link_query_elements.source,
                    unix,
                    now,
                    Some(link_query_elements.tag),
                    SearchStrategy::Bfs,
                    None,
                )?)
            }
        } else {
            SocialContextDNA::make_simple_link_query(Path::from(link_query_elements.source).hash()?, Some(link_query_elements.tag))
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

    //Pretty basic delete as it just removes link from index tree and then removes entry itself.
    //Remnants of the link will still exist in the index tree as indexes are created for each element of triple.
    //TODO: need another method on time_index where we can delete from index where target entry of index == some value
    pub fn remove_link(link: LinkExpression) -> SocialContextResult<()> {
        let entry =
            get(link.hash()?, GetOptions::latest())?.ok_or(SocialContextError::RequestError(
                "Could not find link expression that was requested for deletion",
            ))?;

        if *ENABLE_TIME_INDEX {
            hc_time_index::remove_index(link.hash()?)?;
        } else {
            let link_indexes = generate_link_path_permutations(&link)?;
            let link_hash = link.hash()?;
            for link_index in link_indexes {
                let path_source = Path::from(link_index.source);
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

    //Right now this solution is pretty basic and opts for just deleting the source link and then creating the second
    //ideally here we could dynamically update links between source, target, predicate -> new link target where overlap occurs
    pub fn update_link(update_link: UpdateLink) -> SocialContextResult<()> {
        SocialContextDNA::remove_link(update_link.source)?;
        SocialContextDNA::add_link(update_link.target)?;
        Ok(())
    }
}
