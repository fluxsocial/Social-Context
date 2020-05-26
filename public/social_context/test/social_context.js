/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require('path')

const { Orchestrator, Config, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/social_context.dna.json")
const dna = Config.dna(dnaPath, 'SocialContextDNA')

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,
  ),
  //   waiter: {
  //   hardTimeout: 100000,
  //   strict: true,
  // }
});

const conductorConfig = Config.gen(
  {
    SocialContext: dna,
  },
  {
    logger: {
      type: 'debug',
      state_dump: false,
      // rules: {
      //     rules: [{ exclude: true, pattern: ".*" }]
      // }
    },
    network: {
      type: 'sim2h',
      sim2h_url: 'ws://localhost:9000'
    }
  }
)

const sample_dna_address = "QmZ6mav8UBRzA5YzApoVRdUWQGCw4wgxBvEkkYN1sQXXkH"
const sample_entry_address = "14f5e1afcbfb2a7d617ddb3423d742b3959eb36100e3efdc481c1966b4d06858"
const sample_entry_address2 = "62ccd5f507d61e28fe590a6487e120d9bf87bf7d61a447c4ccddbc447382873e"

orchestrator.registerScenario("post and read communication by dna & agent", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)

  //Create communication
  const expression1 = await alice.call("SocialContext", "social_context", "post", {expression_ref: {dna_address: sample_dna_address, entry_address: sample_entry_address} })
  t.deepEqual(expression1.hasOwnProperty("Ok"), true)

  //Create second expression ref on same DNA
  const expression2 = await alice.call("SocialContext", "social_context", "post", {expression_ref: {dna_address: sample_dna_address, entry_address: sample_entry_address2} })
  t.deepEqual(expression2.hasOwnProperty("Ok"), true)
  await s.consistency()

  //Read communications by DNA
  const get_communications = await bob.call("SocialContext", "social_context", "read_communications", {by_dna: sample_dna_address, by_agent: null, count: 10, page: 0})
  t.deepEqual(get_communications.hasOwnProperty("Ok"), true)
  t.deepEqual(get_communications.Ok.length, 2)

  //Read communications by agent
  const get_communications_by_agent = await bob.call("SocialContext", "social_context", "read_communications", {by_dna: null, by_agent: alice.instance('SocialContext').agentAddress, count: 10, page: 0})
  t.deepEqual(get_communications_by_agent.hasOwnProperty("Ok"), true)
  t.deepEqual(get_communications_by_agent.Ok.length, 2)
})

// orchestrator.registerScenario("post and read communication methods", async (s, t) => {
//   const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
// })

// orchestrator.registerScenario("read members", async (s, t) => {
//   const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)
// })

orchestrator.run()
