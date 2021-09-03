import { localConductorConfig, installation, sleep } from '../common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("Link delete", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        //Create another index for one day ago
        var dateOffset = (24*60*60*1000) / 2; //12 hr ago
        var date = new Date();
        date.setTime(date.getTime() - dateOffset);
    
        let now = new Date().toISOString();
        let add_link_input = { 
            linkExpression: {
                data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
                author: "test1", 
                timestamp: now, 
                proof: {signature: "sig", key: "key"},
            },
            indexStrategy: { simple: null },
        };
    
        //Create link
        await alice_sc_happ.cells[0].call("social_context", "add_link", add_link_input);
    
        console.log("Getting links");
        const subj_pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_pred_links.length, 1);

        //There are no links for source/target and other permutations, so no need to check after remove_link.
        const subj_obj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_obj_links.length, 0);
        
        console.log("Removing link");
        await alice_sc_happ.cells[0].call("social_context", "remove_link", add_link_input.linkExpression);
        await sleep(1000);
    
        console.log("Getting links");
        const subj_links_pd = await alice_sc_happ.cells[0].call("social_context", "get_links", 
        {source: "subject-full", target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_links_pd.length, 0);
    })
}