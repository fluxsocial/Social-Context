import { localConductorConfig, installation } from './common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("Subject predicate link test", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        //Create another index for one day ago
        var dateOffset = (24*60*60*1000) / 2; //12 hr ago
        var date = new Date();
        date.setTime(date.getTime() - dateOffset);
    
        //Test case where subject and predicate are given
    
        await alice_sc_happ.cells[0].call("social_context", "add_link",  { data: {source: "subject-3", target: null, predicate: "predicate-3"},
        author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"}})
    
        //Get links on subject
        const subj_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-3", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_links3.length, 1);
        console.log("INT-TEST: subject links", subj_links3);
    
        //Get links on subject & object
        const subj_obj_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: "subject-3", target: "object-3", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_obj_links3.length, 0);
        console.log("INT-TEST: subject object links", subj_obj_links3);
    
        //Get links on object
        const object_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-3", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(object_links3.length, 0);
        console.log("INT-TEST: object links", object_links3);
    
        //Get links on object & predicate
        const object_pred_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: "object-3", predicate: "predicate-3", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(object_pred_links3.length, 0);
        console.log("INT-TEST: object predicate links", object_pred_links3)
    
        //Get links on predicate
        const pred_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: "predicate-3", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(pred_links3.length, 1);
        console.log("INT-TEST: predicate links", pred_links3)
    })
}