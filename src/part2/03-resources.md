## Chapter 3: Resources

A resource in Basis, which [extends the concept of a resource as modeled by ValueFlows][resources-vf], can be anything that's useful in a productive sense, either as an input to production or an output of production. A resource could be a house, a river, a chair, a factory, a farm, a tractor, etc. It's important to note that in Basis, resources must be explicitly entered. There must be intention behind a resource existing in the system.

Ownership of all resources described in the protocol is shared by all agents *equally*. An agent in India is an equal owner of the factory in Canada, and an agent in Hawaii is an equal owner of widget batch #506771 produced by Widgets"R"Us in Africa.

Everyone owning everything might be a fact of the protocol, but it's not a particularly useful distinction. Resources in Basis have two more relationships (beyond ownership) that apply: stewardship and use.

### Stewardship

Stewardship is a relationship between a [bloc] and a resource. Stewardship allows determining how a resource can be used or consumed. This can include setting time limits on use, determining costs of use, and any other facet that might arise. For instance, a farming guild bloc might have tractors available for local farmers to use. A chair production bloc might make their chairs available to order for other blocs or for consumption by agents. Blocs don't have to be productive either: a housing bloc, which stewards houses or apartments, can determine the rules for use of their units. A municipal bloc might set hours of acceptable use on a public park.

### Use

Use is a relationship in which a resource is in active consumption by either an agent or another bloc. Usage of a resource might often happen by members of the bloc in stewardship, but this isn't a given, and anyone can use a resource within the bounds set by the steward.

Usage could be something like living in a house, taking a bus, operating a lathe, etc.

### Determining stewardship

The system we use for distributing "property" and resource stewardship is somewhat simple: agents and blocs negotiate this amongst themselves. There is no protocol-defined criteria for who stewards what and why. Trying to devise a framework which can determine optimal placement of resources for any set of people or circumstances is futile, and any attempt would be brittle. So we don't.

As a general guideline, resources are best stewarded by the groups of people in use of them. However, while Basis strives to quantify some forms of use, it's impossible to do so for *all* types of use, so participants must determine this for themselves. And to a large extent, people already do this in the current system.

#### Reassigning stewardship

TODO:

- [Reassigning stewardship](https://github.com/basisproject/tracker/issues/134)

### Resource costs

As mentioned, resources in Basis extend resources in ValueFlows. Basis adds two properties to VF resources: who is in current use of the resource and a set of costs associated with the resource.

What exactly costs are is expanded in more detail in the [costs] section of the paper, however it's worth mentioning here. Costs are useful to think about in the context of usefulness of resources: a building that took labor and resources to construct, an inventory item for a factory that's consumed for the creation of another product, or even some set of land that requires ongoing maintenance like a forest or a park. Almost all useful resources generally have some cost associated with them.

Note that while resources can hold a *cost*, the bloc in stewardship of that resource is responsible for the cost.

[bloc]: #chapter-4-blocs
[costs]: #costs
[resources-vf]: https://www.valueflo.ws/concepts/resources/

