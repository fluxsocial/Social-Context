import { localConductorConfig, installation, sleep } from './common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("Link delete", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        //Create another index for one day ago
        var dateOffset = (24*60*60*1000) / 2; //12 hr ago
        var date = new Date();
        date.setTime(date.getTime() - dateOffset);
    
        let link_data = { data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
        author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"}};
    
        //Create link
        await alice_sc_happ.cells[0].call("social_context", "add_link", link_data);
    
        console.log("Getting links");
        
        //Get links on subject
        const subj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_links.length, 1);
    
        console.log("Removing link");
        await alice_sc_happ.cells[0].call("social_context", "remove_link", link_data);
        await sleep(1000);
    
        console.log("Getting links");
    
        //Get links on subject
        const subj_links_pd = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_links_pd.length, 0);
    })
}