# Public-Social-Content

Holochain DNA that implements the Social-Context [trait](https://github.com/juntofoundation/Holochain-Trait-Definitions#social-context) for collective based entry sharing a DNA discovery. 
This Repo contains two implementation of the Social-Context trait. A private implementation and a public one. The private implementation also includes a social_graph zome. This zome is developed in accordance with the Social-Graph [trait](https://github.com/juntofoundation/Holochain-Trait-Definitions#social-graph). 

There is no validation on commited GlobalEntryRef's to check if target dna address & entry address are valid. More work to be done in this department.

## Validation

Implementation plan:

1. Define "admin" users for every DNA, writing their addresses in DNA properties. This list would be fairly small, which is an upside, but it will make our permission system more rigid and require us to create a new DNA once admins change. I don't yet have any thoughts on how to work around that.
2. Define another DNA property for the default security approach: either "deny all" or "allow all". That would determine the context in which we process entries created in this DNA. (That thing is optional and can be omitted, I'm still uncertain).
3. Define "permission entry": a single entry that can only be modified by admin users and is discoverable through a hard-coded anchor (like "permissions") . At any moment in time, latest version of this entry will keep the full definition of things that are allowed/denied in this DNA (If using idea from p2, it could only contain one list, or it could contain both if we skip p2). All new versions of this permission entry will be committed as `update_entry` so we can rely on an update chain for linking and knowing if there's a new data.
4. When creating link entries, require a hash of the current permission entry that user has access to. If there's a newer version of permission entry available, fail the validation and require to attempt again with the newest permission hash. Forbid updating permission entry hash at all. This way we'll ensure that every user is only creating link entry using latest available permissions and validation is truly deterministic.
5. When validating link entries, use deterministic permission entry address in the link entry being validated.
6. Make a callback that's called on every permission update and emphasize to users that changing permissions may be expensive. The callback will go through all the entries that would be invalid according to this update (like if you posted link with language A on Mon when it was ok, but then on Tue somebody denied usage of A) and mark them as hidden or something. They would still be valid according to the permission version that's specified in their field, but will be hidden from users to comply with the new rules.