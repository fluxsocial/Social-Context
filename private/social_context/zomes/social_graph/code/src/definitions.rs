use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::prelude::{EntryType, ValidatingEntryType};

use crate::{FriendshipRequest, FollowersAnchor, FollowingsAnchor};

pub fn friendship_request_def() -> ValidatingEntryType {
    entry!(
        name: "friendship_request",
        description: "expresses the willingness of one agent to be in a friendship relation with another one",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<FriendshipRequest> | {
            Ok(())
        },
        links: [
            from!(
                EntryType::AgentId,
                link_type: "friendship_request_send",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            from!(
                EntryType::AgentId,
                link_type: "friendship_request_receive",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

pub fn followers_anchor_def() -> ValidatingEntryType {
    entry!(
        name: "followers_anchor", 
        description: "each agent A links to its own followers_anchor. This anchor then links to all the agents that follow agent A", 
        sharing: Sharing::Public, 
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        }, 
        validation: | _validation_data: hdk::EntryValidationData<FollowersAnchor> | {
            Ok(())
        }, 
        links: [
            from!(
                EntryType::AgentId, 
                link_type: "has_followers_anchor", 
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!(
                EntryType::AgentId, 
                link_type: "is_followed_by", 
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

pub fn followings_anchor_def() -> ValidatingEntryType {
    entry!(
        name: "followings_anchor", 
        description: "each agent A links to its own followings_anchor. This anchor then links to all the agents that follow agent A", 
        sharing: Sharing::Public, 
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        }, 
        validation: | _validation_data: hdk::EntryValidationData<FollowingsAnchor> | {
            Ok(())
        }, 
        links: [
            from!(
                EntryType::AgentId, 
                link_type: "has_followings_anchor", 
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!(
                EntryType::AgentId, 
                link_type: "follows", 
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}