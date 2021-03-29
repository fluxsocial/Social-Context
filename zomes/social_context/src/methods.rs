use chrono::Utc;
use hc_time_index::IndexableEntry;
use hdk::prelude::*;
use holo_hash::error::HoloHashResult;

use crate::errors::{SocialContextError, SocialContextResult};
use crate::utils::generate_link_path_permutations;
use crate::{
    Agent, GetLinks, LinkExpression, SocialContextDNA, UpdateLink, ACTIVE_AGENT_DURATION,
};

impl SocialContextDNA {
    pub fn add_link(link: LinkExpression) -> SocialContextResult<()> {
        let link_indexes = generate_link_path_permutations(&link)?;
        create_entry(&link)?;

        for link_index in link_indexes {
            let (source, tag) = link_index;
            hc_time_index::index_entry(source, link.clone(), LinkTag::new(tag))?;
        }

        //Here we should get link on some "chatters" index so we can find active agents and try to emit_signal
        //NOTE: when adding a link on active_agent index it should be validated that source is active_agent and target is agent address
        //and validated that agent address is matching committing agent
        let recent_agents = hc_time_index::get_links_and_load_for_time_span::<LinkExpression>(
            String::from("active_agent"), Utc::now() - *ACTIVE_AGENT_DURATION, Utc::now(), None, None)?
            .into_iter()
            .map(|val| Ok(AgentPubKey::try_from(val.data.object.expect("Object for active agent subject should never be none"))?))
            .collect::<HoloHashResult<Vec<AgentPubKey>>>().expect("Unwrapping here until we upgrade holo_hash version where std err is integrated to error type");
        remote_signal(link.get_sb()?, recent_agents)?;

        Ok(())
    }

    pub fn get_links(get_links: GetLinks) -> SocialContextResult<Vec<LinkExpression>> {
        let num_entities = get_links.triple.num_entities();
        if num_entities == 0 {
            return Err(SocialContextError::RequestError("Link has no entities"));
        };

        let (index, lt) = if get_links.triple.subject.is_some() {
            if get_links.triple.object.is_some() {
                (get_links.triple.subject.unwrap(),
                    LinkTag::new(get_links.triple.object.unwrap()))
            } else if get_links.triple.predicate.is_some() {
                (get_links.triple.subject.unwrap(),
                    LinkTag::new(get_links.triple.predicate.unwrap()))
            } else {
                (get_links.triple.subject.unwrap(), LinkTag::new("*"))
            }
        } else if get_links.triple.object.is_some() {
            if get_links.triple.predicate.is_some() {
                (get_links.triple.object.unwrap(),
                    LinkTag::new(get_links.triple.predicate.unwrap()))
            } else {
                (get_links.triple.object.unwrap(), LinkTag::new("*"))
            }
        } else {
            (get_links.triple.predicate.unwrap(), LinkTag::new("*"))
        };

        Ok(hc_time_index::get_links_and_load_for_time_span::<
            LinkExpression,
        >(
            index,
            get_links.from,
            get_links.until,
            None,
            Some(lt),
        )?)
    }

    pub fn get_others() -> SocialContextResult<Vec<Agent>> {
        Ok(vec![])
    }

    //Pretty basic delete as it just removes link from index tree and then removes entry itself.
    //Reminants of the link will still exist in the index tree as indexes are created for each element of triple.
    pub fn remove_link(link: LinkExpression) -> SocialContextResult<()> {
        let entry =
            get(link.hash()?, GetOptions::latest())?.ok_or(SocialContextError::RequestError(
                "Could not find link expression that was requested for deletion",
            ))?;
        hc_time_index::remove_index(link.hash()?)?;
        delete_entry(entry.header_address().to_owned())?;
        Ok(())
    }

    //Right now this solution is pretty basic and opts for just deleting the source link and then creating the second
    //ideally here we could dynamically update links between source, object, predicate -> new link object where overlap occurs
    pub fn update_link(update_link: UpdateLink) -> SocialContextResult<()> {
        SocialContextDNA::remove_link(update_link.source)?;
        SocialContextDNA::add_link(update_link.target)?;
        Ok(())
    }
}
