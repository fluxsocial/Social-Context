use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    prelude::{Address, Entry, LinkMatch},
};
use meta_traits::{Identity, SocialGraphDao};

use crate::{
    FollowersAnchor, FollowingsAnchor, FriendshipAnchor, FriendshipAnchorTypes, SocialGraph,
};

pub fn create_anchors() -> Result<(), String> {
    let agent = hdk::AGENT_ADDRESS.clone();
    //No need for link between anchor(s) -> agent as this already contained inside anchor;
    //This means address is always derivable for agent so long as they know their agent address
    hdk::commit_entry(&Entry::App(
        "followings_anchor".into(),
        (FollowingsAnchor {
            agent: agent.clone(),
        })
        .into(),
    ))?;
    hdk::commit_entry(&Entry::App(
        "followers_anchor".into(),
        (FollowersAnchor {
            agent: agent.clone(),
        })
        .into(),
    ))?;
    hdk::commit_entry(&Entry::App(
        "friendship_anchor".into(),
        (FriendshipAnchor {
            agent: agent.clone(),
            anchor_type: FriendshipAnchorTypes::Live,
        })
        .into(),
    ))?;
    hdk::commit_entry(&Entry::App(
        "friendship_request_anchor".into(),
        (FriendshipAnchor {
            agent: agent.clone(),
            anchor_type: FriendshipAnchorTypes::Request,
        })
        .into(),
    ))?;
    hdk::commit_entry(&Entry::App(
        "friendship_receive_anchor".into(),
        (FriendshipAnchor {
            agent: agent.clone(),
            anchor_type: FriendshipAnchorTypes::Receive,
        })
        .into(),
    ))?;

    Ok(())
}

