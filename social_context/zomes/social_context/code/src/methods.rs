use hdk::holochain_persistence_api::hash::HashString;
use hdk::{
    error::{ZomeApiError, ZomeApiResult},
    holochain_json_api::json::JsonString,
    prelude::{Address, Entry, GetLinksOptions, LinkMatch, Pagination, SizePagination, SortOrder},
};
use meta_traits::{GlobalEntryRef, SocialContextDao, Identity};

use crate::SocialContextDNA;

impl SocialContextDao for SocialContextDNA {
    /// Persist to social context that you have made an entry at expression_ref.dna_address/@expression_ref.entry_address
    /// which is most likely contextual to the collective of host social context
    fn post(expression_ref: GlobalEntryRef) -> ZomeApiResult<()> {
        Ok(())
    }
    /// Register that there is some dna at dna_address that you are communicating in.
    /// Others in collective can use this to join you in new DNA's
    fn register_comment_link_dna(dna_address: Address) -> ZomeApiResult<()> {
        Ok(())
    }
    /// Is current agent allowed to write to this DNA
    fn writable() -> bool {
        true
    }
    /// Get GlobalEntryRef for collective; queryable by dna or agent or all. DHT hotspotting @Nico?
    fn read_links(
        by_dna: Option<Address>,
        by_agent: Option<Identity>,
        count: usize,
        page: usize,
    ) -> ZomeApiResult<GlobalEntryRef> {
        Ok(())
    }
    /// Get DNA's this social context is communicating in
    fn get_comment_link_dnas(count: usize, page: usize) -> ZomeApiResult<GlobalEntryRef> {
        Ok(())
    }
    /// Get agents who are a part of this social context
    /// optional to not force every implementation to create a global list of members - might be ok for small DHTs
    fn members(count: usize, page: usize) -> ZomeApiResult<Option<Vec<Identity>>> {
        Ok(None)
    }
}