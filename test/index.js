/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example



const path = require('path')

const { Orchestrator, Config, combine, singleConductor, localOnly, tapeExecutor } = require('@holochain/tryorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});


const dnaPath = path.join(__dirname, "../dist/Converge19.dna.json")
console.log("dnaPath=", dnaPath)



const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape')),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly,

    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor,
  ),
})

const dna = Config.dna(dnaPath, 'converge')
const conductorConfig = Config.gen({newPossibility: dna}, {
	network: {
		type: 'sim2h',
		sim2h_url: 'ws://localhost:9000',
	}
})

orchestrator.registerScenario("can create and retrieve a possibility", async (s, t) => {

  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig}, true)

  var entry = {"title": "Game Night",
                 "description": "Winners and losers",
                 "suggested_min": 4,
                 "suggested_max": 8,
                 "criteria": [
                   {
                     "name": "My Criteria", 
                     "description":"Must have already recovered from the virus."
                    }
                 ]}

  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("newPossibility", "converge", "create_possibility", {"entry" :
                                                                                  entry })

  // Wait for all network activity to settle
  await s.consistency()

  const result = await bob.call("newPossibility", "converge", "get_possibility", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result.Ok, entry)
})

orchestrator.run()
