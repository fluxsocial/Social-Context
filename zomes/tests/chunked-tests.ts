import { Orchestrator, Config, InstallAgentsHapps, ConfigSeed, InstallHapp, InstalledHapps } from '@holochain/tryorama'
import { TransportConfigType, ProxyAcceptConfig, ProxyConfigType } from '@holochain/tryorama'
import { HoloHash } from '@holochain/conductor-api'
import path from 'path'
import blake2b from 'blake2b'

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

const sample_entry_address = Buffer.from([132, 41, 36, 152, 75, 230, 90, 132, 255, 226, 123, 128, 91, 101, 140, 101, 81, 59, 25, 154, 90, 104, 24, 14, 40, 255, 86, 43, 199, 71, 78, 232, 9, 217, 198, 124, 106, 153, 126])
const sample_entry_address2 = Buffer.from([132, 41, 36, 218, 236, 8, 48, 108, 119, 131, 20, 1, 217, 228, 69, 187, 188, 88, 82, 180, 102, 60, 47, 145, 78, 105, 200, 4, 139, 0, 114, 160, 130, 164, 188, 53, 247, 97, 186])

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

orchestrator.registerScenario("post and read communication by dna & agent", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  let sample_dna_address = generateDnaAddress();
  //Create communication on chunk 0
  await alice_sc_happ.cells[0].call("social_context_chunked", "post", {dna: sample_dna_address, entry_address: sample_entry_address, chunk: 0})

  //Create second expression ref on same DNA different entry_address on chunk 1
  await alice_sc_happ.cells[0].call("social_context_chunked", "post", {dna: sample_dna_address, entry_address: sample_entry_address2, chunk: 1})

  //Read communications by DNA for chunk 0
  const get_communications_first_chunk = await bob_sc_happ.cells[0].call("social_context_chunked", "read_communications",
    {by_dna: sample_dna_address, by_agent: null, from_chunk: 0, to_chunk: 0})
  t.deepEqual(get_communications_first_chunk.length, 1)

  //Read communications by DNA for chunk 1
  const get_communications_second_chunk = await bob_sc_happ.cells[0].call("social_context_chunked", "read_communications",
    {by_dna: sample_dna_address, by_agent: null, from_chunk: 1, to_chunk: 1})
  t.deepEqual(get_communications_second_chunk.length, 1)

  //Read communications by DNA for chunk 0-1
  const get_communications_all_chunk = await bob_sc_happ.cells[0].call("social_context_chunked", "read_communications",
    {by_dna: sample_dna_address, by_agent: null, from_chunk: 0, to_chunk: 1})
  t.deepEqual(get_communications_all_chunk.length, 2)

  //Read communications by agent; verify this still works; note that chunking is not being used here
  const get_communications_by_agent = await bob_sc_happ.cells[0].call("social_context_chunked", "read_communications",
    {by_dna: null, by_agent: alice_sc_happ.agent, from_chunk: 0, to_chunk: 1})
  t.deepEqual(get_communications_by_agent.length, 2)
})

//Communication method test
orchestrator.registerScenario("get communication methods", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  //Create communication methods on chunk 0; should be created at chunk 0 
  await alice_sc_happ.cells[0].call("social_context_chunked", "register_communication_method", {dna_address: generateDnaAddress()})

  //Create a bunch of other communication methods that should overflow onto next chunk
  for (let i = 0; i < 32; i++) { 
    await bob_sc_happ.cells[0].call("social_context_chunked", "register_communication_method", {dna_address: generateDnaAddress()})
  }

  //Get communication methods on chunk 0; verify it is at soft limit
  const communication_methods = await alice_sc_happ.cells[0].call("social_context_chunked", "get_communication_methods", {from_chunk: 0, to_chunk: 0})
  t.deepEqual(communication_methods.length, 30)

  //Get communication methods on chunk 1; verify that left over communication methods are there
  const communication_methods_chunk2 = await alice_sc_happ.cells[0].call("social_context_chunked", "get_communication_methods", {from_chunk: 1, to_chunk: 1})
  t.deepEqual(communication_methods_chunk2.length, 2);
})

