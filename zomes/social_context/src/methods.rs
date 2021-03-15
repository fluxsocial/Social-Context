use hdk3::prelude::*;

use crate::{Agent, GetLinks, LinkExpression, SocialContextDNA, UpdateLink};
use crate::utils::generate_link_path_permutations;

impl SocialContextDNA {
    pub fn add_link(link: LinkExpression) -> ExternResult<()> {
        debug!("Adding a link");
        create_entry(&link)?;
        let link_indexes = generate_link_path_permutations(&link)?;

        for link_index in link_indexes {
            let (source, tag) = link_index;
            debug!("Creating link index for source: {:?}", source);
            hc_time_index::index_entry(source, link.clone(), LinkTag::new(tag))?;
        }

        Ok(())
    }

    pub fn get_links(get_links: GetLinks) -> ExternResult<Vec<LinkExpression>> {
        let num_entities = get_links.triple.num_entities();
        if num_entities == 0 {
            return Err(WasmError::Zome(String::from("Link has no entities")));
        } else if num_entities == 3 {
            return Err(WasmError::Zome(String::from(
                "You already have all the entities",
            )));
        };

        let start = if get_links.triple.subject.is_some() {
            if get_links.triple.object.is_some() {
                format!("{}.{}", get_links.triple.subject.unwrap(), get_links.triple.object.unwrap())
            } else if get_links.triple.predicate.is_some() {
                format!("{}.{}", get_links.triple.subject.unwrap(), get_links.triple.predicate.unwrap())
            } else {
                get_links.triple.subject.unwrap()
            }
        } else if get_links.triple.object.is_some() {
            if get_links.triple.predicate.is_some() {
                format!("{}.{}", get_links.triple.object.unwrap(), get_links.triple.predicate.unwrap())
            } else {
                get_links.triple.object.unwrap()
            }
        } else {
            get_links.triple.predicate.unwrap()
        };

        Ok(hc_time_index::get_links_and_load_for_time_span::<LinkExpression>(
            start,
            get_links.from,
            get_links.until,
            None,
            Some(LinkTag::new("*")),
        )?)
    }

    pub fn get_others() -> ExternResult<Vec<Agent>> {
        Ok(vec![])
    }

    pub fn remove_link(_link: LinkExpression) -> ExternResult<()> {
        Ok(())
    }

    //Right now this solution is pretty basic and opts for just deleting the source link and then creating the second
    //ideally here we could dynamically update links between source, object, predicate -> new link object where overlap occurs
    pub fn update_link(_update_link: UpdateLink) -> ExternResult<()> {
        Ok(())
    }
}
