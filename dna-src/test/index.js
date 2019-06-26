const path = require('path')
const tape = require('tape')
const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

// const { Config, Scenario } = require('@holochain/holochain-nodejs')
// Scenario.setTape(require('tape'))
const dnaPath = path.join(__dirname, "../dist/dna-src.dna.json")
const dna = Diorama.dna(dnaPath, 'hylo-messenger')

// const agentAlice = Config.agent('alice')
// const instanceAlice = Config.instance(agentAlice, dna)
// const singleAgentScenario = new Scenario([instanceAlice], { debugLog: true })

// const agentBob = Config.agent('bob')
// const instanceBob = Config.instance(agentBob, dna)
// const twoAgentScenario = new Scenario([instanceAlice, instanceBob], { debugLog: true })

const singleAgentDiorama = new Diorama({
  instances: {
    alice: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

const twoAgentDiorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

require('./agent/communities')(singleAgentDiorama.registerScenario)
require('./agent/posts')(singleAgentDiorama.registerScenario)
require('./agent/comments')(singleAgentDiorama.registerScenario)
require('./agent/threads')(singleAgentDiorama.registerScenario)
require('./agent/messages')(singleAgentDiorama.registerScenario)
require('./agent/people')(twoAgentDiorama.registerScenario)

// disabled graphql tests
// require('./agent/register')(singleAgentScenario)
// require('./agent/gql_comments')(singleAgentScenario)
// require('./agent/gql_threads')(singleAgentScenario)
// require('./agent/gql_messages')(singleAgentScenario)
// require('./agent/gql_posts')(singleAgentScenario)
// require('./agent/gql_communities')(singleAgentScenario)

// singleAgentScenario.runTape('Reference GraphQL schema matches the implementation', async (t, {alice}) => {

// 	const fs = require('fs');
// 	const { buildSchema, buildClientSchema, introspectionQuery } = require('graphql');
// 	require('graphql-schema-utils');

// 	const referenceSchemaDef = fs.readFileSync('../schema.graphql', "utf8");
// 	const referenceSchema = buildSchema(referenceSchemaDef);

// 	const getSchemaResult = await alice.callSync("graphql", "graphql", {
// 	  	query: introspectionQuery,
// 		variables: {}
// 	})
// 	const implSchemaDef = JSON.parse(getSchemaResult.Ok)
// 	const implSchema = buildClientSchema(implSchemaDef)

// 	const diffs = referenceSchema.diff(implSchema).filter(d => !d.backwardsCompatible)

// 	if(diffs.length > 0) {
// 		console.log(diffs)
// 	}

// 	t.equal(diffs.length, 0)
// })


// require('./scenarios/retrieve_agents_people_query')(twoAgentScenario)

const run = async () => {
  await singleAgentDiorama.run()
  await twoAgentDiorama.run()
}

run()
