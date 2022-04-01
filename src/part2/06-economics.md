## Chapter 6: Costs

Cost tracking is the core of Basis; its primary function. How costs are interpreted or acted upon is important, but secondary to the actual gathering of costing information.

When people think of a cost, they often think of a number, and usually a price. Basis expands this concept of cost into something with much more dimensionality than a number. It tries to answer the question "what did it cost to make something?" How much labor? How much raw materials? What transformational processes did resources/raw materials go through to become the final product?

## Chapter 6: Bloc economics


### Costs


### Intra-bloc economics

### Inter-bloc economics






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

Note that this is known as in-kind [^cik] cost tracking.

#### Memorialization

- TODO: describe how costs memorialize resource prices

#### Mathematical operations on costs

- TODO: give examples of addition and division

#### Converting costs to a single value

Although costs are tracked as separate collections of distinct values, it's important that they are able to be summed into one aggregate number: a total cost. This allows blocs to get a quick sense for their overall cost amounts, and as we'll see later, something called a [cost allowance].

Deriving this total value is simple in the case of `labor` and `labor_hours`, we simply sum the values together. For instance, in the object above, we would have `12.89 + 3.38 + 41.45 + 0.2578 + 0.0965 + 1.0363` or `59.1106`.

The tricky bit comes when we convert our resource values into a cost, and this is where [trackers] come in: they allow categorizing a [resource][resources] within the cost tracking system such that a standard unit can be assigned a cost value. Thus, if in our example above, `iron` is tracked in grams, and the cost-per-gram is `0.3` then the iron cost would be `8.5 * 0.3` or `2.55`. If done for all `resources` in the costs set, we can add this to our labor costs and find our total aggregate cost value.

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

This system of pricing is described in "Towards a New Socialism" [^tns].

[resources]: #chapter-3-resources

