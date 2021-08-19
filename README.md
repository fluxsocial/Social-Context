# Social-Context

This is a sophisticated AD4M Language for link sharing (LinkLanguage) in [Neighbourhoods](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/neighbourhood/Neighbourhood.ts#L8), built using Holochain.

Ad4m [Links](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/links/Links.ts#L5) are a shape much like an RDF [triple](https://en.wikipedia.org/wiki/Semantic_triple). This shape allows us to associate a given subject to a given object with a predicate as another data point in this link. From such a structure you can start to associate pieces of data (in this case ad4m expression references) to each other and begin encoding application logic.<br> 
The ad4m linkLanguage provides the following interface for querying/adding links: [LinkLanguage](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/language/Language.ts#L104).<br>

The ad4m language defined here will call the included holochain DNA (found in `hc-dna`). This holochain DNA is configurable based on the dna properties that are passed when being installed (see `hc-dna/workdir/*.yaml`) for some examples. Overview of the DNA properties and what they do are as follows:

- `enforce_spam_limit`: number of links a given agent should be able to make per `max_chunk_interval` before failing validation due to spam protection.<br>
- `max_chunk_interval`: This value if read by: https://github.com/holochain-open-dev/holochain-time-index and is used to determine the depth of the time tree that is generated in order to index links under a given time index.<br>
- `active_agent_duration_s`: Length of time that an agent is considered online for after adding an `active_agent` link pointing to their agent address.<br>
- `enable_signals`: Determines if holochain signals should be sent to `active_agent(s)` when adding a link<br>
- `enable_time_index`: Determines if links should be added to a time index that makes links queryable between time bounds, see (LinkQuery)[https://github.com/juntofoundation/Social-Context/blob/16f99a5f8c8c97febca1876968a2f1f6d37a0fa8/hc-dna/zomes/social_context/src/inputs.rs#L16]<br>
- `index_strategy`: Determines what values from the triple are indexed and thus queryable in the future. Options are `FullWithWildCard`, `Full` & `Simple`. Full with wildcard will make links discoverable by subject, predicate, target & *. Full will make discoverable by subject, predicate, target. Simple by only discoverable subject.<br>

## Build
```
npm i
npm run build
```
This builds the Holochain DNA first, using `nix-shell` (which needs to be installed) and then packages the whole language including the DNA in `build/bundle.js`.
