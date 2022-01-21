import { localConductorConfig, installation, sleep } from '../common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("wildcard test", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        let add_link_input = {
            linkExpression: {
                data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
                author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"}
            },
            indexStrategy: {
                type: "FullWithWildCard"
            },
        };
    
        //Create link
        await alice_sc_happ.cells[0].call("social_context", "add_link", add_link_input);
        
        //Get links on nothing; wildcard
        const wildcardLinks = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {})
        t.deepEqual(wildcardLinks.length, 1);

        //Get links on source only
        const wildcardLinksSource = await alice_sc_happ.cells[0].call("social_context", "get_links", 
            {source: "subject-full"})
        t.deepEqual(wildcardLinksSource.length, 1);

        //Get links on target only
        const wildcardLinksTarget = await alice_sc_happ.cells[0].call("social_context", "get_links", 
            {target: "object-full"})
        t.deepEqual(wildcardLinksTarget.length, 1);

        //Get links on predicate only
        const wildcardLinksPredicate = await alice_sc_happ.cells[0].call("social_context", "get_links", 
            {predicate: "predicate-full"})
        t.deepEqual(wildcardLinksPredicate.length, 1);
        
        await alice_sc_happ.cells[0].call("social_context", "remove_link", add_link_input.linkExpression);
        await sleep(1000);
  
        //Get links on subject
        const deletedWildcardLinks = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {})
        t.deepEqual(deletedWildcardLinks.length, 0);
    })
}