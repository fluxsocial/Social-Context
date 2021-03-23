import { Orchestrator, Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import { HoloHash, InstallAppRequest } from '@holochain/conductor-api'
import * as msgpack from '@msgpack/msgpack';
import path from 'path'

// Set up a Conductor configuration using the handy `Conductor.config` helper.
// Read the docs for more on configuration.
const network = {
    transport_pool: [{
      type: TransportConfigType.Proxy,
      sub_transport: {type: TransportConfigType.Quic},
      proxy_config: {
        type: ProxyConfigType.LocalProxyServer,
        proxy_accept_config: ProxyAcceptConfig.AcceptAll
      }
    }],
    bootstrap_service: "https://bootstrap.holo.host"
};
//const conductorConfig = Config.gen({network});
const conductorConfig = Config.gen();

const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [path.join("../../social-context.dna.gz")]
  ]
]

const orchestrator = new Orchestrator()

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

orchestrator.registerScenario("basic link testing", async (s, t) => {
    const [alice] = await s.players([conductorConfig])

    const req: InstallAppRequest = {
      installed_app_id: `my_app:1234`, // my_app with some unique installed id value
      agent_key: await alice.adminWs().generateAgentPubKey(),
      dnas: [{
        path: path.join(__dirname, '../../social-context.dna.gz'),
        nick: `my_cell_nick`,
        properties: {
          "enforce_spam_limit": 20,
          "max_chunk_interval": 100000,
          "active_agent_duration_ms": 7200
        },
        //membrane_proof: Array.from(msgpack.encode({role:"steward", signature:"..."})),
      }]
    }
    const alice_sc_happ = await alice._installHapp(req)
    //Create another index for one day ago
    var dateOffset = (24*60*60*1000) / 2; //12 hr ago
    var date = new Date();
    date.setTime(date.getTime() - dateOffset);

    /// SIMPLE LINK TEST
     
    //Test case where subject object and predicate are given
    await alice_sc_happ.cells[0].call("social_context", "add_link",  { data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
    author: {did: "test1", name: null, email: null}, timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"} })

    //Get links on subject; expect back object & predicate
    const subj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_links.length, 1);
    console.log("INT-TEST: subject links", subj_links);

    //Get links on subject & object; expect back predicate 
    const subj_obj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-full", target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_obj_links.length, 1);
    console.log("INT-TEST: subject object links", subj_obj_links);

    //Get links on object; expect back subject and predicate
    const object_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "object-full", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_links.length, 1);
    console.log("INT-TEST: object links", object_links);

    //Get links on object & predicate; expect back subject
    const object_pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "object-full", predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_pred_links.length, 1);
    console.log("INT-TEST: object predicate links", object_pred_links)

    //Get links on predicate; expect back subject and object
    const pred_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: null, predicate: "predicate-full", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(pred_links.length, 1);
    console.log("INT-TEST: predicate links", pred_links)
})

orchestrator.registerScenario("Subject object link test", async (s, t) => {
    const [alice] = await s.players([conductorConfig])

    const req: InstallAppRequest = {
      installed_app_id: `my_app:1234`, // my_app with some unique installed id value
      agent_key: await alice.adminWs().generateAgentPubKey(),
      dnas: [{
        path: path.join(__dirname, '../../social-context.dna.gz'),
        nick: `my_cell_nick`,
        properties: {
          "enforce_spam_limit": 20,
          "max_chunk_interval": 100000
        },
        //membrane_proof: Array.from(msgpack.encode({role:"steward", signature:"..."})),
      }]
    }
    const alice_sc_happ = await alice._installHapp(req)
    //Create another index for one day ago
    var dateOffset = (24*60*60*1000) / 2; //12 hr ago
    var date = new Date();
    date.setTime(date.getTime() - dateOffset);

    //Test case where subject and object are given
    await alice_sc_happ.cells[0].call("social_context", "add_link",  { data: {source: "subject-2", target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null},
    author: {did: "test1", name: null, email: null}, timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"} })

    //Get links on subject; expect back object & predicate
    const subj_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-2", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_links2.length, 1);
    console.log("INT-TEST: subject links", subj_links2);

    //Get links on subject & object; expect back link 
    const subj_obj_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-2", target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_obj_links2.length, 1);
    console.log("INT-TEST: subject object links", subj_obj_links2);

    //Get links on object; expect back subject and predicate
    const object_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_links2.length, 1);
    console.log("INT-TEST: object links", object_links2);

    //Get links on object & predicate; expect back none
    const object_pred_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: "predicate-2", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_pred_links2.length, 0);
    console.log("INT-TEST: object predicate links", object_pred_links2)

    //Get links on predicate; expect back none
    const pred_links2 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: null, predicate: "predicate-2", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(pred_links2.length, 0);
    console.log("INT-TEST: predicate links", pred_links2)
})

