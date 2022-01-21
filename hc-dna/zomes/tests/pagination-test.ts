/// This test file tests the functioning of the social context w/ time_index & signals disabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'

let orchestrator = new Orchestrator()
require('./pagination/ascending')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./pagination/descending-large-timeframe')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./pagination/descending')(orchestrator)
orchestrator.run()