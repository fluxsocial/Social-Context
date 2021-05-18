use crate::LinkExpression;
use hdk::prelude::*;

pub fn generate_link_path_permutations(
    link: &LinkExpression,
) -> ExternResult<Vec<(String, String)>> {
    let num_entities = link.data.num_entities();
    let mut out = vec![];
    let wildcard = String::from("*");

    if num_entities == 0 {
        Err(WasmError::Host(String::from("Link has no entities")))
    } else if num_entities == 3 {
        let source = link.data.source.clone().unwrap();
        let target = link.data.target.clone().unwrap();
        let predicate = link.data.predicate.clone().unwrap();
        //source -> * -> LinkExpression
        out.push((source.clone(), wildcard.clone()));
        //target -> * -> LinkExpression
        out.push((target.clone(), wildcard.clone()));
        //Predicate -> * -> LinkExpression
        out.push((predicate.clone(), wildcard.clone()));

        //source target -> * -> LinkExpression
        out.push((source.clone(), target.clone()));
        //source predicate -> * -> LinkExpression
        out.push((source, predicate.clone()));
        //target predicate -> * -> LinkExpression
        out.push((target, predicate));
        Ok(out)
    } else if num_entities == 2 {
        if link.data.source.is_some() {
            if link.data.target.is_some() {
                let source = link.data.source.clone().unwrap();
                let target = link.data.target.clone().unwrap();
                //source target -> wildcard -> LinkExpression
                out.push((source.clone(), target.clone()));

                //source -> wildcard -> LinkExpression
                out.push((source, wildcard.clone()));

                //target -> wildcard -> LinkExpression
                out.push((target, wildcard));
            } else {
                let source = link.data.source.clone().unwrap();
                let predicate = link.data.predicate.clone().unwrap();
                //source predicate -> wildcard -> LinkExpression
                out.push((source.clone(), predicate.clone()));

                //source -> wildcard -> LinkExpression
                out.push((source, wildcard.clone()));

                //Predicate -> wildcard -> LinkExpression
                out.push((predicate, wildcard));
            };
        } else if link.data.target.is_some() {
            let target = link.data.target.clone().unwrap();
            let predicate = link.data.predicate.clone().unwrap();
            //target, predicate -> wildcard -> LinkExpression
            out.push((target.clone(), predicate.clone()));
            //target -> * -> LinkExpression
            out.push((target, wildcard.clone()));
            //Predicate -> * -> LinkExpression
            out.push((predicate, wildcard));
        } else {
            unreachable!()
        };
        Ok(out)
    } else if link.data.source.is_some() {
        let source = link.data.source.clone().unwrap();
        //source -> * -> LinkExpression
        out.push((source, wildcard));
        Ok(out)
    } else if link.data.target.is_some() {
        let target = link.data.target.clone().unwrap();
        //target -> * -> LinkExpression
        out.push((target, wildcard));
        Ok(out)
    } else {
        let predicate = link.data.predicate.clone().unwrap();
        //Predicate -> * -> LinkExpression
        out.push((predicate, wildcard));
        Ok(out)
    }
}
