use std::convert::TryFrom;

use hc_time_index::entries::IndexSegment;
use hc_time_index::entries::WrappedPath;
use hdk::prelude::*;

use crate::LinkExpression;
use crate::validation::LinkTypes;

pub fn get_index_segment(bytes: Option<SerializedBytes>) -> Option<IndexSegment> {
    match bytes {
        Some(bytes) => {
            let get_path = Path::try_from(bytes);
            get_index_segment_from_path(get_path.ok())
        }
        None => None,
    }
}

pub fn get_index_segment_from_path(path: Option<Path>) -> Option<IndexSegment> {
    match path {
        Some(path) => {
            match IndexSegment::try_from(WrappedPath(path)) {
                Ok(is) => Some(is),
                Err(_err) => None,
            }
        },
        None => None
    }
}

pub fn get_link_expression(bytes: Option<SerializedBytes>) -> Option<LinkExpression> {
    match bytes {
        Some(bytes) => {
            let link_expression = LinkExpression::try_from(bytes);
            match link_expression {
                Ok(le) => Some(le),
                Err(_err) => None,
            }
        }
        None => None,
    }
}

pub fn add_if_none(data: &mut Option<LinkTypes>, found: Option<LinkTypes>) {
    match data {
        None => {
            if found.is_some() {
                *data = found
            }
        }
        _ => {}
    }
}
