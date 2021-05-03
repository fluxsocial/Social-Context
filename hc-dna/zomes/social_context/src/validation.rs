//use hdk::prelude::*;

// fn return_app_entry_type(entry_type: &EntryType) -> Option<AppEntryType> {
//     match entry_type {
//         EntryType::App(et) => Some(et.to_owned()),
//         _ => None
//     }
// }

// fn get_element_and_type(addr: EntryHash) -> ExternResult<(Element, Option<AppEntryType>)> {
//     let element = get(addr, GetOptions::default())?
//         .ok_or(WasmError::Host("Could not get element entry for link".to_string()))?;
//     let header = element.header().entry_type().ok_or(WasmError::Host("Could not get entry_type from header".to_string()))?;
//     let entry_type = return_app_entry_type(header);
//     Ok((element, entry_type))
// }

// #[hdk_extern]
// pub fn validate_create_link(create_link_data: ValidateCreateLinkData) -> ExternResult<ValidateLinkCallbackResult> {
//     debug!("Got link with create_link_data: {:#?}", create_link_data);
//     let (_base_elem, base_type) = get_element_and_type(create_link_data.link_add.base_address)?;
//     let (_tg_elemt, tg_type) = get_element_and_type(create_link_data.link_add.target_address)?;
//     if base_type.is_none() {
//         return Ok(ValidateLinkCallbackResult::Valid)
//     };
//     if tg_type.is_none() {
//         return Ok(ValidateLinkCallbackResult::Valid)
//     };
//     debug!("got types {:#?} and {:#?}", base_type, tg_type);
//     Ok(ValidateLinkCallbackResult::Valid)

// }
