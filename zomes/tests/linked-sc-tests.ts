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

  //Emulate creating root link
  await alice_sc_happ.cells[0].call("social_context_link_store", "add_simple_link", {subject: "root://hash", object: "explang://exp", predicate: null})

  //Emulate creating language -> expression link
  await alice_sc_happ.cells[0].call("social_context_link_store", "add_simple_link", {subject: "explang://explang", object: "explang://exp", predicate: null})

  //Emulate creating agent -> expression link
  await alice_sc_happ.cells[0].call("social_context_link_store", "add_simple_link", {subject: "did://alice", object: "explang://exp", predicate: null})

  //Emulate creating agent -> language link
  await alice_sc_happ.cells[0].call("social_context_link_store", "add_simple_link", {subject: "did://alice", object: "explang://explang", predicate: null})

  //Get links on root
  const root_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "root://hash", predicate: null})
  t.deepEqual(root_links.length, 1);
  console.log("root links", root_links);

  const root_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: root_links[0].object, predicate: null})
  t.deepEqual(root_links_chunk.length, 1);
  console.log("Links on root first chunk", root_links_chunk);

  //Get links on exp lang
  const exp_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "explang://explang", predicate: null})
  t.deepEqual(exp_links.length, 1);
  console.log("exp links", exp_links);

  const exp_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: exp_links[0].object, predicate: null})
  t.deepEqual(exp_links_chunk.length, 2);
  console.log("Links on exp first chunk", exp_links_chunk);

  //Get links on alice
  const alice_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "did://alice", predicate: null})
  t.deepEqual(alice_links.length, 1);
  console.log("Alice links", alice_links);

  const alice_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: alice_links[0].object, predicate: null})
  t.deepEqual(alice_links_chunk.length, 2);
  console.log("Links on alice first chunk", alice_links_chunk);
})

//Still testing/playing
orchestrator.registerScenario("auto index link testing", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  //Emulate creating root link
  await alice_sc_happ.cells[0].call("social_context_link_store", "add_link_auto_index", {subject: "root://hash", object: "explang://exp", predicate: null})

  //Get links on root
  const root_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "root://hash", predicate: null})
  //t.deepEqual(root_links.length, 1);
  console.log("root links", root_links);

  const root_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: root_links[0].object, predicate: null})
 // t.deepEqual(root_links_chunk.length, 1);
  console.log("Links on root first chunk", root_links_chunk);

  //Get links on expression
  const exp_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "explang://exp", predicate: null})
  //t.deepEqual(exp_links.length, 1);
  console.log("exp links", exp_links);

  const exp_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: exp_links[0].object, predicate: null})
  //t.deepEqual(exp_links_chunk.length, 1);
  console.log("exp links first chunk", exp_links_chunk);

  //Get links on expression
  const exp_lang_links = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: "explang://explang", predicate: null})
  //t.deepEqual(exp_links.length, 1);
  console.log("exp lang links", exp_lang_links);

  const exp_lang_links_chunk = await alice_sc_happ.cells[0].call("social_context_link_store", "get_links", {subject: exp_links[0].object, predicate: null})
  //t.deepEqual(exp_lang_links_chunk.length, 1);
  console.log("exp lang links first chunk", exp_lang_links_chunk);
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)