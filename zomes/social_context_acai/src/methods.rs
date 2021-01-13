use hdk3::{hash_path::path::Component, prelude::*};

use crate::{Agent, LinkExpression, SocialContextDNA, Triple};

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

        let link_indexes = generate_link_path_permutations(link)?;

        for link_index in link_indexes {
            let (source, tag, target) = link_index;
            let target_hash = hash_entry(&target)?;
            create_entry(&target)?;
            create_entry(&source)?;

            create_link(
                source.hash()?,
                target_hash,
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
            let subject = triple.subject.unwrap();
            if triple.object.is_some() {
                (
                    Path::from(vec![
                        Component::from(subject),
                        Component::from(triple.object.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else if triple.predicate.is_some() {
                (
                    Path::from(vec![
                        Component::from(subject),
                        Component::from(triple.predicate.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else {
                (
                    Path::from(vec![Component::from(subject)]),
                    Component::from("*"),
                )
            }
        } else if triple.object.is_some() {
            let object = triple.object.unwrap();
            if triple.predicate.is_some() {
                (
                    Path::from(vec![
                        Component::from(object),
                        Component::from(triple.predicate.unwrap()),
                    ]),
                    Component::from("*"),
                )
            } else {
                (
                    Path::from(vec![Component::from(object)]),
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
            let target = get(link.target, GetOptions::default())?.ok_or(HdkError::Wasm(
                WasmError::Zome(String::from("Could not find target for link")),
            ))?;
            let acai_exp_data: LinkExpression =
                target
                    .entry()
                    .to_app_option()?
                    .ok_or(HdkError::Wasm(WasmError::Zome(String::from(
                        "Could not deserialize link expression data into LinkAcaiData",
                    ))))?;

            Ok(acai_exp_data)
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
                let did_agent = get(link.target, GetOptions::default())?.ok_or(HdkError::Wasm(
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
}

pub fn generate_link_path_permutations(
    link: LinkExpression,
) -> ExternResult<Vec<(Path, Path, LinkExpression)>> {
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
            link.clone(),
        ));
        //Object -> * -> LinkExpression
        out.push((
            Path::from(vec![object.clone()]),
            Path::from(vec![wildcard.clone()]),
            link.clone(),
        ));
        //Predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![predicate.clone()]),
            Path::from(vec![wildcard.clone()]),
            link.clone(),
        ));

        //Subject object -> * -> LinkExpression
        out.push((
            Path::from(vec![subject.clone(), object.clone()]),
            Path::from(vec![wildcard.clone()]),
            link.clone(),
        ));
        //Subject predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![subject, predicate.clone()]),
            Path::from(vec![wildcard.clone()]),
            link.clone(),
        ));
        //Object predicate -> * -> LinkExpression
        out.push((
            Path::from(vec![object, predicate]),
            Path::from(vec![wildcard]),
            link,
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
                    link.clone(),
                ));

                //Subject -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject]),
                    Path::from(vec![wildcard.clone()]),
                    link.clone(),
                ));

                //Object -> wildcard -> LinkExpression
                out.push((Path::from(vec![object]), Path::from(vec![wildcard]), link));
            } else {
                let subject = Component::from(link.data.subject.clone().unwrap());
                let predicate = Component::from(link.data.predicate.clone().unwrap());
                //Subject predicate -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject.clone(), predicate.clone()]),
                    Path::from(vec![wildcard.clone()]),
                    link.clone(),
                ));

                //Subject -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![subject]),
                    Path::from(vec![wildcard.clone()]),
                    link.clone(),
                ));

                //Predicate -> wildcard -> LinkExpression
                out.push((
                    Path::from(vec![predicate]),
                    Path::from(vec![wildcard]),
                    link,
                ));
            };
        } else if link.data.object.is_some() {
            let object = Component::from(link.data.object.clone().unwrap());
            let predicate = Component::from(link.data.predicate.clone().unwrap());
            //Object, predicate -> wildcard -> LinkExpression
            out.push((
                Path::from(vec![object.clone(), predicate.clone()]),
                Path::from(vec![wildcard.clone()]),
                link.clone(),
            ));

            //Object -> wildcard -> LinkExpression
            out.push((
                Path::from(vec![object]),
                Path::from(vec![wildcard.clone()]),
                link.clone(),
            ));

            //Predicate -> wildcard -> LinkExpression
            out.push((
                Path::from(vec![predicate]),
                Path::from(vec![wildcard]),
                link,
            ));
        } else {
            unreachable!()
        };
        Ok(out)
    } else if link.data.subject.is_some() {
        let subject = Component::from(link.data.subject.clone().unwrap());
        out.push((Path::from(vec![subject]), Path::from(vec![wildcard]), link));
        Ok(out)
    } else if link.data.object.is_some() {
        let object = Component::from(link.data.object.clone().unwrap());
        out.push((Path::from(vec![object]), Path::from(vec![wildcard]), link));
        Ok(out)
    } else {
        let predicate = Component::from(link.data.predicate.clone().unwrap());
        out.push((
            Path::from(vec![predicate]),
            Path::from(vec![wildcard]),
            link,
        ));
        Ok(out)
    }
}
