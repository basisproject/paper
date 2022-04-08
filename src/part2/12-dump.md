## Dumping ground

### NOTES / dumping ground

### Cost tracking

#### Assigning costs to products and services

How costs are assigned to products and services by each bloc is completely up to them. They might want to automate this based on inputs and outputs over time. They might want to carefully assign costs manually for each product. They are free to handle this however they want, provided that their total costs are kept under their cost ceiling.

#### Labor costs

#### (Semi) raw material costs

Basis can track not just labor costs by occupation, but also costs of raw and semi-raw materials. In effect, if wood from a lumber mill is tagged as a tracked resource, anyone who orders that wood from the lumber yard will have the cost of that wood added to their total costs. For instance, if I make wooden chairs, I need lumber, so I order 10kg of it. The lumber has a cost of:

```
{
    "labor": {
        "mill worker": 8.0
    }
    "resources": {
        "wood": 10000.0
    }
}
```

If I pay myself 10₡/hr and I use that wood to build four chairs in one hour, the total cost would be (notice I added in my own labor to the costs):

```
{
    "labor": {
        "mill worker": 8.0
        "chair maker": 10.0
    }
    "resources": {
        "wood": 10000.0
    }
}
```

Now if I price my four chairs equally, the cost of each chair would be:

```
{
    "labor": {
        "mill worker": 2.0
        "chair maker": 2.5
    }
    "resources": {
        "wood": 2500.0
    }
}
```

If a company were to order one of my chairs, the above cost would be subtracted from my company's costs and added to their costs. If a consumer were to order one of my chairs, the costs are subtracted from my company and both the chair and the costs are marked as *consumed* and disappear from the system.

The above is a contrived example that ignores things like shipping, amortized costs of machinery, costs of property usage, etc. However, we can see how resource and labor costs accumulate, divide, and flow within and between companies.

#### Flows of costs

While this has been touched on already, it's important to note that costs can be created (via labor and resources) but that costs can only ever be destroyed by three processes: consumption, taxation, and market sales. Consumption means a consumer covers the cost personally, using credits they have earned via labor. Taxation is the process of spreading a company's costs equally among its members over time such that they use their personal credits to pay the cost. Lastly is market sales, where a company sells a product into the market system at which point the costs are wiped out of the system.

Within the productive network, costs are accumulated, divided, and transferred but are never destroyed.

### Raw material tracking and costing

By default, all products in Basis are categorized the same way: as a product! A hunk of coal or a barrel of oil is modeled the same way in the system as a pack of socks. So how do we tag certain products as tracked resources?

It makes sense that we would want to track raw materials (oil, silicon, iron) but it also might make sense to track resources that are taken one or two steps further: diesel fuel, steel, various chemicals. How it's decided to tag a product as a tracked resource is a matter of systemic governance.

It will also be the case that not just the resource type will be tagged, but also its source. Is an old-growth forest being decimated to make gift shop trinkets? Maybe producers or consumers would want to know this information. This helps us not just with individual knowledge, but also systemic knowledge: where are our resources coming from and at what rates? Having exact data on this could help us achieve much better ecological equilibrium.

Once we have our list of tagged resources, how exactly do we cost them? This is another issue of systemic governance. The process behind this is still being decided on.

TODO:

