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

// Instatiate your test's orchestrator.
// It comes loaded with a lot default behavior which can be overridden, including:
// * custom conductor startup
// * custom test result reporting
// * scenario middleware, including integration with other test harnesses
const orchestrator = new Orchestrator()

//This is pretty annoying
const sample_dna_address = Buffer.from([132, 45, 36, 113, 136, 200, 19, 133, 133, 247, 57, 115, 177, 240, 222, 155, 1, 139, 201, 211, 194, 243, 204, 18, 126, 156, 114, 87, 85, 222, 170, 25, 156, 188, 109, 47, 3, 147, 52])
const sample_entry_address = Buffer.from([132, 41, 36, 152, 75, 230, 90, 132, 255, 226, 123, 128, 91, 101, 140, 101, 81, 59, 25, 154, 90, 104, 24, 14, 40, 255, 86, 43, 199, 71, 78, 232, 9, 217, 198, 124, 106, 153, 126])
const sample_entry_address2 = Buffer.from([132, 41, 36, 218, 236, 8, 48, 108, 119, 131, 20, 1, 217, 228, 69, 187, 188, 88, 82, 180, 102, 60, 47, 145, 78, 105, 200, 4, 139, 0, 114, 160, 130, 164, 188, 53, 247, 97, 186])

const sleep = (ms) => new Promise((resolve) => setTimeout(() => resolve(), ms));

orchestrator.registerScenario("post and read communication by dna & agent", async (s, t) => {
  // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
  // note that the first argument to players is just an array conductor configs that that will
  // be used to spin up the conductor processes which are returned in a matching array.
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  // install your happs into the conductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  //Create communication
  await alice_sc_happ.cells[0].call("social_context", "post", {dna: sample_dna_address, entry_address: sample_entry_address})

  //Create second expression ref on same DNA
  await alice_sc_happ.cells[0].call("social_context", "post", {dna: sample_dna_address, entry_address: sample_entry_address2})

  //Read communications by DNA
  const get_communications = await bob_sc_happ.cells[0].call("social_context", "read_communications", {by_dna: sample_dna_address, by_agent: null, count: 10, page: 0})
  t.deepEqual(get_communications.length, 2)

  //Read communications by agent
  const get_communications_by_agent = await bob_sc_happ.cells[0].call("social_context", "read_communications", {by_dna: null, by_agent: alice_sc_happ.agent, count: 10, page: 0})
  t.deepEqual(get_communications_by_agent.length, 2)
})

orchestrator.registerScenario("post and read communication methods", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  //Create communication method 
  await alice_sc_happ.cells[0].call("social_context", "register_communication_method", {dna_address: sample_dna_address})

  const get_methods = await bob_sc_happ.cells[0].call("social_context", "get_communication_methods", {dna_address: sample_dna_address, count: 10, page: 0})
  t.deepEqual(get_methods.length, 1)
})

orchestrator.registerScenario("read members", async (s, t) => {
  const [alice, bob] = await s.players([conductorConfig, conductorConfig])

  const [
    [alice_sc_happ],
  ] = await alice.installAgentsHapps(installation)
  const [
    [bob_sc_happ],
  ] = await bob.installAgentsHapps(installation)

  //Do something from bob and alice. This seems to ensure that both agents have their init fn run before calling members function
  await alice_sc_happ.cells[0].call("social_context", "post", {dna: sample_dna_address, entry_address: sample_entry_address})
  await bob_sc_happ.cells[0].call("social_context", "post", {dna: sample_dna_address, entry_address: sample_entry_address2})

  sleep(10000);

  const get_members_bob2 = await bob_sc_happ.cells[0].call("social_context", "members", {count: 0, page: 0})
  t.deepEqual(get_members_bob2.length, 2)

  const get_members2 = await alice_sc_happ.cells[0].call("social_context", "members", {count: 0, page: 0})
  t.deepEqual(get_members2.length, 2)
})

// Run all registered scenarios as a final step, and gather the report,
// if you set up a reporter
const report = orchestrator.run()

// Note: by default, there will be no report
console.log(report)