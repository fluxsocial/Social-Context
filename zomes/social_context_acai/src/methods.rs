use hdk3::{hash_path::path::Component, prelude::*};

use crate::{Agent, LinkExpression, SocialContextDNA, Triple, UpdateLink, add_link, remove_link};

impl SocialContextDNA {
    pub fn add_link(link: LinkExpression) -> ExternResult<()> {
        //TODO this should use chunking mixin in the future; same with links above
        let user_anchor = Anchor {
            anchor_text: None,
            anchor_type: String::from("users"),
        };
        let did_agent = link.author.clone();
        create_entry(&user_anchor)?;
        create_entry(&did_agent)?;
        create_link(
            hash_entry(&user_anchor)?,
            hash_entry(&did_agent)?,
            LinkTag::new(link.author.did.clone()),
        )?;

        let link_indexes = generate_link_path_permutations(&link)?;
        let target_hash = hash_entry(&link)?;
        create_entry(&link)?;

        for link_index in link_indexes {
            let (source, tag) = link_index;
            create_entry(&source)?;

            create_link(
                source.hash()?,
                target_hash.clone(),
                LinkTag::new(tag.hash()?.to_string()),
            )?;
        }

        Ok(())
    }

    pub fn get_links(triple: Triple) -> ExternResult<Vec<LinkExpression>> {
        let num_entities = triple.num_entities();
        if num_entities == 0 {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "Link has no entities",
            ))));
        } else if num_entities == 3 {
            return Err(HdkError::Wasm(WasmError::Zome(String::from(
                "You already have all the entities",
            ))));
        };

        let (start, tag) = if triple.subject.is_some() {
            if triple.object.is_some() {
                (
                    Path::from(vec![
                        Component::from(triple.subject.unwrap()),
                        Component::from(triple.object.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else if triple.predicate.is_some() {
                (
                    Path::from(vec![
                        Component::from(triple.subject.unwrap()),
                        Component::from(triple.predicate.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else {
                (
                    Path::from(vec![Component::from(triple.subject.unwrap())]),
                    Component::from("*"),
                )
            }
        } else if triple.object.is_some() {
            if triple.predicate.is_some() {
                (
                    Path::from(vec![
                        Component::from(triple.object.unwrap()),
                        Component::from(triple.predicate.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else {
                (
                    Path::from(vec![Component::from(triple.object.unwrap())]),
                    Component::from("*"),
                )
            }
        } else {
            (
                Path::from(vec![Component::from(triple.predicate.unwrap())]),
                Component::from("*"),
            )
        };

        Ok(get_links(
            hash_entry(&start)?,
            Some(LinkTag::new(Path::from(vec![tag]).hash()?.to_string())),
        )?
        .into_inner()
        .into_iter()
        .map(|link| {
            match get(link.target, GetOptions::content())? {
                Some(entry) => {
                    let acai_exp_data: LinkExpression =
                        entry
                            .entry()
                            .to_app_option()?
                            .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                                "Could not deserialize link expression data into LinkAcaiData",
                            ))))?;
    
                    Ok(Some(acai_exp_data))
                },
                None => Ok(None)
            }
        })
        .filter(|val| {
            match val {
                Ok(val) => val.is_some(),
                Err(_err) => true
            }
        })
        .map(|val| {
            if val.is_ok() {
                Ok(val.unwrap().unwrap())
            } else {
                Err(val.err().unwrap())
            }
        })
        .collect::<ExternResult<Vec<LinkExpression>>>()?)
    }

    pub fn get_others() -> ExternResult<Vec<Agent>> {
        let user_anchor = Anchor {
            anchor_text: None,
            anchor_type: String::from("users"),
        };
        Ok(get_links(hash_entry(&user_anchor)?, None)?
            .into_inner()
            .into_iter()
            .map(|link| {
                let did_agent = get(link.target, GetOptions::content())?.ok_or(HdkError::Wasm(
                    WasmError::Zome(String::from("Could not find target for link")),
                ))?;
                let did_agent: Agent =
                    did_agent
                        .entry()
                        .to_app_option()?
                        .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                            "Could not deserialize link expression data into LinkAcaiData",
                        ))))?;

                Ok(did_agent)
            })
            .collect::<ExternResult<Vec<Agent>>>()?)
    }

    pub fn remove_link(link: LinkExpression) -> ExternResult<()> {
        let link_indexes = generate_link_path_permutations(&link)?;

        let source_hash = hash_entry(&link)?;
        let source_entry = get(source_hash.clone(), GetOptions::default())?.ok_or(
            HdkError::Wasm(WasmError::Zome(String::from("Source link does not exist"))),
        )?;
        let source_header_hash = source_entry.signed_header().as_hash();
        delete_entry(source_header_hash.to_owned())?;

        for link_index in link_indexes {
            let (source, tag) = link_index;

            let link_source_hash = hash_entry(&source)?;
            let links = get_links(
                link_source_hash.clone(),
                Some(LinkTag::new(tag.hash()?.to_string())),
            )?;
            let link_delete = links
                .into_inner()
                .into_iter()
                .filter(|link| link.target == source_hash)
                .collect::<Vec<_>>();
            debug!("Found {} links to delete", link_delete.len());
            let link_hash = link_delete
                .first()
                .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                    "No link to delete for source",
                ))))?
                .to_owned()
                .create_link_hash;

            delete_link(link_hash)?;
        };

        Ok(())
    }

    //Right now this solution is pretty basic and opts for just deleting the source link and then creating the second
    //ideally here we could dynamically update links between source, object, predicate -> new link object where overlap occurs
    pub fn update_link(update_link: UpdateLink) -> ExternResult<()> {
        remove_link(update_link.source)?;
        add_link(update_link.target)?;

        Ok(())
    }
}

pub fn generate_link_path_permutations(link: &LinkExpression) -> ExternResult<Vec<(Path, Path)>> {
    let num_entities = link.data.num_entities();
    let mut out = vec![];
    let wildcard = Component::from("*");

    if num_entities == 0 {
        Err(HdkError::Wasm(WasmError::Zome(String::from(
            "Link has no entities",
        ))))
    } else if num_entities == 3 {
        let subject = Component::from(link.data.subject.clone().unwrap());
        let object = Component::from(link.data.object.clone().unwrap());
        let predicate = Component::from(link.data.predicate.clone().unwrap());
        //Subject -> * -> LinkExpression
        out.push((
            Path::from(vec![subject.clone()]),
            Path::from(vec![wildcard.clone()]),
        ));
        //Object -> * -> LinkExpression
        out.push((
            Path::from(vec![object.clone()]),
            Path::from(vec![wildcard.clone()]),
        ));
        //Predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![predicate.clone()]),
            Path::from(vec![wildcard.clone()]),
        ));

        //Subject object -> * -> LinkExpression
        out.push((
            Path::from(vec![subject.clone(), object.clone()]),
            Path::from(vec![wildcard.clone()]),
        ));
        //Subject predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![subject, predicate.clone()]),
            Path::from(vec![wildcard.clone()]),
        ));
        //Object predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![object, predicate]),
            Path::from(vec![wildcard]),
        ));
        Ok(out)
    } else if num_entities == 2 {
        if link.data.subject.is_some() {
            if link.data.object.is_some() {
                let subject = Component::from(link.data.subject.clone().unwrap());
                let object = Component::from(link.data.object.clone().unwrap());
                //Subject object -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject.clone(), object.clone()]),
                    Path::from(vec![wildcard.clone()]),
                ));

                //Subject -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject]),
                    Path::from(vec![wildcard.clone()]),
                ));

                //Object -> wildcard -> LinkExpression
                out.push((Path::from(vec![object]), Path::from(vec![wildcard])));
            } else {
                let subject = Component::from(link.data.subject.clone().unwrap());
                let predicate = Component::from(link.data.predicate.clone().unwrap());
                //Subject predicate -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject.clone(), predicate.clone()]),
                    Path::from(vec![wildcard.clone()]),
                ));

                //Subject -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject]),
                    Path::from(vec![wildcard.clone()]),
                ));

                //Predicate -> wildcard -> LinkExpression
                out.push((Path::from(vec![predicate]), Path::from(vec![wildcard])));
            };
        } else if link.data.object.is_some() {
            let object = Component::from(link.data.object.clone().unwrap());
            let predicate = Component::from(link.data.predicate.clone().unwrap());
            //Object, predicate -> wildcard -> LinkExpression
            out.push((
                Path::from(vec![object.clone(), predicate.clone()]),
                Path::from(vec![wildcard.clone()]),
            ));

            //Object -> wildcard -> LinkExpression
            out.push((Path::from(vec![object]), Path::from(vec![wildcard.clone()])));

            //Predicate -> wildcard -> LinkExpression
            out.push((Path::from(vec![predicate]), Path::from(vec![wildcard])));
        } else {
            unreachable!()
        };
        Ok(out)
    } else if link.data.subject.is_some() {
        let subject = Component::from(link.data.subject.clone().unwrap());
        out.push((Path::from(vec![subject]), Path::from(vec![wildcard])));
        Ok(out)
    } else if link.data.object.is_some() {
        let object = Component::from(link.data.object.clone().unwrap());
        out.push((Path::from(vec![object]), Path::from(vec![wildcard])));
        Ok(out)
    } else {
        let predicate = Component::from(link.data.predicate.clone().unwrap());
        out.push((Path::from(vec![predicate]), Path::from(vec![wildcard])));
        Ok(out)
    }
}
