# Social-Context

This is a sophisticated AD4M Language for link sharing in [Neighbourhoods](https://neighbourhoods.network/) which has the following [AD4M type signature](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/neighbourhood/Neighbourhood.ts#L8).

AD4M [Links](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/links/Links.ts#L5) are a shape much like an [RDF triple](https://en.wikipedia.org/wiki/Semantic_triple). This shape allows us to associate a given `subject` to a given `target` with a `predicate` as another data point in this link. From such a structure you can start to associate pieces of data (in this case AD4M expression references) to each other and begin encoding application logic.<br> 
The ad4m linkLanguage provides the following interface for querying/adding links: [LinkLanguage](https://github.com/perspect3vism/ad4m/blob/68f3a48148391b94f929996d91dc0882a1bbf2d0/src/language/Language.ts#L104).<br>

The ad4m language defined here will call the included holochain DNA (found in `hc-dna`). This holochain DNA is configurable based on the dna properties that are passed when being installed (see `hc-dna/workdir/*.yaml`) for some examples. Overview of the DNA properties and what they do are as follows:

- `enforce_spam_limit`: number of links a given agent should be able to make per `max_chunk_interval` before failing validation due to spam protection.<br>
- `max_chunk_interval`: This value if read by [holochain-time-index](https://github.com/holochain-open-dev/holochain-time-index) and is used to determine the depth of the time tree that is generated in order to index links under a given time index.<br>
- `active_agent_duration_s`: Length of time that an agent is considered online for after adding an `active_agent` link pointing to their agent address.<br>
- `enable_signals`: Determines if holochain signals should be sent to `active_agent(s)` when adding a link<br>
- `enable_time_index`: Determines if links should be added to a time index that makes links queryable between time bounds, see [LinkQuery](https://github.com/juntofoundation/Social-Context/blob/16f99a5f8c8c97febca1876968a2f1f6d37a0fa8/hc-dna/zomes/social_context/src/inputs.rs#L16)<br>
- `index_strategy`: Determines what values from the triple are indexed and thus queryable in the future. Options are `FullWithWildCard`, `Full` & `Simple`. Full with wildcard will make links discoverable by subject, predicate, target & *. Full will make discoverable by subject, predicate, target. Simple by only discoverable subject. It's an input parameter of `add_link` zome external function and may be different for each call.<br>

# How is this used in Junto?

Neighbourhoods (and this ad4m language as the backbone) are used in the Junto [communities app](https://github.com/juntofoundation/communities) to represent a community and more specifically share links agents create on a community with each other. <br>
Via the [ad4m-executor](https://github.com/perspect3vism/ad4m-executor) we can create neighbourhoods + add links into the neighbourhood to be retreived by agents and used to construct application logic/experience.<br>

The source, target, predicate values found in these links are usually ad4m expression references in the form: `languageHash://expressionId` and resolvable by the given ad4m language to expression objects. An example of a link you may find in a junto community neighbourhood may look like:
<br>
```
{
    source: "neighbourhood://neighbourhoodHash",
    predicate: "hasPost",
    target: "shortFormLanguageHash://expressionHash"
}
```
<br>

Here the target would point to an expression found in the [ShortForm language](https://github.com/juntofoundation/Short-Form-Expression) and be resolvable to an object found there. The `hasPost` predicate tag tells us this is a post on the neighbourhood found at source: `neighbourhood://neighbourhoodHash`.

# Development

## Build

```
npm i
npm run build
```
This builds the Holochain DNA first, using `nix-shell` (which needs to be installed) and then packages the whole language including the DNA in `build/bundle.js`.

## Test

```
make test
```

Add `RUST_LOG=debug TRYORAMA_LOG_LEVEL=debug` to test commands in package.json to view debug logs.