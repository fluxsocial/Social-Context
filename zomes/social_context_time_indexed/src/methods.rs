use chrono::{DateTime, Utc};
use hc_time_index::IndexableEntry;
use hdk3::prelude::*;

use crate::{Agent, GetLinks, LinkExpression, SocialContextDNA, UpdateLink};

impl IndexableEntry for LinkExpression {
    fn entry_time(&self) -> DateTime<Utc> {
        self.timestamp.to_owned()
    }

    fn hash(&self) -> ExternResult<EntryHash> {
        hash_entry(self)
    }
}

impl SocialContextDNA {
    pub fn add_link(link: LinkExpression) -> ExternResult<()> {
        let link_indexes = generate_link_path_permutations(&link)?;

        for link_index in link_indexes {
            let (source, tag) = link_index;
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

pub fn generate_link_path_permutations(
    link: &LinkExpression,
) -> ExternResult<Vec<(String, String)>> {
    let num_entities = link.data.num_entities();
    let mut out = vec![];
    let wildcard = String::from("*");

    if num_entities == 0 {
        Err(WasmError::Zome(String::from("Link has no entities")))
    } else if num_entities == 3 {
        let subject = link.data.subject.clone().unwrap();
        let object = link.data.object.clone().unwrap();
        let predicate = link.data.predicate.clone().unwrap();
        //Subject -> * -> LinkExpression
        out.push((subject.clone(), wildcard.clone()));
        //Object -> * -> LinkExpression
        out.push((object.clone(), wildcard.clone()));
        //Predicate -> * -> LinkExpression
        out.push((predicate.clone(), wildcard.clone()));

        //Subject object -> * -> LinkExpression
        out.push((format!("{}.{}", subject, object), wildcard.clone()));
        //Subject predicate -> * -> LinkExpression
        out.push((format!("{}.{}", subject, predicate), wildcard.clone()));
        //Object predicate -> * -> LinkExpression
        out.push((format!("{}.{}", object, predicate), wildcard));
        Ok(out)
    } else if num_entities == 2 {
        if link.data.subject.is_some() {
            if link.data.object.is_some() {
                let subject = link.data.subject.clone().unwrap();
                let object = link.data.object.clone().unwrap();
                //Subject object -> wildcard -> LinkExpression
                out.push((format!("{}.{}", subject, object), wildcard.clone()));

                //Subject -> wildcard -> LinkExpression
                out.push((subject, wildcard.clone()));

                //Object -> wildcard -> LinkExpression
                out.push((object, wildcard));
            } else {
                let subject = link.data.subject.clone().unwrap();
                let predicate = link.data.predicate.clone().unwrap();
                //Subject predicate -> wildcard -> LinkExpression
                out.push((format!("{}.{}", subject, predicate), wildcard.clone()));

                //Subject -> wildcard -> LinkExpression
                out.push((subject, wildcard.clone()));

                //Predicate -> wildcard -> LinkExpression
                out.push((predicate, wildcard));
            };
        } else if link.data.object.is_some() {
            let object = link.data.object.clone().unwrap();
            let predicate = link.data.predicate.clone().unwrap();
            //Object, predicate -> wildcard -> LinkExpression
            out.push((format!("{}.{}", object, predicate), wildcard.clone()));
            //Object -> * -> LinkExpression
            out.push((object, wildcard.clone()));
            //Predicate -> * -> LinkExpression
            out.push((predicate, wildcard));
        } else {
            unreachable!()
        };
        Ok(out)
    } else if link.data.subject.is_some() {
        let subject = link.data.subject.clone().unwrap();
        //Subject -> * -> LinkExpression
        out.push((subject, wildcard));
        Ok(out)
    } else if link.data.object.is_some() {
        let object = link.data.object.clone().unwrap();
        //Object -> * -> LinkExpression
        out.push((object, wildcard));
        Ok(out)
    } else {
        let predicate = link.data.predicate.clone().unwrap();
        //Predicate -> * -> LinkExpression
        out.push((predicate, wildcard));
        Ok(out)
    }
}