//User anchor test
orchestrator.registerScenario("get communication methods", async (s, t) => {
  //Init a bunch of users
  let configs: [ConfigSeed] = [conductorConfig]
  for (let i = 0; i < 31; i++) { 
    configs.push(conductorConfig);
  }
  const agents = await s.players(configs)

  const agents_happs: InstalledHapps = [agents[0].installAgentsHapps(installation)];
  await agents_happs[0][0].cells[0].call("social_context_chunked", "post", {dna: generateDnaAddress(), entry_address: sample_entry_address, chunk: 0});
  for (let i = 1; i < 32; i++) {
    let [
      [current_agent],
    ] = await agents[i].installAgentsHapps(installation)
    await current_agent.cells[0].call("social_context_chunked", "post", {dna: generateDnaAddress(), entry_address: sample_entry_address, chunk: 0});
    agents_happs.push(current_agent);
  };

  //Make some more agents as I cant seem to use the last ones
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  await alice_sc_happ.cells[0].call("social_context_chunked", "post", {dna: generateDnaAddress(), entry_address: sample_entry_address, chunk: 0});
  await bob_sc_happ.cells[0].call("social_context_chunked", "post", {dna: generateDnaAddress(), entry_address: sample_entry_address, chunk: 0});

  //Test that first chunk has soft limit number of links
  const members = await alice_sc_happ.cells[0].call("social_context_chunked", "members", {from_chunk: 0, to_chunk: 0})
  t.deepEqual(members.length, 30)

  //Check that second chunk has leftover links on it; this is 3 not 4 as first agent never ran any zome functions
  const members_chunk2 = await bob_sc_happ.cells[0].call("social_context_chunked", "members", {from_chunk: 1, to_chunk: 1})
  t.deepEqual(members_chunk2.length, 3)
})

//Test the creating of communication methods on chunks from multiple agents; right now this is failing due to eventual consistency of hc; this was expected.
//Once validation rules are in place then this can be ran again to be sure that links are kept under hard limit
// orchestrator.registerScenario("get communication methods async", async (s, t) => {
//   const [alice, bob] = await s.players([conductorConfig, conductorConfig])

//   const [
//     [alice_sc_happ],
//   ] = await alice.installAgentsHapps(installation)
//   const [
//     [bob_sc_happ],
//   ] = await bob.installAgentsHapps(installation)

//   //Create communication methods on chunk 0; should be created at chunk 0 
//   await alice_sc_happ.cells[0].call("social_context_chunked", "register_communication_method", {dna_address: generateDnaAddress()})

//   //Create a bunch of other communication methods that should overflow onto next chunk
//   for (let i = 0; i < 32; i++) { 
//     if (randomIntFromInterval(0, 1) == 0) {
//       await bob_sc_happ.cells[0].call("social_context_chunked", "register_communication_method", {dna_address: generateDnaAddress()})
//     } else {
//       await alice_sc_happ.cells[0].call("social_context_chunked", "register_communication_method", {dna_address: generateDnaAddress()})
//     }
//   }

//   //Get communication methods on chunk 0; verify it is at soft limit
//   const communication_methods = await alice_sc_happ.cells[0].call("social_context_chunked", "get_communication_methods", {from_chunk: 0, to_chunk: 0})
//   t.deepEqual(communication_methods.length, 30)

//   //Get communication methods on chunk 1; verify that left over communication methods are there
//   const communication_methods_chunk2 = await bob_sc_happ.cells[0].call("social_context_chunked", "get_communication_methods", {from_chunk: 1, to_chunk: 1})
//   t.deepEqual(communication_methods_chunk2.length, 2);
// })

function randomIntFromInterval(min, max) { // min and max included 
  return Math.floor(Math.random() * (max - min + 1) + min);
}

function generateDnaAddress() {
  var output = new Uint8Array(32)
  let input = Buffer.from(makeid(10));
  let hash = blake2b(output.length).update(input).digest();

  return Buffer.from(concatTypedArrays(Uint8Array.from([132, 45, 36]), hash));
}

function makeid(length) {
  var result           = '';
  var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  var charactersLength = characters.length;
  for ( var i = 0; i < length; i++ ) {
     result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

function concatTypedArrays(a, b) { // a, b TypedArray of same type
  var c = new (a.constructor)(a.length + b.length);
  c.set(a, 0);
  c.set(b, a.length);
  return c;
}

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)