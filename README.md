# Public-Social-Content

Holochain DNA that implements the Social-Context [trait](https://github.com/juntofoundation/Holochain-Trait-Definitions#social-context) for collective based entry sharing a DNA discovery. 
This DNA's implementation of the Social-Context trait is public and open; there is no kind of permission enforcing or membrane rules. Permission handling should be organised outside of the trait implementation and in some other zome/dna that implements a common known Permissions [trait](https://github.com/juntofoundation/Holochain-Trait-Definitions) (WIP).

This implementation also does not keep a "global" index of collective entries; they are only retrievable by dna address or user identity. This is due to DHT hotspotting concerns.

There is also no validation on commited GlobalEntryRef's to check if target dna address & entry address are valid. More work to be done in this department.