orchestrator.registerScenario("Subject predicate link test", async (s, t) => {
    const [alice] = await s.players([conductorConfig])

    const req: InstallAppRequest = {
      installed_app_id: `my_app:1234`, // my_app with some unique installed id value
      agent_key: await alice.adminWs().generateAgentPubKey(),
      dnas: [{
        path: path.join(__dirname, '../../social-context.dna.gz'),
        nick: `my_cell_nick`,
        properties: {
          "enforce_spam_limit": 20,
          "max_chunk_interval": 100000,
          "active_agent_duration_ms": 7200
        },
        //membrane_proof: Array.from(msgpack.encode({role:"steward", signature:"..."})),
      }]
    }
    const alice_sc_happ = await alice._installHapp(req)
    //Create another index for one day ago
    var dateOffset = (24*60*60*1000) / 2; //12 hr ago
    var date = new Date();
    date.setTime(date.getTime() - dateOffset);

    //Test case where subject and predicate are given

    await alice_sc_happ.cells[0].call("social_context", "add_link",  { data: {source: "subject-3", target: null, predicate: "predicate-3"},
    author: {did: "test1", name: null, email: null}, timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"} })

    //Get links on subject
    const subj_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-3", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_links3.length, 1);
    console.log("INT-TEST: subject links", subj_links3);

    //Get links on subject & object
    const subj_obj_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-3", target: "object-3", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_obj_links3.length, 0);
    console.log("INT-TEST: subject object links", subj_obj_links3);

    //Get links on object
    const object_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "object-3", predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_links3.length, 0);
    console.log("INT-TEST: object links", object_links3);

    //Get links on object & predicate
    const object_pred_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: "object-3", predicate: "predicate-3", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(object_pred_links3.length, 0);
    console.log("INT-TEST: object predicate links", object_pred_links3)

    //Get links on predicate
    const pred_links3 = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: null, target: null, predicate: "predicate-3", from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(pred_links3.length, 1);
    console.log("INT-TEST: predicate links", pred_links3)
})

    //Test case where object and predicate are given
orchestrator.registerScenario("Link delete", async (s, t) => {
    const [alice] = await s.players([conductorConfig])

    const req: InstallAppRequest = {
      installed_app_id: `my_app:1234`, // my_app with some unique installed id value
      agent_key: await alice.adminWs().generateAgentPubKey(),
      dnas: [{
        path: path.join(__dirname, '../../social-context.dna.gz'),
        nick: `my_cell_nick`,
        properties: {
          "enforce_spam_limit": 20,
          "max_chunk_interval": 100000,
          "active_agent_duration_ms": 7200
        },
        //membrane_proof: Array.from(msgpack.encode({role:"steward", signature:"..."})),
      }]
    }
    const alice_sc_happ = await alice._installHapp(req);

    //Create another index for one day ago
    var dateOffset = (24*60*60*1000) / 2; //12 hr ago
    var date = new Date();
    date.setTime(date.getTime() - dateOffset);

    let link_data = { data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
    author: {did: "test1", name: null, email: null}, timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"} };

    //Create link
    await alice_sc_happ.cells[0].call("social_context", "add_link", link_data);

    console.log("Getting links");
    const subj_links = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_links.length, 1);
    
    console.log("Removing link");
    await alice_sc_happ.cells[0].call("social_context", "remove_link", link_data);
    sleep(300000);

    console.log("Getting links");
    const subj_links_pd = await alice_sc_happ.cells[0].call("social_context", "get_links", 
      {source: "subject-full", target: null, predicate: null, from: date.toISOString(), until: new Date().toISOString()})
    t.deepEqual(subj_links_pd.length, 0);
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)