use crate::{LinkExpression, get_wildcard};
use hdk::prelude::*;

pub (crate) struct LinkPermutation {
    pub source: String,
    pub tag: LinkTag
}

impl LinkPermutation {
    pub (crate) fn new<T: Into<Vec<u8>>>(source: String, tag: T) -> LinkPermutation {
        LinkPermutation {
            source,
            tag: LinkTag::new(tag)
        }
    }
}

pub (crate) fn generate_link_path_permutations(
    link: &LinkExpression,
) -> ExternResult<Vec<LinkPermutation>> {
    let mut out = vec![];

    let num_entities = link.data.num_entities();
    let wildcard = get_wildcard();

    if num_entities == 0 {
        Err(WasmError::Host(String::from("Link has no entities")))
    } else if num_entities == 3 {
        let source = link.data.source.clone().unwrap();
        let target = link.data.target.clone().unwrap();
        let predicate = link.data.predicate.clone().unwrap();
        //source -> * -> LinkExpression
        out.push(LinkPermutation::new(source.clone(), wildcard));
        //target -> * -> LinkExpression
        out.push(LinkPermutation::new(target.clone(), wildcard));
        //Predicate -> * -> LinkExpression
        out.push(LinkPermutation::new(predicate.clone(), wildcard));

        //source target -> * -> LinkExpression
        out.push(LinkPermutation::new(source.clone(), target.clone()));
        //source predicate -> * -> LinkExpression
        out.push(LinkPermutation::new(source, predicate.clone()));
        //target predicate -> * -> LinkExpression
        out.push(LinkPermutation::new(target, predicate));
        Ok(out)
    } else if num_entities == 2 {
        if link.data.source.is_some() {
            if link.data.target.is_some() {
                let source = link.data.source.clone().unwrap();
                let target = link.data.target.clone().unwrap();
                //source target -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(source.clone(), target.clone()));

                //source -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(source, wildcard));

                //target -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(target, wildcard));
            } else {
                let source = link.data.source.clone().unwrap();
                let predicate = link.data.predicate.clone().unwrap();
                //source predicate -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(source.clone(), predicate.clone()));

                //source -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(source, wildcard));

                //Predicate -> wildcard -> LinkExpression
                out.push(LinkPermutation::new(predicate, wildcard));
            };
        } else if link.data.target.is_some() {
            let target = link.data.target.clone().unwrap();
            let predicate = link.data.predicate.clone().unwrap();
            //target, predicate -> wildcard -> LinkExpression
            out.push(LinkPermutation::new(target.clone(), predicate.clone()));
            //target -> * -> LinkExpression
            out.push(LinkPermutation::new(target, wildcard));
            //Predicate -> * -> LinkExpression
            out.push(LinkPermutation::new(predicate, wildcard));
        } else {
            unreachable!()
        };
        Ok(out)
    } else if link.data.source.is_some() {
        let source = link.data.source.clone().unwrap();
        //source -> * -> LinkExpression
        out.push(LinkPermutation::new(source, wildcard));
        Ok(out)
    } else if link.data.target.is_some() {
        let target = link.data.target.clone().unwrap();
        //target -> * -> LinkExpression
        out.push(LinkPermutation::new(target, wildcard));
        Ok(out)
    } else {
        let predicate = link.data.predicate.clone().unwrap();
        //Predicate -> * -> LinkExpression
        out.push(LinkPermutation::new(predicate, wildcard));
        Ok(out)
    }
}
