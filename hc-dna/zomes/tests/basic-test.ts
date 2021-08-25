/// This test file tests the functioning of the social context w/ time_index & signals disabled
/// NOTE: if all tests are run together then some will fail

import { Orchestrator } from '@holochain/tryorama'

let orchestrator = new Orchestrator()
require('./basic/subject-predicate-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./basic/subject-object-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./basic/subject-predicate-link')(orchestrator)
orchestrator.run()

orchestrator = new Orchestrator()
require('./basic/delete-link')(orchestrator)
orchestrator.run()
