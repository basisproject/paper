# Chapter 4: Economics

Basis defines a system of economics with the end goal of eliminating production for profit and for compensating members based on their contribution.

It's worth mentioning that Basis uses the [ValueFlows vocabulary](https://valueflo.ws) for much of its economic system, which provides a general set of operations that can describe and model an incredibly diverse set of economic scenarios. It should be noted that while Basis does use ValueFlows for whatever it can, it also layers some other things on top of it (like disaggregate cost tracking).

## Cost tracking

Cost tracking is a very important concept in a socialist economy, especially absent the exchange of money between producing entities. Basis tracks costs on a per-company level based on that company's incoming and outgoing orders (what would be "sales" and "purchases" in capitalism). Incoming and outgoing orders are *known* because they either go through the [public market](#public-market-1) or through the [banking system](#chapter-5-banking), and thus we can easily determine a company's total costs.

Each company has a set of costs and cost ceiling. The system uses the cost ceiling to determine how many costs it can take on before it needs to transfer to another company (via orders). The cost ceiling ensures that costs keep flowing and do not accumulate or stagnate too much. For instance, if a company's cost ceiling is 1000₡ and their current total costs are 990₡, they might want to seriously consider trying to get some other companies or consumers to order their products, otherwise they won't be able to pay themselves or order more inputs to production.

It's worth mentioning that cost tracking is more effective as the network grows: more participants transacting in-network means more accurate cost tracking. Remember, market prices obscure costs, so every time a company has to order out-of-network that's a cost we can't disaggregate, and every time a member company sells into the market, those costs are destroyed. The more participants, the more accurate, and ultimately useful, cost tracking will be.

### Assigning costs to products and services

How costs are assigned to products and services by each company is completely up to them. They might want to automate this based on inputs and outputs over time. They might want to carefully assign costs manually for each product. They are free to handle this however they want, provided that their total costs are kept under their cost ceiling.

### Labor costs

All things take labor to make or use. Apples must be picked. Iron must be mined. Chairs must be crafted.

Basis tracks labor in two ways. First, is occupation-wages, and secondly is occupation-hours. Occupation-wages track the credit values paid out to workers, and occupation-hours track labor in hours that occurred. This means if I work at the widget factory and I make 10₡/hr, then if I work 7.5 hour day, the occupation-wages would be tracked as  `{"factory-worker": 75}` and the occupation-hours would be tracked as `{"factory-worker": 7.5}`.

From the above, we see that we don't track just labor wages/hours in total, but rather bucket them by occupation. This gives us more disaggregate information that would otherwise be wiped out by a price: how much were the factory workers paid? How much were CEOs paid? How much was spent in marketing? Knowing these things about a product give us valuable insights into how it was produced.

Which occupations we track is a matter of systemic governance.

TODO:

