use hdk::holochain_persistence_api::hash::HashString;
use hdk::{
    error::ZomeApiResult,
    prelude::{Entry, LinkMatch, Address},
    holochain_json_api::json::JsonString,
};
use meta_traits::{SocialGraphDao, Identity};
use multihash::Hash;

use crate::{SocialGraph, FollowersAnchor, FollowingsAnchor, FriendshipRequest};

pub fn create_anchors() -> Result<(), String> {
    let agent = hdk::AGENT_ADDRESS.clone(); 
    //No need for link between anchor -> agent as this already contained inside anchor; 
    //This means address is always derivable for agent so long as they know their agent address
    hdk::commit_entry(&Entry::App("followings_anchor".into(), (FollowingsAnchor{ agent: agent.clone() }).into()))?; 
    hdk::commit_entry(&Entry::App("followers_anchor".into(), (FollowersAnchor{ agent: agent.clone() }).into()))?;
    
    Ok(())
}

impl SocialGraphDao for SocialGraph {
    fn my_followers(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let followers_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followers_anchor".into(),
                FollowingsAnchor { agent: source_agent }.into(),
            )),
            Hash::SHA2256,
        );

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followers_anchor_addr, 
            LinkMatch::Exactly("followed"), 
            by_tag
        ) {
            Ok(result) => Ok(result.addresses()), 
            Err(err) => Err(err) 
        }
    }

    fn followers(followed_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let followers_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followers_anchor".into(),
                FollowingsAnchor { agent: followed_agent.clone() }.into(),
            )),
            Hash::SHA2256,
        );

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followers_anchor_addr, 
            LinkMatch::Exactly("followed"), 
            by_tag
        ) {
            Ok(result) => Ok(result.addresses()), 
            Err(err) => Err(err) 
        }
    }

    fn nth_level_followers(_n: usize, _followed_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }
    
    fn my_followings(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let followings_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followings_anchor".into(),
                FollowingsAnchor { agent: source_agent }.into(),
            )),
            Hash::SHA2256,
        );

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followings_anchor_addr, 
            LinkMatch::Exactly("follows"), 
            by_tag
        ) {
            Ok(result) => Ok(result.addresses()), 
            Err(err) => Err(err) 
        }
    }

    fn following(following_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let followings_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followings_anchor".into(),
                FollowingsAnchor { agent: following_agent }.into(),
            )),
            Hash::SHA2256,
        );

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followings_anchor_addr, 
            LinkMatch::Exactly("follows"), 
            by_tag
        ) {
            Ok(result) => Ok(result.addresses()), 
            Err(err) => Err(err) 
        }
    }

    fn nth_level_following(_n: usize, _following_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }
    
    fn follow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let by = &by.unwrap_or(String::from(""));
        //Create follow link between sources followings_anchor -> target identity
        let followings_anchor_addresses: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followings_anchor".into(),
                FollowingsAnchor { agent: source_agent.clone() }.into(),
            )),
            Hash::SHA2256,
        );
        hdk::link_entries(&followings_anchor_addresses, &target_agent, "follows", by)?;
        
        //Create a follow reference on targets followers_anchor so that they have a place to view follows
        let followers_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followers_anchor".into(),
                FollowingsAnchor { agent: target_agent.clone() }.into(),
            )),
            Hash::SHA2256,
        );
        hdk::link_entries(&followers_anchor_addr, &source_agent.clone(), "followed", by)?;
        Ok(())
    }
    
    fn unfollow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let by = &by.unwrap_or(String::from(""));
        let followings_anchor_addresses: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followings_anchor".into(),
                FollowingsAnchor { agent: source_agent.clone() }.into(),
            )),
            Hash::SHA2256,
        );
        hdk::remove_link(&followings_anchor_addresses, &target_agent, "follows", by)?;

        let followers_anchor_addr: Address = HashString::encode_from_json_string(
            JsonString::from(Entry::App(
                "followers_anchor".into(),
                FollowingsAnchor { agent: target_agent.clone() }.into(),
            )),
            Hash::SHA2256,
        );
        hdk::remove_link(&followers_anchor_addr, &source_agent.clone(), "followed", by)?;
        Ok(())
    }
    
    // friendships

    fn my_friends() -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn friends_of(_agent: Identity) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }
    
    fn request_friendship(
        target_agent: Identity,
    ) -> ZomeApiResult<()> {
        let sender_address = hdk::AGENT_ADDRESS.clone().into();
        let friendship_request = FriendshipRequest {};
        let entry = Entry::App("friendship_request".into(), friendship_request.into());
        let entry_address = hdk::commit_entry(&entry)?;
        hdk::link_entries(
            &sender_address,
            &entry_address,
            "friendship_request_send",
            "",
        )?;
        hdk::link_entries(
            &target_agent,
            &entry_address,
            "friendship_request_receive",
            "",
        )?;
        Ok(())
    }
    
    fn decline_friendship(_target_agent: Identity) -> ZomeApiResult<()> {
        Ok(())
    }
    
    fn incoming_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        let my_agent_address = hdk::AGENT_ADDRESS.clone().into(); 
        match hdk::api::get_links(
            &my_agent_address, 
            LinkMatch::Exactly("friendship_request_receive"), 
            LinkMatch::Any
        ) {
            Ok(result) => Ok(result.addresses().iter().map(|address| address.clone()).collect()), 
            Err(e) => Err(e)
        }
    }
    
    fn outgoing_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        let my_agent_address = hdk::AGENT_ADDRESS.clone().into(); 
        match hdk::api::get_links(
            &my_agent_address, 
            LinkMatch::Exactly("friendship_request_send"), 
            LinkMatch::Any
        ) {
            Ok(result) => Ok(result.addresses().iter().map(|address| address.clone()).collect()), 
            Err(e) => Err(e)
        }
    }

    fn drop_friendship(_target_agent: Identity) -> ZomeApiResult<()> {
        Ok(())
    }
}