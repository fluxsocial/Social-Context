use hc_time_index::entries::IndexSegment;

use crate::LinkExpression;

mod validation;
mod utils;

pub use validation::*;

#[derive(Debug)]
pub enum LinkTypes {
    String(String),
    IndexSegment(IndexSegment),
    LinkExpression(LinkExpression),
}

#[derive(Debug)]
pub struct LinkData {
    pub base: Option<LinkTypes>,
    pub target: Option<LinkTypes>,
    pub tag: Option<LinkTypes>,
}
