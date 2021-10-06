use hdk::prelude::*;

use crate::validation::{LinkTypes, LinkData, utils::{add_if_none, get_index_segment, get_index_segment_from_path, get_link_expression}};

#[hdk_extern]
pub fn validate_create_link(
    create_link_data: ValidateCreateLinkData,
) -> ExternResult<ValidateLinkCallbackResult> {
    //Init the link data structure we want to create to start making validation from
    let mut link_data = LinkData {
        base: None,
        target: None,
        tag: None,
    };

    //Get the base and target bytes we want to work with
    let base_bytes = match create_link_data.base.clone() {
        Entry::App(bytes) => Some(bytes.into_sb()),
        _ => None,
    };
    let target_bytes = match create_link_data.target.clone() {
        Entry::App(bytes) => Some(bytes.into_sb()),
        _ => None,
    };

    //Try and get an IndexSegment for the base and target
    let base_segment = get_index_segment(base_bytes.clone());
    let target_segment = get_index_segment(target_bytes.clone());
    add_if_none(
        &mut link_data.base,
        base_segment.map(|val| LinkTypes::IndexSegment(val)),
    );
    add_if_none(
        &mut link_data.target,
        target_segment.map(|val| LinkTypes::IndexSegment(val)),
    );

    //Try and get a LinkExpression for the base and target
    let base_link_expression = get_link_expression(base_bytes.clone());
    let target_link_expression = get_link_expression(target_bytes.clone());
    add_if_none(
        &mut link_data.base,
        base_link_expression.map(|val| LinkTypes::LinkExpression(val)),
    );
    add_if_none(
        &mut link_data.target,
        target_link_expression.map(|val| LinkTypes::LinkExpression(val)),
    );

    //Try and get a string from the tag
    let tag_data_string = String::from_utf8(create_link_data.link_add.tag.0.clone()).ok();
    add_if_none(
        &mut link_data.tag,
        tag_data_string.map(|val| LinkTypes::String(val)),
    );

    //Have to check if none here else Path::try_from() panic's even though it can return an Err
    if link_data.tag.is_none() {
        //Try and get an IndexSegment from the tag
        let tag_data_path = Path::try_from(&create_link_data.link_add.tag);
        let tag_data_path = get_index_segment_from_path(tag_data_path.ok());
        add_if_none(
            &mut link_data.tag,
            tag_data_path.map(|val| LinkTypes::IndexSegment(val)),
        );
    }

    debug!("Got link data for link: {:#?}", link_data);
    Ok(ValidateLinkCallbackResult::Valid)
}
