use crate::inputs::Triple;
use hdk::prelude::*;

#[derive(PartialEq, Debug)]
pub (crate) struct LinkPermutation {
    pub root_index: String,
    pub tag: LinkTag
}

impl LinkPermutation {
    pub (crate) fn new<T: Into<Vec<u8>>>(source: String, tag: T) -> LinkPermutation {
        LinkPermutation {
            root_index: source,
            tag: LinkTag::new(tag)
        }
    }
}

pub(crate) fn get_wildcard() -> &'static str {
    "*"
}

/// This function generates the required source index value & tag that allows us to create an index for each element of the triple found in the link expression 
pub(crate) fn generate_link_path_permutations(
    triple: &Triple,
) -> ExternResult<Vec<LinkPermutation>> {

    // Get the wildcard identifier; note this is used when we want to index by some value but dont have another value to pair it with and thus are just indexing the LinkExpression by one value
    let wildcard = get_wildcard();

    let Triple { source, target, predicate } = triple;
    match (source, target, predicate) {
        (Some(source), Some(target), Some(predicate)) => {
            // Triple contains source, target and predicate so lets create an index that makes this LinkExpression queryable by:
            // source, target, predicate, source + target, source + predicate, target + predicate
            Ok(vec![
                LinkPermutation::new(source.clone(), wildcard),
                LinkPermutation::new(target.clone(), wildcard),
                LinkPermutation::new(predicate.clone(), wildcard),
                LinkPermutation::new(source.clone(), target.clone()),
                LinkPermutation::new(source.clone(), predicate.clone()),
                LinkPermutation::new(target.clone(), predicate.clone()),
            ])
        },
        (Some(source), Some(target), None) => {
            // Generate permutations to create indexes that makes this discoverable by: source + target, source, target
            Ok(vec![
                LinkPermutation::new(source.clone(), target.clone()),
                LinkPermutation::new(source.clone(), wildcard),
                LinkPermutation::new(target.clone(), wildcard),
            ])
        },
        (Some(source), None, Some(predicate)) => {
            // Generate permutations to create indexes that makes this discoverable by: source + predicate, source, predicate
            Ok(vec![
                LinkPermutation::new(source.clone(), predicate.clone()),
                LinkPermutation::new(source.clone(), wildcard),
                LinkPermutation::new(predicate.clone(), wildcard),
            ])
        },
        (None, Some(target), Some(predicate)) => {
            // Generate permutations to create indexes that makes this discoverable by: target + predicate, target, predicate
            Ok(vec![
                LinkPermutation::new(target.clone(), predicate.clone()),
                LinkPermutation::new(target.clone(), wildcard),
                LinkPermutation::new(predicate.clone(), wildcard),
            ])
        },
        (Some(source), None, None) => {
            // Source -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(source.clone(), wildcard),
            ])
        },
        (None, Some(target), None) => {
            // Target -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(target.clone(), wildcard),
            ])
        },
        (None, None, Some(predicate)) => {
            // Predicate -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(predicate.clone(), wildcard),
            ])
        },
        (None, None, None) => {
            Err(WasmError::Host(String::from("Link has no entities")))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_link_path_permutations_works() {
        const TRIPLE_SOURCE: &str = "source";
        const TRIPLE_TARGET: &str = "target";
        const TRIPLE_PREDICATE: &str = "predicate";
        const WILDCARD: &str = "*";

        // The triple contains source, target, predicate
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));
        assert_eq!(result[1], LinkPermutation::new("target".to_string(), WILDCARD.to_string()));
        assert_eq!(result[2], LinkPermutation::new(TRIPLE_PREDICATE.to_string(), WILDCARD.to_string()));
        assert_eq!(result[3], LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_TARGET.to_string()));
        assert_eq!(result[4], LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_PREDICATE.to_string()));
        assert_eq!(result[5], LinkPermutation::new(TRIPLE_TARGET.to_string(), TRIPLE_PREDICATE.to_string()));

        // The triple contains source, target
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: None,
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_TARGET.to_string()));
        assert_eq!(result[1], LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));
        assert_eq!(result[2], LinkPermutation::new(TRIPLE_TARGET.to_string(), WILDCARD.to_string()));

        // The triple contains source, predicate
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: None,
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_PREDICATE.to_string()));
        assert_eq!(result[1], LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));
        assert_eq!(result[2], LinkPermutation::new(TRIPLE_PREDICATE.to_string(), WILDCARD.to_string()));

        // The triple contains target, predicate
        let triple = Triple {
            source: None,
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_TARGET.to_string(), TRIPLE_PREDICATE.to_string()));
        assert_eq!(result[1], LinkPermutation::new(TRIPLE_TARGET.to_string(), WILDCARD.to_string()));
        assert_eq!(result[2], LinkPermutation::new(TRIPLE_PREDICATE.to_string(), WILDCARD.to_string()));

        // The triple contains source
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: None,
            predicate: None,
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));

        // The triple contains target
        let triple = Triple {
            source: None,
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: None,
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_TARGET.to_string(), WILDCARD.to_string()));

        // The triple contains predicate
        let triple = Triple {
            source: None,
            target: None,
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_PREDICATE.to_string(), WILDCARD.to_string()));

        // The triple contains nothing
        let triple = Triple {
            source: None,
            target: None,
            predicate: None,
        };
        let result = generate_link_path_permutations(&triple);
        assert!(result.is_err());
    }
}
