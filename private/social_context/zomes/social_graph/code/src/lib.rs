#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

pub mod methods;
pub mod definitions;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::Address;
use hdk::{entry_definition::ValidatingEntryType, error::ZomeApiResult};
use hdk_proc_macros::zome;

use meta_traits::{SocialGraphDao, Identity};

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
struct FriendshipRequest {
}

// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct OutgoingFriendshipRequestsAnchor {
// 	agent: HashString
// }
// 
// #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
// pub struct IncomingFriendshipRequestsAnchor {
// 	agent: HashString
// }

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FollowingsAnchor {
	agent: Address
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FollowersAnchor {
	agent: Address
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Friendship {}

pub struct SocialGraph();


#[zome]
pub mod social_context {
    #[entry_def]
    pub fn friendship_request_def() -> ValidatingEntryType {
        definitions::friendship_request_def()
    }

    #[entry_def]
    pub fn followers_anchor_def() -> ValidatingEntryType {
        definitions::followers_anchor_def()
    }

    #[entry_def]
    pub fn followings_anchor_def() -> ValidatingEntryType {
        definitions::followings_anchor_def()
    }

    #[init]
    pub fn init() {
        methods::create_anchors()?;
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        //Here should live some membrane rules to make this DNA private
        Ok(())
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn my_followers(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::my_followers(by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn followers(followed_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::followers(followed_agent, by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn nth_level_followers(        
        n: usize,
        followed_agent: Identity,
        by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::nth_level_followers(n, followed_agent, by)
    }
    
    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn my_followings(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::my_followings(by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn following(followed_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::following(followed_agent, by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn nth_level_following(        
        n: usize,
        following_agent: Identity,
        by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::nth_level_following(n, following_agent, by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn follow(target_agent_address: Identity, by: Option<String>) -> ZomeApiResult<()> {
        SocialGraph::follow(target_agent_address, by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn unfollow(target_agent_address: Identity, by: Option<String>) -> ZomeApiResult<()> {
        SocialGraph::unfollow(target_agent_address, by)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn my_friends() -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::my_friends()
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn friends_of(agent: Identity) -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::friends_of(agent)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn request_friendship(target_agent_address: Identity) -> ZomeApiResult<()> {
        SocialGraph::request_friendship(target_agent_address)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn decline_friendship(target_agent_address: Identity) -> ZomeApiResult<()> {
        SocialGraph::decline_friendship(target_agent_address)
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn incoming_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::incoming_friendship_requests()
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn outgoing_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        SocialGraph::outgoing_friendship_requests()
    }

    #[zome_fn("hc_public")]
    #[zome_fn("social_graph")]
    pub fn drop_friendship(target_agent_address: Identity) -> ZomeApiResult<()> {
        SocialGraph::drop_friendship(target_agent_address)
    }
}