impl SocialGraphDao for SocialGraph {
    fn my_followers(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let followers_anchor_addr: Address =
            FollowingsAnchor::generate_anchor_address(source_agent);

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followers_anchor_addr,
            LinkMatch::Exactly("followed"),
            by_tag,
        ) {
            Ok(result) => Ok(result.addresses()),
            Err(err) => Err(err),
        }
    }

    fn followers(followed_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let followers_anchor_addr: Address =
            FollowersAnchor::generate_anchor_address(followed_agent);

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followers_anchor_addr,
            LinkMatch::Exactly("followed"),
            by_tag,
        ) {
            Ok(result) => Ok(result.addresses()),
            Err(err) => Err(err),
        }
    }

    fn nth_level_followers(
        _n: usize,
        _followed_agent: Identity,
        _by: Option<String>,
    ) -> ZomeApiResult<Vec<Identity>> {
        Err(ZomeApiError::Internal(String::from(
            "nth_level_followers not implemented for private social graphs",
        )))
    }

    fn my_followings(by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let followings_anchor_addr: Address =
            FollowingsAnchor::generate_anchor_address(source_agent.clone());

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followings_anchor_addr,
            LinkMatch::Exactly("follows"),
            by_tag,
        ) {
            Ok(result) => Ok(result.addresses()),
            Err(err) => Err(err),
        }
    }

    fn following(following_agent: Identity, by: Option<String>) -> ZomeApiResult<Vec<Identity>> {
        let followings_anchor_addr: Address =
            FollowingsAnchor::generate_anchor_address(following_agent);

        let by_tag = if by.is_some() {
            LinkMatch::Exactly(by.as_ref().map(String::as_str).unwrap())
        } else {
            LinkMatch::Any
        };

        match hdk::get_links(
            &followings_anchor_addr,
            LinkMatch::Exactly("follows"),
            by_tag,
        ) {
            Ok(result) => Ok(result.addresses()),
            Err(err) => Err(err),
        }
    }

    fn nth_level_following(
        _n: usize,
        _following_agent: Identity,
        _by: Option<String>,
    ) -> ZomeApiResult<Vec<Identity>> {
        Err(ZomeApiError::Internal(String::from(
            "nth_level_following not implemented for private social graphs",
        )))
    }

    fn follow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let by = &by.unwrap_or(String::from(""));
        //Create follow link between sources followings_anchor -> target identity
        let followings_anchor_addresses: Address =
            FollowingsAnchor::generate_anchor_address(source_agent.clone());
        hdk::link_entries(&followings_anchor_addresses, &target_agent, "follows", by)?;

        //Create a follow reference on targets followers_anchor so that they have a place to view follows
        let followers_anchor_addr: Address = FollowersAnchor::generate_anchor_address(target_agent);
        hdk::link_entries(
            &followers_anchor_addr,
            &source_agent.clone(),
            "followed",
            by,
        )?;
        Ok(())
    }

    fn unfollow(target_agent: Identity, by: Option<String>) -> ZomeApiResult<()> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let by = &by.unwrap_or(String::from(""));
        let followings_anchor_addresses: Address =
            FollowingsAnchor::generate_anchor_address(source_agent.clone());
        hdk::remove_link(&followings_anchor_addresses, &target_agent, "follows", by)?;

        let followers_anchor_addr: Address = FollowersAnchor::generate_anchor_address(target_agent);
        hdk::remove_link(
            &followers_anchor_addr,
            &source_agent.clone(),
            "followed",
            by,
        )?;
        Ok(())
    }

    // friendships

    fn my_friends() -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn friends_of(_agent: Identity) -> ZomeApiResult<Vec<Identity>> {
        Ok(vec![])
    }

    fn request_friendship(target_agent: Identity) -> ZomeApiResult<()> {
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let target_live_anchor: Address = FriendshipAnchor::generate_anchor_address(
            target_agent.clone(),
            FriendshipAnchorTypes::Live,
        );
        let target_receive_anchor: Address = FriendshipAnchor::generate_anchor_address(
            target_agent.clone(),
            FriendshipAnchorTypes::Receive,
        );

        let source_receive_anchor: Address = FriendshipAnchor::generate_anchor_address(
            source_agent.clone(),
            FriendshipAnchorTypes::Receive,
        );

        //Get links on target_live_anchor & check if live friendship already exists if so then do nothing
        if hdk::get_links(
            &target_live_anchor,
            LinkMatch::Exactly("friendship"),
            LinkMatch::Exactly(source_agent.to_string().as_ref()),
        )?.addresses().len() > 0 {
            return Ok(())
        };

        //If source user already has a request from target; then drop the requests and make a Live connection
        if hdk::get_links(
            &source_receive_anchor,
            LinkMatch::Exactly("friendship_request"),
            LinkMatch::Exactly(target_agent.to_string().as_ref()),
        )?.addresses().len() > 0 {
            //Remove request from source user
            hdk::remove_link(&source_receive_anchor, &target_agent, "friendship_request", target_agent.to_string().as_ref())?;
            //Create friendship link on source and targets live anchor
            let sources_live_anchor = FriendshipAnchor::generate_anchor_address(source_agent.clone(), FriendshipAnchorTypes::Live);
            hdk::link_entries(&sources_live_anchor, &target_agent, "friendship", target_agent.to_string().as_ref())?;
            hdk::link_entries(&target_live_anchor, &source_agent, "friendship", source_agent.to_string().as_ref())?;
        } else {
            //Create request link on targets anchor
            hdk::link_entries(&target_receive_anchor, &source_agent, "friendship_request", source_agent.to_string().as_ref())?;
        };

        Ok(())
    }

    fn decline_friendship(target_agent: Identity) -> ZomeApiResult<()> {
        //Check for friendship request in receive anchor and drop
        let source_agent = hdk::AGENT_ADDRESS.clone();
        let source_receive_anchor: Address = FriendshipAnchor::generate_anchor_address(source_agent, FriendshipAnchorTypes::Receive);
        if hdk::get_links(
            &source_receive_anchor,
            LinkMatch::Exactly("friendship_request"),
            LinkMatch::Exactly(target_agent.to_string().as_ref()),
        )?.addresses().len() > 0 {
            hdk::remove_link(&source_receive_anchor, &target_agent, "friendship_request", target_agent.to_string().as_ref())?;
        };
        Ok(())
    }

    fn incoming_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        let my_agent_address = hdk::AGENT_ADDRESS.clone();
        match hdk::api::get_links(
            &my_agent_address,
            LinkMatch::Exactly("friendship_request_receive"),
            LinkMatch::Any,
        ) {
            Ok(result) => Ok(result
                .addresses()
                .iter()
                .map(|address| address.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }

    fn outgoing_friendship_requests() -> ZomeApiResult<Vec<Identity>> {
        let my_agent_address = hdk::AGENT_ADDRESS.clone();
        match hdk::api::get_links(
            &my_agent_address,
            LinkMatch::Exactly("friendship_request_send"),
            LinkMatch::Any,
        ) {
            Ok(result) => Ok(result
                .addresses()
                .iter()
                .map(|address| address.clone())
                .collect()),
            Err(e) => Err(e),
        }
    }

    fn drop_friendship(_target_agent: Identity) -> ZomeApiResult<()> {
        Ok(())
    }
}
