use hc_time_index::entries::IndexSegment;
use hc_time_index::entries::WrappedPath;
use hdk::prelude::*;

use crate::LinkExpression;

pub fn get_index_segment(bytes: Option<SerializedBytes>) -> Option<IndexSegment> {
    match bytes {
        Some(bytes) => {
            let get_path = Path::try_from(bytes);
            match get_path {
                Ok(path) => match IndexSegment::try_from(WrappedPath(path)) {
                    Ok(is) => Some(is),
                    Err(_err) => None,
                },
                Err(_err) => None,
            }
        }
        None => None,
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

fn add_if_none(data: &mut Option<LinkTypes>, found: Option<LinkTypes>) {
    match data {
        None => {
            if found.is_some() {
                *data = found
            }
        }
        _ => {}
    }
}

#[derive(Debug)]
pub enum LinkTypes {
    IndexSegment(IndexSegment),
    LinkExpression(LinkExpression),
}

#[derive(Debug)]
pub struct LinkData {
    pub base: Option<LinkTypes>,
    pub target: Option<LinkTypes>,
    pub tag: Option<LinkTypes>,
}

#[hdk_extern]
pub fn validate_create_link(
    create_link_data: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult> {
    let mut link_data = LinkData {
        base: None,
        target: None,
        tag: None,
    };
    let base_bytes = match create_link_data.base.clone() {
        Entry::App(bytes) => Some(bytes.into_sb()),
        _ => None,
    };
    let target_bytes = match create_link_data.target.clone() {
        Entry::App(bytes) => Some(bytes.into_sb()),
        _ => None,
    };
    let tag_bytes = Some(SerializedBytes::try_from(UnsafeBytes::from(
        create_link_data.link_add.tag.0,
    ))?);
    let base_segment = get_index_segment(base_bytes.clone());
    let target_segment = get_index_segment(target_bytes.clone());
    let tag_segment = get_index_segment(tag_bytes.clone());
    add_if_none(
        &mut link_data.base,
        base_segment.map(|val| LinkTypes::IndexSegment(val)),
    );
    add_if_none(
        &mut link_data.target,
        target_segment.map(|val| LinkTypes::IndexSegment(val)),
    );
    add_if_none(
        &mut link_data.tag,
        tag_segment.map(|val| LinkTypes::IndexSegment(val)),
    );

    let base_link_expression = get_link_expression(base_bytes.clone());
    let target_link_expression = get_link_expression(target_bytes.clone());
    let tag_link_expression = get_link_expression(tag_bytes.clone());
    add_if_none(
        &mut link_data.base,
        base_link_expression.map(|val| LinkTypes::LinkExpression(val)),
    );
    add_if_none(
        &mut link_data.target,
        target_link_expression.map(|val| LinkTypes::LinkExpression(val)),
    );
    add_if_none(
        &mut link_data.tag,
        tag_link_expression.map(|val| LinkTypes::LinkExpression(val)),
    );

    debug!("Got link data for link: {:#?}", link_data);
    Ok(ValidateLinkCallbackResult::Valid)
}
