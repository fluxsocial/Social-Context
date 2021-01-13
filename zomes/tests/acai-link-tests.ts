import { Orchestrator, Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import { HoloHash } from '@holochain/conductor-api'
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
const conductorConfig = Config.gen({network});
//const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const socialContext = path.join(__dirname, '../../social-context.dna.gz')

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [socialContext] // contains 1 dna, the "social-context" dna
  ]
]

const orchestrator = new Orchestrator()
const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

orchestrator.registerScenario("basic link testing", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig])

    const [
    [alice_sc_happ],
    ] = await alice.installAgentsHapps(installation)
    const [
    [bob_sc_happ],
    ] = await bob.installAgentsHapps(installation)

    /// SIMPLE LINK TEST
     
    //Test case where subject object and predicate are given
    await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
    author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    //Get links on subject; expect back object & predicate
    const subj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-full", target: null, predicate: null})
    t.deepEqual(subj_links.length, 1);
    console.log("subject links", subj_links);

    //Get links on subject & object; expect back predicate 
    const subj_obj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-full", target: "object-full", predicate: null})
    t.deepEqual(subj_obj_links.length, 1);
    console.log("subject object links", subj_obj_links);

    //Get links on object; expect back subject and predicate
    const object_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-full", predicate: null})
    t.deepEqual(object_links.length, 1);
    console.log("object links", object_links);

    //Get links on object & predicate; expect back subject
    const object_pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-full", predicate: "predicate-full"})
    t.deepEqual(object_pred_links.length, 1);
    console.log("object predicate links", object_pred_links)

    //Get links on predicate; expect back subject and object
    const pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: null, predicate: "predicate-full"})
    t.deepEqual(pred_links.length, 1);
    console.log("predicate links", pred_links)

    /// SUBJECT OBJECT LINK TEST

    //Test case where subject and object are given
    await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {source: "subject-2", target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null},
    author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    //Get links on subject; expect back object & predicate
    const subj_links2 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-2", target: null, predicate: null})
    t.deepEqual(subj_links2.length, 1);
    console.log("subject links", subj_links2);

    //Get links on subject & object; expect back link 
    const subj_obj_links2 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-2", target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null})
    t.deepEqual(subj_obj_links2.length, 1);
    console.log("subject object links", subj_obj_links2);

    //Get links on object; expect back subject and predicate
    const object_links2 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: null})
    t.deepEqual(object_links2.length, 1);
    console.log("object links", object_links2);

    //Get links on object & predicate; expect back none
    const object_pred_links2 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "Qmd6AZzLjfGWNAqWLGTGy354JC1bK26XNf7rTEEsJfv7Fe://Qmdrbjto9DDbUY8eMALPfmB35xh9m2Yce8ksk1NkMEZnQ9", predicate: "predicate-2"})
    t.deepEqual(object_pred_links2.length, 0);
    console.log("object predicate links", object_pred_links2)

    //Get links on predicate; expect back none
    const pred_links2 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: null, predicate: "predicate-2"})
    t.deepEqual(pred_links2.length, 0);
    console.log("predicate links", pred_links2)

    //Test case where subject and predicate are given

    await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {source: "subject-3", target: null, predicate: "predicate-3"},
    author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    //Get links on subject
    const subj_links3 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-3", target: null, predicate: null})
    t.deepEqual(subj_links3.length, 1);
    console.log("subject links", subj_links3);

    //Get links on subject & object
    const subj_obj_links3 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-3", target: "object-3", predicate: null})
    t.deepEqual(subj_obj_links3.length, 0);
    console.log("subject object links", subj_obj_links3);

    //Get links on object
    const object_links3 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-3", predicate: null})
    t.deepEqual(object_links3.length, 0);
    console.log("object links", object_links3);

    //Get links on object & predicate
    const object_pred_links3 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-3", predicate: "predicate-3"})
    t.deepEqual(object_pred_links3.length, 0);
    console.log("object predicate links", object_pred_links3)

    //Get links on predicate
    const pred_links3 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: null, predicate: "predicate-3"})
    t.deepEqual(pred_links3.length, 1);
    console.log("predicate links", pred_links3)

    //Test case where object and predicate are given

    await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {source: null, target: "object-4", predicate: "predicate-4"},
    author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    //Get links on subject
    const subj_links4 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-4", target: null, predicate: null})
    t.deepEqual(subj_links4.length, 0);
    console.log("subject links", subj_links4);

    //Get links on subject & object 
    const subj_obj_links4 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: "subject-4", target: "object-4", predicate: null})
    t.deepEqual(subj_obj_links4.length, 0);
    console.log("subject object links", subj_obj_links4);

    //Get links on object
    const object_links4 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-4", predicate: null})
    t.deepEqual(object_links4.length, 1);
    console.log("object links", object_links4);

    //Get links on object & predicate
    const object_pred_links4 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: "object-4", predicate: "predicate-4"})
    t.deepEqual(object_pred_links4.length, 1);
    console.log("object predicate links", object_pred_links4)

    //Get links on predicate
    const pred_links4 = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {source: null, target: null, predicate: "predicate-4"})
    t.deepEqual(pred_links4.length, 1);
    console.log("predicate links", pred_links4)
})

orchestrator.registerScenario("test get others", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
  [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
  [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)
   
  //Create a link so that the agent actually exists
  await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
  author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

  //Get links on subject; expect back object & predicate
  const others = await bob_sc_happ.cells[0].call("social_context_acai", "get_others", null)
  t.deepEqual(others.length, 1);
  console.log("others", others);
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)