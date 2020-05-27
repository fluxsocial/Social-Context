use hdk::{
    error::ZomeApiResult,
    prelude::{Entry, LinkMatch},
};
use meta_traits::{SocialGraphDao, Identity};

use crate::{SocialGraph, FollowersAnchor, FollowingsAnchor, FriendshipRequest};

pub fn create_anchors() -> Result<(), String> {
    let agent = hdk::AGENT_ADDRESS.clone(); 
    let anchor1 = Entry::App("followings_anchor".into(), (FollowingsAnchor{ agent: agent.clone() }).into()); 
    let anchor1_addr = hdk::commit_entry(&anchor1)?; 
    hdk::link_entries(&agent, &anchor1_addr, "has_followings_anchor", "")?;

    let anchor2 = Entry::App("followers_anchor".into(), (FollowersAnchor{ agent: agent.clone() }).into()); 
    let anchor2_addr = hdk::commit_entry(&anchor2)?;
    hdk::link_entries(&agent, &anchor2_addr, "has_followers_anchor", "")?;
    
    Ok(())
}

impl SocialGraphDao for SocialGraph {
    fn my_followers(_by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn followers(_followed_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn nth_level_followers(_n: usize, _followed_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }
    
    fn my_followings(_by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let my_agent_address = hdk::AGENT_ADDRESS.clone().into();
        let followings_anchor_addresses = hdk::get_links(
            &my_agent_address, 
            LinkMatch::Exactly("has_followings_anchor"), 
            LinkMatch::Any
        )?.addresses(); 
        let anchor_addr = followings_anchor_addresses.first().unwrap(); 
    
        match hdk::get_links(
            &anchor_addr, 
            LinkMatch::Exactly("follows"), 
            LinkMatch::Any
        ) {
            Ok(result) => Ok(result.addresses()), 
            Err(err) => Err(err) 
        }
    }

    fn following(_following_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn nth_level_following(_n: usize, _following_agent: Identity, _by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }
    
    fn follow(target_agent: Identity, _by: Option<String>) -> ZomeApiResult<()> {
        let agent_addr = hdk::AGENT_ADDRESS.clone(); 
        let follower_anchor_addr = hdk::get_links(
            &agent_addr, 
            LinkMatch::Exactly("has_followings_anchor"),
            LinkMatch::Any
        )?.addresses();
        let fad = follower_anchor_addr.first().unwrap();
        hdk::link_entries(&fad, &target_agent, "follows", "")?;
    
        let followed_anchor_addr = hdk::get_links(
            & target_agent.clone(), 	
            LinkMatch::Exactly("has_followings_anchor"),
            LinkMatch::Any
        )?.addresses();
        let fad2 = followed_anchor_addr.first().unwrap(); 
        hdk::link_entries(&fad2, &agent_addr, "is_followed_by", "")?;
    
        Ok(())
    }
    
    fn unfollow(target_agent: Identity, _by: Option<String>) -> ZomeApiResult<()> {
        let sender_address = hdk::AGENT_ADDRESS.clone().into();
        hdk::remove_link(&sender_address, &target_agent, "follows", "")?;
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