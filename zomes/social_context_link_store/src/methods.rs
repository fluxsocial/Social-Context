use hdk3::{hash_path::path::Component, prelude::*};

use crate::{AcaiUrl, SocialContextDNA, Triple, TripleParsed, SOFT_CHUNK_LIMIT};

impl SocialContextDNA {
    pub fn add_link(link: TripleParsed) -> ExternResult<()> {
        //Put predicate back into string
        let predicate_url = link
            .predicate
            .map(|pred| format!("{}", pred))
            .unwrap_or(String::from(""));

        //Create links between subjects language and expression
        create_bidir_chunked_links(
            &format!("{}", link.subject),
            &format!("{}", link.object),
            &predicate_url,
        )?;
        Ok(())
    }

    pub fn get_links(subject: String, predicate: Option<String>) -> ExternResult<Vec<Triple>> {
        let subject_acai = AcaiUrl::try_from(subject.clone())?;

        let subject_path = if subject_acai.language != "chunk" {
            Path::from(subject.clone())
        } else {
            //Parse the expression to create the correct chunk shape
            let split: Vec<&str> = subject_acai
                .expression
                .split("?chunk=")
                .into_iter()
                .collect();
            Path::from(vec![Component::from(split[0]), Component::from(split[1])])
        };
        let links = get_links(
            subject_path.hash()?,
            predicate.clone().map(|pred| LinkTag::new(pred)),
        )?;

        Ok(links
            .into_inner()
            .into_iter()
            .map(|link| {
                let element = get(link.target.clone(), GetOptions::default())?.ok_or(
                    HdkError::Wasm(WasmError::Zome(format!(
                        "Could not get entry for link with target: {}",
                        link.target
                    ))),
                )?;

                let path: Vec<Component> = try_from_entry::<Path>(
                    element
                        .entry()
                        .as_option()
                        .ok_or(HdkError::Wasm(WasmError::Zome(format!(
                            "Could not get entry for link with target: {}",
                            link.target
                        ))))?
                        .to_owned(),
                )?
                .into();
                Ok(Triple {
                    subject: Some(subject.clone()),
                    object: Some(if path.len() == 1 {
                        String::try_from(&path[0]).unwrap()
                    } else if path.len() == 2 {
                        format!(
                            "chunk://{}",
                            format!(
                                "{}?chunk={}",
                                String::try_from(&path[0])?,
                                String::try_from(&path[1])?
                            )
                        )
                    } else {
                        return Err(HdkError::Wasm(WasmError::Zome(
                            "Got path with more than two elements".to_string(),
                        )));
                    }),
                    predicate: predicate.clone(),
                })
            })
            .collect::<ExternResult<Vec<Triple>>>()?)
    }

    pub fn add_link_auto_index(link: TripleParsed) -> ExternResult<()> {
        //Put predicate back into string
        let predicate_url = link
            .predicate
            .map(|pred| format!("{}", pred))
            .unwrap_or(String::from(""));

        //Unwrap the subject and object
        let subject_url = format!("{}", link.subject);
        let subject_language = format!("{}://{}", link.subject.language, link.subject.language);
        let object_url = format!("{}", link.object);
        let object_language = format!("{}://{}", link.object.language, link.object.language);
        let agent_url = format!("hc-agent://{}", agent_info()?.agent_latest_pubkey);

        //Create link between subject and object. Subject relates to object.
        create_bidir_chunked_links(&subject_url, &object_url, &predicate_url)?;

        //Create link between subject and object lang; i.e this subject relates to this language
        create_bidir_chunked_links(
            &subject_url,
            &object_language,
            &predicate_url,
        )?;

        //Create link between object and subject lang; i.e this object is related to subjects language
        create_bidir_chunked_links(
            &object_url,
            &subject_language,
            &predicate_url,
        )?;

        //Create link between subject and agent; i.e this agent is doing something on this subject
        create_bidir_chunked_links(
            &subject_url,
            &agent_url,
            &predicate_url,
        )?;

        //Create link between agent and object; i.e agent is communicating on/with this object
        create_bidir_chunked_links(
            &object_url,
            &agent_url,
            &predicate_url,
        )?;

        //Create link between object language and object expression; i.e this language has given expression
        create_bidir_chunked_links(
            &object_language,
            &object_url,
            &predicate_url,
        )?;

        //Create link between subject language and subject expression; i.e this language has given expression
        create_bidir_chunked_links(
            &subject_language,
            &subject_url,
            &predicate_url,
        )?;

        //Create link between agent and object language; i.e agent is communicating on this object language
        create_bidir_chunked_links(
            &agent_url,
            &object_language,
            &predicate_url,
        )?;

        //Create link between agent and subject language; i.e agent is communicating on this subject language
        create_bidir_chunked_links(
            &agent_url,
            &subject_language,
            &predicate_url,
        )?;
        
        //Create link between object lang and object
        create_bidir_chunked_links(
            &object_language,
            &object_url,
            &predicate_url,
        )?;

        //Create link between subject lang and subject
        create_bidir_chunked_links(
            &subject_language,
            &subject_url,
            &predicate_url,
        )?;

        Ok(())
    }
}

pub fn create_bidir_chunked_links(
    source: &String,
    target: &String,
    predicate: &String,
) -> ExternResult<()> {
    //Get a free chunk for source
    let source_path = Path::from(source);
    source_path.ensure()?;

    let chunk = get_free_chunk(&source_path)?;

    //Should now path of form [lang, foundchunk]
    let source_path = add_chunk_path(source_path, chunk);
    source_path.ensure()?;

    //Get a free chunk for subject_exp
    let target_path = Path::from(target);
    target_path.ensure()?;

    let chunk = get_free_chunk(&target_path)?;

    //Should now path of form [exp, foundchunk]
    let target_path = add_chunk_path(target_path, chunk);
    target_path.ensure()?;

    create_link(
        source_path.hash()?,
        hash_entry(&Path::from(target))?,
        LinkTag::new(predicate.clone()),
    )?;
    create_link(
        target_path.hash()?,
        hash_entry(&Path::from(source))?,
        LinkTag::new(predicate.clone()),
    )?;
    Ok(())
}

pub fn add_chunk_path(path: Path, chunk: u32) -> Path {
    //Get components of path
    let mut components: Vec<_> = path.into();

    components.push(format!("{}", chunk).into());
    components.into()
}

pub fn get_free_chunk(path: &Path) -> ExternResult<u32> {
    let mut current_chunk = 0;
    let chunked_path = add_chunk_path(path.clone(), current_chunk);
    let mut chunked_path_hashed = hash_entry(&chunked_path)?;

    let chunk_val = loop {
        let links_len = get_links(chunked_path_hashed.clone(), None)?
            .into_inner()
            .len();
        //debug!("Found {:?}", links_len);
        if links_len < *SOFT_CHUNK_LIMIT {
            break current_chunk;
        } else {
            current_chunk = current_chunk + 1;
            let chunked_path = add_chunk_path(path.clone(), current_chunk.clone());
            //debug!("Checking: {:?}", chunked_path);
            chunked_path_hashed = hash_entry(&chunked_path)?;
        };
    };
    Ok(chunk_val)
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
