/// This test file tests the functioning of social context w/ time index enabled & signals disabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'

let orchestrator = new Orchestrator()
require('./full-time-index/subject-predicate-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./full-time-index/subject-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./full-time-index/subject-predicate-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./full-time-index/delete-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./full-time-index/wildcard-test')(orchestrator)
orchestrator.run()
