/// This test files tests the functioning of the social context with time_index & signals enabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator, Config, InstallAgentsHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType, NetworkType } from '@holochain/tryorama'
import { HoloHash, InstallAppRequest } from '@holochain/conductor-api'
import * as msgpack from '@msgpack/msgpack';
import path from 'path'
import blake2b from "blake2b";

// Set up a Conductor configuration using the handy `Conductor.config` helper.
// Read the docs for more on configuration.
const network = {
  network_type: NetworkType.QuicBootstrap,
  transport_pool: [{
    type: TransportConfigType.Proxy,
    sub_transport: {type: TransportConfigType.Quic},
    proxy_config: {
      type: ProxyConfigType.LocalProxyServer,
      proxy_accept_config: ProxyAcceptConfig.AcceptAll
    }
  }],
  bootstrap_service: "https://bootstrap.holo.host",
}
//const conductorConfig = Config.gen({network});
const conductorConfig = Config.gen();

const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [path.join("../../workdir/social-context.dna")]
  ]
]

const orchestrator = new Orchestrator()

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

orchestrator.registerScenario("basic link signal testing", async (s, t) => {
    const [alice, bob] = await s.players([conductorConfig, conductorConfig])
    let aliceSignalCount = 0;
    let bobSignalCount = 0;
    alice.setSignalHandler((signal) => {
        console.log("Alice Received Signal:",signal)
        aliceSignalCount += 1;
    });
    bob.setSignalHandler((signal) => {
        console.log("Bob Received Signal:",signal)
        bobSignalCount += 1;
    });
    const [[alice_sc_happ]] = await alice.installAgentsHapps(installation)
    const [[bob_sc_happ]] = await bob.installAgentsHapps(installation)
    //Create another index for one day ago
    var dateOffset = (24*60*60*1000) / 2; //12 hr ago
    var date = new Date();
    date.setTime(date.getTime() - dateOffset);

    //Register as active agent
    await alice_sc_happ.cells[0].call("social_context", "add_link", {
        link: {
            data: {source: "active_agent", target: alice_sc_happ.agent.toString("hex"), predicate: "*"}, 
            author: {did: "alice", name: null, email: null}, 
            proof: {signature: "sig", key: "key"},
            timestamp: new Date().toISOString()
        },
        index_strategy: "Simple"
    })

    //Register as active agent
    await bob_sc_happ.cells[0].call("social_context", "add_link", {
        link: {
            data: {source: "active_agent", target: bob_sc_happ.agent.toString("hex"), predicate: "*"}, 
            author: {did: "bob", name: null, email: null}, 
            proof: {signature: "sig", key: "key"},
            timestamp: new Date().toISOString()
        },
        index_strategy: "Simple"
    })
     
    //Test case where subject object and predicate are given
    await alice_sc_happ.cells[0].call("social_context", "add_link",  { link: {data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
    author: {did: "test1", name: null, email: null}, timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"} }, index_strategy: "Simple" })

    t.deepEqual(aliceSignalCount, 2);
    t.deepEqual(bobSignalCount, 1);
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)