- [#89 - Governance of occupation data](https://github.com/basisproject/tracker/issues/89)

### (Semi) raw material costs

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

This is a contrived example that ignores things like shipping, amortized costs of machinery, costs of property usage, etc. However, we can see how resource and labor costs accumulate, divide, and flow within and between companies.

### Currency tracking

When a company needs to buy something from capitalist markets, it spends currency out of a [capital pool](#capital-pools) to do so. The cost of this currency, like all other costs, is tracked by Basis. This is covered in more detail in [the banking section](#currency-tracking-1).

### Flows of costs

While this has been touched on already, it's important to note that costs can be created (via labor and resources) but that costs can only ever be destroyed by three processes: consumption, taxation, and market sales. Consumption means a consumer covers the cost personally, using credits they have earned via labor. Taxation is the process of spreading a company's costs equally among its members over time such that they use their credits to pay the cost. Lastly is market sales, where a company sells a product into the market system at which point the costs are wiped out of the system.

Within the productive network, costs are accumulated, divided, and transferred but are never destroyed.

## Raw material tracking and costing

By default, all products in Basis are categorized the same way: as a product! A hunk of coal or a barrel of oil is modeled the same way in the system as a pack of socks. So how do we tag certain products as tracked resources?

It makes sense that we would want to track raw materials (oil, silicon, iron) but it also might make sense to track resources that are taken one or two steps further: diesel fuel, steel, various chemicals. How it's decided to tag a product as a tracked resource is a matter of systemic governance.

It will also be the case that not just the resource type will be tagged, but also its source. Is an old-growth forest being decimated to make gift shop trinkets? Maybe producers or consumers would want to know this information. This helps us not just with individual knowledge, but also systemic knowledge: where are our resources coming from and at what rates? Having exact data on this could help us achieve much better ecological equilibrium.

Once we have our list of tagged resources, how exactly do we cost them? This is another issue of systemic governance. The process behind this is still being decided on.

TODO:

- [#17 - Raw material tagging](https://github.com/basisproject/tracker/issues/17)
- [#58 - Resource costing](https://github.com/basisproject/tracker/issues/58)
- [#90 - Raw material transformations](https://github.com/basisproject/tracker/issues/90)

## Pricing

So far we've talked a lot about costs. Does this mean that every product purchased by a consumer will be provided at-cost? No, and there two facets to this.

First, for members, the system will *optimize for* providing products at-cost. In other words, companies that do sell products for exactly their cost will be rewarded (more on this in the [cybernetics section](#cybernetics). That said, companies can set whatever prices above or below cost they want. The difference from a capitalist market however is that when a company sells a widget for 10₡ that only cost 8₡ to produce, it doesn't realize the extra 2₡: the credits are still burned on purchase.

Secondly, for non-members, companies are incentivized to sell at the highest price they can. So each product that is available for public consumption will have at least two prices: the in-network price and the market price.

So why let companies set arbitrary prices at all? It enables things like clearing inventory for products that didn't sell well, and also it enables inventory control for higher-value items. Member companies don't realize the delta between cost and price as a gain or loss, but rather they are used as signals to determine if the production of that particular product should be expanded or contracted. In other words, we retain the *signal mechanism* from pricing *without using it as a distribution mechanism*.

This system of pricing is described in "Towards a New Socialism" (Cockshott & Cottrell, 1993).

## Cybernetics

Cybernetics is a system of self-regulating and automated control. We employ cybernetics to incentivize companies to act in the best interests of the network. This is done by adjusting a company's cost ceiling: the more the company acts in the interests of the network, the higher its cost ceiling. In effect, a high cost ceiling translates to a higher social investment.

TODO:

- [Cybernetics metrics and signals](https://github.com/basisproject/tracker/issues?q=is%3Aissue+is%3Aopen+label%3Atag%3Acybernetics)

## Public market

The public market is where members (and consumers and producers) find each other's products and services. It is also the medium by which orders are processed.

Non-member companies and users are free to use the public market for buying or selling, but may have to pay a monthly service fee and/or per-transaction fee. Thus, the public market is not just a system for discovery and economic processing inside the network, but also a vector for growth of network capital and resources.

It should be noted that the definitions, processes, and interfaces of the public market are not going to be defined yet because the need for it is somewhat far off. There needs to be a critical mass of members and products in the system before the market even makes sense to define and plan.

### Anonymity

While the transactions between member companies are transparent and freely observable by any member of the network, there are a few cases where privacy is offered:

- Consumer purchases. Whether a member or a non-member, purchasing goods from a member company is anonymous. The order is still be viewable by members, but looks like it came from the Basis system directly and is not tied back to the user in any way. Only the user and the company have the full information on the order.
- Non-member orders. Any time a non-member is involved in an order, the order details will be available, but the non-member company will be anonymized. This is to protect the privacy of companies who do not wish for their purchasing to be transparent. Like consumer purchases, the order will look like it came from the Basis system directly, and the non-member company's identity will be anonymized.

TODO:

- [#4 - Privacy for consumer orders](https://github.com/basisproject/tracker/issues/4)

