import { localConductorConfig, installation } from '../common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("Subject object link test", async (s, t) => {
        const [alice] = await s.players([localConductorConfig])
        const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    
        //Create another index for one day ago
        var dateOffset = (24*60*60*1000) / 2; //12 hr ago
        var date = new Date();
        date.setTime(date.getTime() - dateOffset);
    
        let now = new Date().toISOString();
        //Test case where subject and object are given
        try {
            await alice_sc_happ.cells[0].call(
                "social_context",
                "add_link",
                {
                    linkExpression: {
                        data: {
                            source: "subject-2",
                            target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9",
                            predicate: null
                        },
                        author: "test1", timestamp: now, proof: {signature: "sig", key: "key"}
                    },
                    indexStrategy: {
                        type: "Simple"
                    },
                }
            );
        } catch(err) {
            t.ok(err, "Got expected error");
        }
        
        //Get links on subject; expect back nothing
        const subj_links2 = await alice_sc_happ.cells[0].call(
            "social_context",
            "get_links", 
            {
                source: "subject-2",
                target: null,
                predicate: null,
                from: date.toISOString(),
                until: new Date().toISOString(),
                limit: 10
            }
        );
        t.deepEqual(subj_links2.length, 0);
        console.log("INT-TEST: subject links", subj_links2);
    
        //Get links on subject & object; expect back nothing 
        const subj_obj_links2 = await alice_sc_happ.cells[0].call(
            "social_context",
            "get_links", 
            {
                source: "subject-2",
                target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9",
                predicate: null,
                from: date.toISOString(),
                until: new Date().toISOString(),
                limit: 10
            }
        );
        t.deepEqual(subj_obj_links2.length, 0);
        console.log("INT-TEST: subject object links", subj_obj_links2);
    
        //Get links on object; expect back nothing
        const object_links2 = await alice_sc_happ.cells[0].call(
            "social_context",
            "get_links", 
            {
                source: null,
                target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9",
                predicate: null,
                from: date.toISOString(),
                until: new Date().toISOString(),
                limit: 10
            }
        );
        t.deepEqual(object_links2.length, 0);
        console.log("INT-TEST: object links", object_links2);
    
        //Get links on object & predicate; expect back nothing
        const object_pred_links2 = await alice_sc_happ.cells[0].call(
            "social_context",
            "get_links", 
            {
                source: null,
                target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9",
                predicate: "predicate-2",
                from: date.toISOString(),
                until: new Date().toISOString(),
                limit: 10
            }
        );
        t.deepEqual(object_pred_links2.length, 0);
        console.log("INT-TEST: object predicate links", object_pred_links2)
    
        //Get links on predicate; expect back nothing
        const pred_links2 = await alice_sc_happ.cells[0].call(
            "social_context",
            "get_links", 
            {
                source: null,
                target: null,
                predicate: "predicate-2",
                from: date.toISOString(),
                until: new Date().toISOString(),
                limit: 10
            }
        );
        t.deepEqual(pred_links2.length, 0);
        console.log("INT-TEST: predicate links", pred_links2)
    })
}