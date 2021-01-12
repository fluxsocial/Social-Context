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
     
    // //Test case where subject object and predicate are given
    // await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {subject: "subject-full", object: "object-full", predicate: "predicate-full"},
    // author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    // //Get links on subject; expect back object & predicate
    // const subj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: "subject-full", object: null, predicate: null})
    // //t.deepEqual(subj_links.length, 1);
    // console.log("subject links", subj_links);

    // //Get links on subject & object; expect back predicate 
    // const subj_obj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: "subject-full", object: "object-full", predicate: null})
    // //t.deepEqual(subj_obj_links.length, 1);
    // console.log("subject object links", subj_obj_links);

    // //Get links on object; expect back subject and predicate
    // const object_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: "object-full", predicate: null})
    // //t.deepEqual(object_links.length, 1);
    // console.log("object links", object_links);

    // //Get links on object & predicate; expect back subject
    // const object_pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: "object-full", predicate: "predicate-full"})
    // //t.deepEqual(object_pred_links.length, 1);
    // console.log("object predicate links", object_pred_links)

    // //Get links on predicate; expect back subject and object
    // const pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: null, predicate: "predicate-full"})
    // //t.deepEqual(pred_links.length, 1);
    // console.log("predicate links", pred_links)

    /// SUBJECT OBJECT LINK TEST

    //Test case where subject and object are given
    await alice_sc_happ.cells[0].call("social_context_acai", "add_link",  { data: {subject: "subject-2", object: "object-2", predicate: null},
    author: {did: "test1", name: null, email: null}, timestamp: "iso8601", proof: {signature: "sig", key: "key"} })

    //Get links on subject; expect back object & predicate
    const subj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: "subject-2", object: null, predicate: null})
    //t.deepEqual(subj_links.length, 1);
    console.log("subject links", subj_links);

    //Get links on subject & object; expect back null 
    const subj_obj_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: "subject-2", object: "object-2", predicate: null})
    //t.deepEqual(subj_obj_links.length, 1);
    console.log("subject object links", subj_obj_links);

    //Get links on object; expect back subject and predicate
    const object_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: "object-2", predicate: null})
    //t.deepEqual(object_links.length, 1);
    console.log("object links", object_links);

    //Get links on object & predicate; expect back none
    const object_pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: "object-2", predicate: "predicate-2"})
    //t.deepEqual(object_pred_links.length, 1);
    console.log("object predicate links", object_pred_links)

    //Get links on predicate; expect back none
    const pred_links = await alice_sc_happ.cells[0].call("social_context_acai", "get_links", {subject: null, object: null, predicate: "predicate-2"})
    //t.deepEqual(pred_links.length, 1);
    console.log("predicate links", pred_links)

    //Test case where subject and predicate are given

    //Test case where object and predicate are given

    //NOTE: ensure that there are no overlap in links
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)