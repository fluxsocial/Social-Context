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
