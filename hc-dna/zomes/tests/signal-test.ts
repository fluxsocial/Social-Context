/// This test files tests the functioning of the social context with time_index & signals enabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'
import { localConductorConfig, installation, sleep } from './common'

const orchestrator = new Orchestrator()

orchestrator.registerScenario("basic link signal testing", async (s, t) => {
    const [alice, bob] = await s.players([localConductorConfig, localConductorConfig])
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
    await s.shareAllNodes([alice, bob])

    //Register as active agent
    await alice_sc_happ.cells[0].call("social_context", "add_active_agent_link", null)

    //Register as active agent
    await bob_sc_happ.cells[0].call("social_context", "add_active_agent_link", null)

    //Sleep to give time for bob active agent link to arrive at alice
    await sleep(2000)
    //Test case where subject object and predicate are given
    await alice_sc_happ.cells[0].call(
        "social_context","add_link",
        {
             linkExpression: {
                data: {source: "subject-full", target: "object-full", predicate: "predicate-full"},
                author: "test1", timestamp: new Date().toISOString(), proof: {signature: "sig", key: "key"},
             },
             indexStrategy: {
                 type: "Full"
             },
        }
    )
    //Sleep to give time for signals to arrive
    await sleep(2000)

    t.deepEqual(aliceSignalCount, 1);
    t.deepEqual(bobSignalCount, 1);
})

orchestrator.run()