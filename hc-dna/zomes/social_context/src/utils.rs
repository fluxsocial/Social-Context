use crate::inputs::Triple;
use hdk::prelude::*;

use std::hash::Hash;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub(crate) struct LinkPermutation {
    pub root_index: String,
    pub tag: LinkTag
}

impl LinkPermutation {
    pub(crate) fn new<T: Into<Vec<u8>>>(source: String, tag: T) -> LinkPermutation {
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
                LinkPermutation::new(format!("s{}", source.clone()), wildcard),
                LinkPermutation::new(format!("t{}", target.clone()), wildcard),
                LinkPermutation::new(format!("p{}", predicate.clone()), wildcard),
                LinkPermutation::new(format!("s{}", source.clone()), format!("t{}", target.clone())),
                LinkPermutation::new(format!("s{}", source.clone()), format!("p{}", predicate.clone())),
                LinkPermutation::new(format!("t{}", target.clone()), format!("p{}", predicate.clone())),
            ])
        },
        (Some(source), Some(target), None) => {
            // Generate permutations to create indexes that makes this discoverable by: source + target, source, target
            Ok(vec![
                LinkPermutation::new(format!("s{}", source.clone()), format!("t{}", target.clone())),
                LinkPermutation::new(format!("s{}", source.clone()), wildcard),
                LinkPermutation::new(format!("t{}", target.clone()), wildcard),
            ])
        },
        (Some(source), None, Some(predicate)) => {
            // Generate permutations to create indexes that makes this discoverable by: source + predicate, source, predicate
            Ok(vec![
                LinkPermutation::new(format!("s{}", source.clone()), format!("p{}", predicate.clone())),
                LinkPermutation::new(format!("s{}", source), wildcard),
                LinkPermutation::new(format!("p{}", predicate), wildcard),
            ])
        },
        (None, Some(target), Some(predicate)) => {
            // Generate permutations to create indexes that makes this discoverable by: target + predicate, target, predicate
            Ok(vec![
                LinkPermutation::new(format!("t{}", target.clone()), format!("p{}", predicate.clone())),
                LinkPermutation::new(format!("t{}", target), wildcard),
                LinkPermutation::new(format!("p{}", predicate), wildcard),
            ])
        },
        (Some(source), None, None) => {
            // Source -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(format!("s{}", source), wildcard),
            ])
        },
        (None, Some(target), None) => {
            // Target -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(format!("t{}", target), wildcard),
            ])
        },
        (None, None, Some(predicate)) => {
            // Predicate -> * -> LinkExpression
            Ok(vec![
                LinkPermutation::new(format!("p{}", predicate), wildcard),
            ])
        },
        (None, None, None) => {
            Err(WasmError::Host(String::from("Link has no entities")))
        },
    }
}

/// Derive the source link index value and link tag value to query with based on the values passed in GetLinks.triple
/// Note we are only looking for two or one elements in the triple, since if you have three you already have the LinkExpression! 
pub(crate) fn get_link_permutation_by(triple: Triple) -> LinkPermutation {
    let wildcard = get_wildcard();
    let Triple { source, target, predicate } = triple;
    
    match (source, target, predicate) {
        //Query with source + target; will match all LinkExpression with same source + target
        //In this case the predicate unknown here and thus the value zome caller is interested in
        (Some(source), Some(target), _) => LinkPermutation::new(
            format!("s{}", source),
            format!("t{}", target),
        ),
        //Query with source + predicate
        //Here target is unknown and thus the value the zome caller is looking for
        (Some(source), None, Some(predicate)) => LinkPermutation::new(
            format!("s{}", source),
            format!("p{}", predicate),
        ),
        (None, Some(target), Some(predicate)) => LinkPermutation::new(
            format!("t{}", target),
            format!("p{}", predicate),
        ),
        //Look for all links with the given source
        (Some(source), None, None) => LinkPermutation::new(
            format!("s{}", source),
            wildcard,
        ),
        (None, Some(target), None) => LinkPermutation::new(
            format!("t{}", target),
            wildcard,
        ),
        (None, None, Some(predicate)) => LinkPermutation::new(
            format!("p{}", predicate),
            wildcard,
        ),
        //No elements were supplied in the triple so we use wildcards as source + predicate to simulate a getAllLinks query 
        //(note for this to work the FullWithWildCard index needs to be enabled)
        (None, None, None) => LinkPermutation::new(
            wildcard.to_string(),
            wildcard,
        ),
    }
}

pub (crate) fn dedup<T: Eq + Hash + Clone>(vs: &Vec<T>) -> Vec<T> {
    let hs = vs.iter().cloned().collect::<HashSet<T>>();

    hs.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIPLE_SOURCE: &str = "source";
    const TRIPLE_TARGET: &str = "target";
    const TRIPLE_PREDICATE: &str = "predicate";
    const WILDCARD: &str = "*";
    
    #[test]
    fn generate_link_path_permutations_works() {
        // The triple contains source, target, predicate
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = generate_link_path_permutations(&triple).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(result[0], LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));
        assert_eq!(result[1], LinkPermutation::new(TRIPLE_TARGET.to_string(), WILDCARD.to_string()));
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

    #[test]
    fn get_link_permutation_by_triple_works() {
        // The triple contains source, target, predicate
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_TARGET.to_string()));

        // The triple contains source, target
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: None,
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_TARGET.to_string()));

        // The triple contains source, predicate
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: None,
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_SOURCE.to_string(), TRIPLE_PREDICATE.to_string()));

        // The triple contains target, predicate
        let triple = Triple {
            source: None,
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_TARGET.to_string(), TRIPLE_PREDICATE.to_string()));

        // The triple contains source
        let triple = Triple {
            source: Some(TRIPLE_SOURCE.to_string()),
            target: None,
            predicate: None,
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_SOURCE.to_string(), WILDCARD.to_string()));

        // The triple contains target
        let triple = Triple {
            source: None,
            target: Some(TRIPLE_TARGET.to_string()),
            predicate: None,
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_TARGET.to_string(), WILDCARD.to_string()));

        // The triple contains predicate
        let triple = Triple {
            source: None,
            target: None,
            predicate: Some(TRIPLE_PREDICATE.to_string()),
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(TRIPLE_PREDICATE.to_string(), WILDCARD.to_string()));

        // The triple contains nothing
        let triple = Triple {
            source: None,
            target: None,
            predicate: None,
        };
        let result = get_link_permutation_by(triple);
        assert_eq!(result, LinkPermutation::new(WILDCARD.to_string(), WILDCARD.to_string()));
    }
}
