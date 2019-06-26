module.exports = scenario => {

  scenario("Create and get single community", async (s, t, { alice }) => {
    const name = "Test Community 1"
    const slug = "test1"
    const add_community_result = await alice.callSync("communities", "create", {
      name,
      slug
    })

    const { address } = add_community_result.Ok
    t.equal(address.length, 46)
    t.equal(add_community_result.Ok.name, name)
    t.equal(add_community_result.Ok.slug, slug)    

    const communityResult = {address, name, slug}

    const get_community_result = await alice.callSync("communities", "get", {
      address
    })
    t.equal(get_community_result.Ok.address, address)
    t.equal(get_community_result.Ok.name, name)
    t.equal(get_community_result.Ok.slug, slug)

    const get_by_slug_result = await alice.callSync("communities", "get_by_slug", {
      slug
    })
    t.equal(get_by_slug_result.Ok.address, address)
    t.equal(get_by_slug_result.Ok.name, name)

    const get_communities_result = await alice.callSync("communities", "all", {
    })
    t.deepEqual(get_communities_result.Ok, [communityResult], "Could retrieve the added community from the base")
  })
}
