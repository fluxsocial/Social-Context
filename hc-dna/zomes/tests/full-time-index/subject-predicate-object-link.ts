import { localConductorConfig, installation } from '../common'

module.exports = (orchestrator) => {
	orchestrator.registerScenario("basic link testing", async (s, t) => {
		const [alice] = await s.players([localConductorConfig])
		const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
		//Create another index for one day ago
		var dateOffset = (24*60*60*1000) / 2; //12 hr ago
		var date = new Date();
		date.setTime(date.getTime() - dateOffset);
	
		/// SIMPLE LINK TEST
		 
		//Test case where subject object and predicate are given
		await alice_sc_happ.cells[0].call("social_context", "add_link",  {
            linkExpression: {
                data: {
                    source: "subject-full", target: "object-full", predicate: "predicate-full"
                },
                author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"},
            },
            indexStrategy: {
                type: "FullWithWildCard"
            },
        })
	
		//Get links on subject; expect back object & predicate
		const subj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
		  {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
		t.deepEqual(subj_links.length, 1);
		console.log("INT-TEST: subject links", subj_links);
	
		//Get links on subject & object; expect back predicate 
		const subj_obj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
		  {source: "subject-full", target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
		t.deepEqual(subj_obj_links.length, 1);
		console.log("INT-TEST: subject object links", subj_obj_links);
	
		//Get links on object; expect back subject and predicate
		const object_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
		  {source: null, target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString(), limit: 10})
		t.deepEqual(object_links.length, 1);
		console.log("INT-TEST: object links", object_links);
	
		//Get links on object & predicate; expect back subject
		const object_pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
		  {source: null, target: "object-full", predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
		t.deepEqual(object_pred_links.length, 1);
		console.log("INT-TEST: object predicate links", object_pred_links)
	
		//Get links on predicate; expect back subject and object
		const pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
		  {source: null, target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString(), limit: 10})
		t.deepEqual(pred_links.length, 1);
		console.log("INT-TEST: predicate links", pred_links)
	})
}