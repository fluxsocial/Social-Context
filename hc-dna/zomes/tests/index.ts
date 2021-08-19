/// This test file tests the functioning of social context w/ time index enabled & signals disabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'

let orchestrator = new Orchestrator()
require('./subject-predicate-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./subject-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./subject-predicate-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./link-delete')(orchestrator)
orchestrator.run()
