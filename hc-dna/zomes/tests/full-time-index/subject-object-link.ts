import { localConductorConfig, installation } from '../common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("Subject object link test", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        //Create another index for one day ago
        var dateOffset = (24*60*60*1000) / 2; //12 hr ago
        var date = new Date();
        date.setTime(date.getTime() - dateOffset);
    
        const source = "subject-2";
        const target = "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9";
        //Test case where subject and object are given
        await alice_sc_happ.cells[0].call("social_context", "add_link",  {
            linkExpression: {
                data: {
                    source, target, predicate: null
                },
                author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"},
            },
            indexStrategy: {
                type: "FullWithWildCard"
            },
        })
    
        //Get links on subject; expect back object & predicate
        const subj_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source, target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_links2.length, 1);
    
        //Get links on subject & object; expect back link 
        const subj_obj_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source, target, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(subj_obj_links2.length, 1);
    
        //Get links on object; expect back subject and predicate
        const object_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(object_links2.length, 1);
    
        //Get links on object & predicate; expect back none
        const object_pred_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target, predicate: "predicate-2", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(object_pred_links2.length, 0);
    
        //Get links on predicate; expect back none
        const pred_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
          {source: null, target: null, predicate: "predicate-2", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
        t.deepEqual(pred_links2.length, 0);
    })
}