- [#17 - Raw material tagging](https://github.com/basisproject/tracker/issues/17)
- [#58 - Resource costing](https://github.com/basisproject/tracker/issues/58)
- [#90 - Raw material transformations](https://github.com/basisproject/tracker/issues/90)

### Pricing

So far we've talked a lot about costs. Does this mean that every product purchased by a consumer will be provided at-cost? No, and there two facets to this.

First, for members, the system will *optimize for* providing products at-cost. In other words, companies that do sell products for exactly their cost will be rewarded (more on this in the [cybernetics section](#cybernetics)). That said, companies can set whatever prices above or below cost they want. The difference from a capitalist market however is that when a company sells a widget for 10₡ that only cost 8₡ to produce, it doesn't realize the extra 2₡: the credits are still burned on purchase.

Secondly, for non-members, companies are incentivized to sell at the highest price they can. So each product that is available for public consumption will have at least two prices: the in-network price and the market price.

So why let companies set arbitrary prices at all? It enables things like clearing inventory for products that didn't sell well, and also it enables inventory control for higher-value items. Member companies don't realize the delta between cost and price as a gain or loss, but rather it is used as a signal to determine if the production of that particular product should be expanded or contracted. In other words, we retain the *signal mechanism* from pricing *without using it as a distribution mechanism*.

This system of pricing is described in "Towards a New Socialism" [^tns].


#### Consumer purchases

- Consumption of resources removes them from the protocol

##### Pricing

So far we've talked a lot about costs. Does this mean that every product purchased by a consumer will be provided at-cost? No, and there two facets to this.

First, for members, the system will *optimize for* providing products at-cost. In other words, companies that do sell products for exactly their cost will be rewarded (more on this in the [cybernetics section](#cybernetics)). That said, companies can set whatever prices above or below cost they want. The difference from a capitalist market however is that when a company sells a widget for 10₡ that only cost 8₡ to produce, it doesn't realize the extra 2₡: the credits are still burned on purchase.

Secondly, for non-members, companies are incentivized to sell at the highest price they can. So each product that is available for public consumption will have at least two prices: the in-network price and the market price.

So why let companies set arbitrary prices at all? It enables things like clearing inventory for products that didn't sell well, and also it enables inventory control for higher-value items. Member companies don't realize the delta between cost and price as a gain or loss, but rather it is used as a signal to determine if the production of that particular product should be expanded or contracted. In other words, we retain the *signal mechanism* from pricing *without using it as a distribution mechanism*.

This system of pricing is described in "Towards a New Socialism" [^2].


##### Cybernetics

Cybernetics is a system of self-regulating and automated control. We employ cybernetics to incentivize companies to act in the best interests of the network. This is done by adjusting a company's cost ceiling: the more the company acts in the interests of the network, the higher its cost ceiling. In effect, a high cost ceiling translates to a higher social investment.

TODO:

- [Cybernetics metrics and signals](https://github.com/basisproject/tracker/issues?q=is%3Aissue+is%3Aopen+label%3Atag%3Acybernetics)

##### Anonymity

While the transactions between member companies are transparent and freely observable by any member of the network, there are a few cases where privacy is offered:

- Consumer purchases. Whether a member or a non-member, purchasing goods from a member company is anonymous. The order is viewable by members, but looks like it came from the Basis system directly and is not tied back to the originator in any way. Only the user and the company have the full information on the order.
- Non-member orders. Any time a non-member is involved in an order, the order details will be available, but the non-member company will be anonymized. This is to protect the privacy of companies who do not wish for their purchasing to be transparent. Like consumer purchases, the order will look like it came from the Basis system directly, and the non-member company's identity will be anonymized.

TODO:

- [#4 - Privacy for consumer orders](https://github.com/basisproject/tracker/issues/4)

### Chapter 8: Agent-resource interactions

### Chatper 9: Agent-network interactions

### Chapter 10: Bloc-bloc interactions

Basis extends the ValueFlows protocol for its accounting within and between blocs as well as between blocs and agents. ValueFlows is an REA (Resource-Agent-Event) accounting system that allows tracking detailed flows of resources through an economy.

### Chapter 11: Bloc-resource interactions


---

### Chapter 6: Economics

Basis defines a system of economics with the end goal of eliminating production for profit and for compensating members based on their contribution.

Basis uses the [ValueFlows vocabulary](https://valueflo.ws) for much of its economic system, which provides a general set of operations that can describe and model an incredibly diverse set of economic scenarios. It should be noted that while Basis does use ValueFlows for whatever it can, it also layers some other things on top of it (like disaggregate cost tracking).

### Costs

Costs in Basis are not a number, but rather separate collections of individual costs bundled together.

Costs are bucketed by a few different types: `labor`, `labor_hours`, and `resources`.

- `labor` tracks how many wages were paid to a particular occupation in a set of costs. Occupations are tracked globally in the protocol and are standard across all blocs.
- `labor_hours` tracks how many hours were worked by a particular occupation in a set of costs. Occupations here are the same as in the `labor` object (standard and tracked globally).
- `resources` holds the quantity of the standard unit of [tracked resources][trackers] that exist in a set of costs.

These types can be extended to track other cost types (for instance `currency`, which will be [covered in part 3][part3]).

Here's an example of what a bloc's costs might look like:

```
{
    "labor": {
        "president": 12.89,
        "accountant": 3.38,
        "miner": 41.45
    },
    "labor_hours": {
        "president": 0.2578,
        "accountant": 0.0965,
        "miner": 1.0363
    },
    "resources": {
        "iron": 8.5,
        "gasoline": 2.9,
        "silicon": 0.03
    }
}
```

#### Memorialization

- TODO: describe how costs memorialize resource prices

#### Mathematical operations on costs

- TODO: give examples of addition and division

#### Converting costs to a single value

Although costs are tracked as separate collections of distinct values, it's important that they are able to be summed into one aggregate number: a total cost. This allows blocs to get a quick sense for their overall cost amounts, and as we'll see later, something called a [cost allowance].

Deriving this total value is simple in the case of `labor` and `labor_hours`, we simply sum the values together. For instance, in the object above, we would have `12.89 + 3.38 + 41.45 + 0.2578 + 0.0965 + 1.0363` or `59.1106`.

The tricky bit comes when we convert our resource values into a cost, and this is where [trackers] come in: they allow categorizing a [resource][resources] within the cost tracking system such that a standard unit can be assigned a cost value. Thus, if in our example above, `iron` is tracked in grams, and the cost-per-gram is `0.3` then the iron cost would be `8.5 * 0.3` or `2.55`. If done for all `resources` in the costs set, we can add this to our labor costs and find our total aggregate cost value.

### Cost tracking

Cost tracking is a very important concept in a socialist economy, especially absent the exchange of money between producing entities. Basis tracks costs on a per-bloc level based on that bloc's incoming and outgoing orders (what would be "sales" and "purchases" in capitalism). Because the protocol is also the medium of exchange between blocs, it's able to track costs automatically without the need for manual entry or auditing.

Each bloc has a set of costs and a cost ceiling. If a bloc's total costs reach their cost ceiling, that bloc will be unable to take on any more costs (such as ordering new inputs to production or paying members their wages) until they transfer some of them out, which generally happens when someone orders their products. The cost ceiling ensures that costs keep flowing and do not accumulate or stagnate too much. For instance, if a bloc's cost ceiling is 1000₡ and their current total costs are 990₡, they might want to seriously consider trying to get some other companies or consumers to order their products, otherwise they won't be able to pay themselves or order more inputs to production.

It's worth mentioning that cost tracking is more effective as the network grows: more participants transacting in-network means more accurate cost tracking. Remember, market prices obscure costs, so every time a bloc has to order out-of-network that's a cost we can't disaggregate, and every time a member bloc sells into the market, those costs are destroyed. The more participants, the more accurate, and ultimately useful, cost tracking will be.

#### Assigning costs to products and services

How costs are assigned to products and services by each bloc is completely up to them. They might want to automate this based on inputs and outputs over time. They might want to carefully assign costs manually for each product. They are free to handle this however they want, provided that their total costs are kept under their cost ceiling.

#### Labor costs

All things take labor to make or use. Apples must be picked. Iron must be mined. Chairs must be crafted.

Basis tracks labor in two ways. First, is occupation-wages, and secondly is occupation-hours. Occupation-wages track the credit values paid out to workers, and occupation-hours track labor in hours that occurred. This means if I work at the widget factory and I make 10₡/hr, then if I work 7.5 hour day, the occupation-wages would be tracked as  `{"factory-worker": 75}` and the occupation-hours would be tracked as `{"factory-worker": 7.5}`.

From the above, we see that we don't track just labor wages/hours in total, but rather bucket them by occupation. This gives us more disaggregate information that would otherwise be wiped out by a price: how much were the factory workers paid? How much were CEOs paid? How much was spent in marketing? Knowing these things about a product give us valuable insights into how it was produced.

Which occupations we track is a matter of systemic governance.

TODO:

- [#89 - Governance of occupation data](https://github.com/basisproject/tracker/issues/89)

#### (Semi) raw material costs

Basis can track not just labor costs by occupation, but also costs of raw and semi-raw materials. In effect, if wood from a lumber mill is tagged as a tracked resource, anyone who orders that wood from the lumber yard will have the cost of that wood added to their total costs. For instance, if I make wooden chairs, I need lumber, so I order 10kg of it. The lumber has a cost of:

```
{
    "labor": {
        "mill worker": 8.0
    },
    "resources": {
        "wood": 10000.0
    }
}
```

If I pay myself 10₡/hr and I use that wood to build four chairs in one hour, the total cost would be (notice I added in my own labor to the costs):

```
{
    "labor": {
        "mill worker": 8.0,
        "chair maker": 10.0
    },
    "resources": {
        "wood": 10000.0
    }
}
```

Now if I price my four chairs equally, the cost of each chair would be:

```
{
    "labor": {
        "mill worker": 2.0,
        "chair maker": 2.5
    },
    "resources": {
        "wood": 2500.0
    }
}
```

If a company were to order one of my chairs, the above cost would be subtracted from my company's costs and added to their costs. If a consumer were to order one of my chairs, the costs are subtracted from my company and both the chair and the costs are marked as *consumed* and disappear from the system.

The above is a contrived example that ignores things like shipping, amortized costs of machinery, costs of property usage, etc. However, we can see how resource and labor costs accumulate, divide, and flow within and between companies.

#### Flows of costs

While this has been touched on already, it's important to note that costs can be created (via labor and resources) but that costs can only ever be destroyed by three processes: consumption, taxation, and market sales. Consumption means a consumer covers the cost personally, using credits they have earned via labor. Taxation is the process of spreading a company's costs equally among its members over time such that they use their personal credits to pay the cost. Lastly is market sales, where a company sells a product into the market system at which point the costs are wiped out of the system.

Within the productive network, costs are accumulated, divided, and transferred but are never destroyed.

### Raw material tracking and costing

By default, all products in Basis are categorized the same way: as a product! A hunk of coal or a barrel of oil is modeled the same way in the system as a pack of socks. So how do we tag certain products as tracked resources?

It makes sense that we would want to track raw materials (oil, silicon, iron) but it also might make sense to track resources that are taken one or two steps further: diesel fuel, steel, various chemicals. How it's decided to tag a product as a tracked resource is a matter of systemic governance.

It will also be the case that not just the resource type will be tagged, but also its source. Is an old-growth forest being decimated to make gift shop trinkets? Maybe producers or consumers would want to know this information. This helps us not just with individual knowledge, but also systemic knowledge: where are our resources coming from and at what rates? Having exact data on this could help us achieve much better ecological equilibrium.

Once we have our list of tagged resources, how exactly do we cost them? This is another issue of systemic governance. The process behind this is still being decided on.

TODO:

- [#17 - Raw material tagging](https://github.com/basisproject/tracker/issues/17)
- [#58 - Resource costing](https://github.com/basisproject/tracker/issues/58)
- [#90 - Raw material transformations](https://github.com/basisproject/tracker/issues/90)

### Pricing

So far we've talked a lot about costs. Does this mean that every product purchased by a consumer will be provided at-cost? No, and there two facets to this.

First, for members, the system will *optimize for* providing products at-cost. In other words, companies that do sell products for exactly their cost will be rewarded (more on this in the [cybernetics section](#cybernetics)). That said, companies can set whatever prices above or below cost they want. The difference from a capitalist market however is that when a company sells a widget for 10₡ that only cost 8₡ to produce, it doesn't realize the extra 2₡: the credits are still burned on purchase.

Secondly, for non-members, companies are incentivized to sell at the highest price they can. So each product that is available for public consumption will have at least two prices: the in-network price and the market price.

So why let companies set arbitrary prices at all? It enables things like clearing inventory for products that didn't sell well, and also it enables inventory control for higher-value items. Member companies don't realize the delta between cost and price as a gain or loss, but rather it is used as a signal to determine if the production of that particular product should be expanded or contracted. In other words, we retain the *signal mechanism* from pricing *without using it as a distribution mechanism*.

This system of pricing is described in "Towards a New Socialism" (Cockshott & Cottrell, 1993).

### Cybernetics

Cybernetics is a system of self-regulating and automated control. We employ cybernetics to incentivize companies to act in the best interests of the network. This is done by adjusting a company's cost ceiling: the more the company acts in the interests of the network, the higher its cost ceiling. In effect, a high cost ceiling translates to a higher social investment.

TODO:

- [Cybernetics metrics and signals](https://github.com/basisproject/tracker/issues?q=is%3Aissue+is%3Aopen+label%3Atag%3Acybernetics)

### Public market

The public market is where members advertise and consume each other's products and services. It is also the medium by which orders are processed.

Non-member companies and users are free to use the public market for buying or selling, but may have to pay a monthly service fee and/or per-transaction fee. Thus, the public market is not just a system for discovery and economic processing inside the network, but also a vector for growth of network capital and resources.

It should be noted that the definitions, processes, and interfaces of the public market are not going to be defined yet because the need for it is somewhat far off. There needs to be a critical mass of members and products in the system before the market even makes sense to define and plan.

[resources]: #chapter-3-resources

---

#### Effects and goals of UBI

Defining a currency that everyone gets regularly that cannot be transferred and can only be spent on internally-produced products and services has some great benefits for both participants and the network as a whole. If the UBI is high enough to be a living wage, what this does is effectively remove the cost of survival from the productive system. In effect, we make it easy to quit a job. This is seemingly banal but the effect is incredible: this would lower externalities in production, lower the labor costs of products significantly, and ultimately make the productive system much more adaptable.

If nobody needs a job to survive, then there's no systemic pressure to *win at all costs*, which would effectively not only eliminate entire classes of externalities but also reduce the pressure of competition within the system. No longer do companies have to balance survival with being community members, because survival doesn't need to be covered by the productive system. Companies are free to expand and contract as members see fit without needing to worry about the welfare of those who might come and go. Industries that find themselves teetering on obsolescence (for instance the thermal coal industry) could be phased out more easily and quickly if the participants in that industry no longer relied on their jobs to put food on the table. What were once bureaucracies can now be [adhocracies][adhoc]. The productive system is free to morph and shift to the will of the participants without fear of leaving people behind, and without massive and expensive redistribution programs.

If everyone gets a living wage just for being a participant, it's quite possible that people might not even feel compelled to take a wage for many jobs. This would mean the costs of products and services is no longer the cost of labor and resources, but just resources. In effect, profitless production in combination with a living wage UBI *could make self-organized, resource-based communism a reality*.

There's one more important point to make. There are entire classes of people who do productive labor every day and are not compensated for it. This includes homemakers, stay at home parents, community service volunteers, artists, etc. Don't these people deserve compensation for their contributions to society regardless of how markets value their contributions?

[blasphlam]: https://dolt.com
