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
        let link_data = { 
                data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
                author: "test1", 
                timestamp: now, 
                proof: {signature: "sig", key: "key"}
        };
    
        //Create link
        await alice_sc_happ.cells[0].call("social_context", "add_link", link_data);
    
        //Getting links after add
        const subj_pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_pred_links.length, 1);

        const subj_obj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_obj_links.length, 1);

        const obj_pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-full", predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(obj_pred_links.length, 1);

        const subj_wild_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_wild_links.length, 1);

        const obj_wild_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(obj_wild_links.length, 1);

        const pred_wild_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(pred_wild_links.length, 1);

        const wild_wild_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(wild_wild_links.length, 1);
        
        //Remove links
        await alice_sc_happ.cells[0].call("social_context", "remove_link", link_data);
        await sleep(1000);
    
        //Getting links after remove
        const subj_pred_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
        {source: "subject-full", target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_pred_links_after.length, 0);

        const subj_obj_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_obj_links_after.length, 0);

        const obj_pred_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-full", predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(obj_pred_links_after.length, 0);

        const subj_wild_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_wild_links_after.length, 0);

        const obj_wild_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(obj_wild_links_after.length, 0);

        const pred_wild_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(pred_wild_links_after.length, 0);

        const wild_wild_links_after = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(wild_wild_links_after.length, 0);
    })
}