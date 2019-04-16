const queries = require('../queries')
module.exports = (scenario) => {

scenario.runTape('Can create a new community via graphql', async (t, {alice}) => {
    let register_response = await alice.callSync("graphql", "graphql", {
      query: queries.registerQuery,
      variables: {id: "000", name: "wollum", avatarUrl: "//"}
    })
    console.log(register_response)

    // create a comment
    const addResult = await alice.callSync("graphql", "graphql", {
      query: queries.createCommunityQuery,
      variables: {
        base: "a base string to link from",
        name: "new graphql community",
        slug: "this is the url"
      }
    })
    console.log(addResult)
    let communityId = JSON.parse(addResult.Ok).createCommunity.id
    t.equal(communityId.length, 46) // thread was created and hash returned

    // retrieve community
    const getResult = await alice.callSync("graphql", "graphql", {
      query: queries.getCommunityQuery,
      variables: {id: communityId}
    })
    console.log(getResult)
    let communityName = JSON.parse(getResult.Ok).community.name
    t.equal(communityName, "new graphql community") // thread was created and hash returned
  })
}