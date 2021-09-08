import { Orchestrator } from '@holochain/tryorama'

let orchestrator = new Orchestrator()
require('./basic-full-index/delete-link')(orchestrator)
orchestrator.run()