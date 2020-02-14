# Chapter 4: Cost tracking

Cost tracking is a very important concept in a socialist economy, especially absent the exchange of money between producing entities. Basis tracks costs on a per-company level over a certain period of time, and gives companies the freedom to assign their costs to their products proportionally. This is based on the inputs and outputs of the company, and costs are recalculated whenever any action in the company changes costs (a worker clocking out, a new order coming in, etc).

Breaking things down a bit, companies have inputs and outputs. To put it simply in current terms: inputs are labor and purchases, outputs are sales. How do we measure these? Do we force companies to report all their data to a central agency? No! We use their labor tracking and their incoming and outgoing order list. Because labor and orders all go through Basis, we can track all the costs of production, per-company, given their inputs and outputs.

It's important to note that for cost tracking to be effective, a critical mass of production needs to be using Basis: the more the economic network participates, the more accurate the costs will be.

## Labor costs

All things take labor to make or use. Apples must be picked. Iron must be mined. Chairs must be crafted.

Basis tracks labor costs of companies. So if a worker clocks in, fills an order for 5 chairs in 8 hours, then clocks out, Basis knows that 5 chairs were created in 8 hours. This labor cost is then reflected in the costs of the chairs the company provides. However, not just the labor of the chair makers is accounted for, but also the labor cost of the workers who cut the trees for the lumber, and the workers who milled the lumber, and the delivery truck driver, etc.

## Resource costs and resource tags

By default, Basis only tracks costs in terms of labor. However, it has the ability to track resources using the concept of "resource tags" which are attached to a particular product produced by a company. Resource tags tell the costing system to track that product not just as labor, but also as a resource.

For instance, one might assign a resource tag to the iron that comes out of an iron mine. Then when the chair maker orders iron screws for the chairs she makes, the labor cost of the screws is added to the cost of the chairs, but so is the resource cost of the iron.

If resource tags are added to various types of fossil fuels and scarce resources, then we can get an accurate picture of not just the labor it takes to make something, but also the resource usage, and to some extent the externalities.

## Cost algorithm

The general algorithm for costing is as follows:

```
(costs of purchases + costs of labor) / outputs
```

There must however be some nuance here, because what if one product is heavily inventory-based and thus the cost of inventory should be more directly assigned to that product? This is where companies can use cost tags. Cost tags create "buckets" of different types of costs that can then be assigned to products and services proportionally.

Let's follow an example. 

Our company makes chairs. One is a basic chair and one is a more advanced chair. If built in equal quantities, the basic chair uses 20% of the total inventory cost, while the advanced chair uses the other 80%.

On top of this, we have operational costs, such as the labor of our workers and the cost of the electricity to run our factory.

So we create two cost tags: "Operational" and "Inventory" which will hold our costs. When workers clock in, the labor costs are assigned to the "Operational" bucket, and when we make purchase orders to buy wood for our chairs, those orders will fall under the "Inventory" bucket. It's important to note that while we are using one tag for each cost here, multiple tags can be assigned in different proportions. More on this later.

If we get an order for 10 basic chairs and 5 advanced chairs, we order the lumber we need. The order will take two workers 6 hours to complete, and the lumber costs 25 labor hours.

So once the chairs are built, we have two cost buckets:

- Operational
  - 12 labor hours (two workers * 6 hours)
- Inventory 
  - 25 labor hours

How do we assign our costs accordingly to our products? Since the basic chair uses 20% of inventory and the advanced chair uses 80%, we assign these cost tags to the products accordingly, and because both chairs are the same cost operationally, we assign an equal value to each for operational costs:

- Basic
  - Operational: 1
  - Inventory: 2
- Advanced
  - Operational: 1
  - Inventory: 8

Now it's time to determine our costs:

- Basic:
  - `((OperatingCost * (1 / (1 + 1))) + (InventoryCost * (2 / (2 + 8)))) / NumOrdered`
  - `((12 * (1 / (1 + 1))) + (25 * (2 / (2 + 8)))) / 10`
  - `1.1 labor hours each`
- Advanced:
  - `((OperatingCost * (1 / (1 + 1))) + (InventoryCost * (8 / (2 + 8)))) / NumOrdered`
  - `((12 * (1 / (1 + 1))) + (25 * (8 / (2 + 8)))) / 5`
  - `5.2 labor hours each`

It's important to note that because the costs are a function of `costs over time / outputs over time` that costs will fluctuate more in the beginning of a company's operation and smooth our over time. This can be mitigated to some extent by using amortization pools.

Also note that when the company made the lumber order, if they were going to use 5% of that lumber for an internal project, then they can assign two cost tags to the order: `Operating = 5` and `Inventory = 95` which would assign 5% of the order to the Operating costs bucket and would change the resulting prices. In other words, orders can have any number of cost tags assigned to them, which assign that order's costs to the tags proportionally.

## Time frame

Costs are tracked over time, and end up being average costs of production over a certain time frame. How long of a time frame? It likely depends on the company's production cycle and would likely be managed by the company, although this might also be a case where the bank sets a regional default (for instance a one year time frame) and companies can petition to have this changed if needed.

## Amortization pools

A company that makes chairs might need to invest up front in a chair-making machine. If this machine costs 100 labor hours, then the first order the company gets will be astronomically costly, and the first order likely won't ever come.

It's important that companies be able to spread startup costs, the cost of large purchases, or lulls in production over a long period of time. This is where amortization pools come in.

A company can open an amortization pool, which has a spending limit and a term (end date). Once opened, the pool will add to the company's costs *hourly* as such:

```
spending limit / hours in term
```

Now the company can use cost tags to assign labor costs and orders into the amortization pool, and those costs will not be counted in the company's direct costs (but rather will be accounted for by the amortization pool's hourly cost).

If a company spends more into the amortization pool than the spending limit, the extra costs will automatically be added to their direct costs. If an amortization pool term completes and the company has not reached the spending limit, costs can still be assigned to the pool until the spending limit is reached.

Because amortization pools allow spreading costs over a long period of time (much like a bank loan), it's important to note that various regions might have different rules on the circumstances these pools can be opened. For instance, there might be limits on the spending limits, term limits, number of open pools, etc.






