## Chapter 6: Cost and resource flows

Now that we know what costs are and the information they track, let's look at how costs (and resources) are tracked and how they move through the system.

### Bloc cost tracking

Costs are tracked by blocs in two main ways: costs are created when workers perform labor and costs are assumed from other blocs when ordering products/inventory. Looking at this from a high level, a bloc orders inventory, applies labor to it, and creates a product. The resulting cost of the product would be `cost_inventory + cost_labor` (see the section on [cost addition][cost-addition]).

So if a woodworking bloc with one worker orders 50kg of lumber with a cost of

```
{
    labor_hours: {
        miller: 0.5
        trucker: 0.1
    }
    labor_wages: {
        miller: 15
        trucker: 2.5
    }
    resources: {
        lumber: 50
        diesel: 0.5
    }
    processes: {
        refine: {oil: 5}
    }
```

and then applies 10 hours of labor to make five chairs, we get the total costs:

```
{
    labor_hours: {
        miller: 0.5
        trucker: 0.1
        woodworker: 10
    }
    labor_wages: {
        miller: 15
        trucker: 2.5
        woodworker: 200
    }
    resources: {
        lumber: 50
        diesel: 0.5
    }
    processes: {
        refine: {oil: 5}
    }
```

If the woodworker wishes to assign each of the five chairs the same cost, then we take the above cost and divide by `5`:

```
{
    labor_hours: {
        miller: 0.1
        trucker: 0.02
        woodworker: 2
    }
    labor_wages: {
        miller: 3
        trucker: 0.5
        woodworker: 40
    }
    resources: {
        lumber: 10
        diesel: 0.1
    }
    processes: {
        refine: {oil: 1}
    }
```

This is the cost of each chair. Now, if another bloc needs a chair, when they order one from our woodworker, the cost of that chair is removed from the woodworker's costs and assigned to the ordering bloc's cost because the *cost is associated with the chair resource* and as the chair flows through the system, so does the cost.

This is an example of how costs are created and assigned to resources. Blocs are free to manage costs however they see fit with a few constraints enforced by the protocol:

1. Costs out must equal costs in. Blocs cannot remove, reduce, or otherwise modify the amount of costs (other than adding to them).
2. A bloc must not exceed its [cost allowance][cost-allowance].

#### Bloc processes

At a lower level, a bloc can use *processes* as a way of breaking down flows of resources and labor internally. A process is an activity that transforms economic inputs to outputs. In Basis, processes extend the concept of [processes in ValueFlows][ext-vf-processes] by adding a set of [costs] associated with that process. A bloc can have as many processes as it wants, each with a set of associated costs.

In the above example, our chair maker might have a process for `unload lumber shipment`, which yields usable lumber which can be used in the `make chairs` process.

It's important to note that as resources have costs, so do processes. For instance, `make chairs` might first have the cost of the lumber added to it, and then the labor of the chair maker. Once the chairs are created, the total cost of the `make chairs` process might be assigned equally to the chairs.

#### Labor and Wages

When an agent performs labor for a bloc they are a member of and if they track this labor within the protocol then they will receive wages in the form of [credits][labor-credits]. The exact arrangement of how the wage is arranged is between the member and the bloc: it could be hourly, it could be salary, it could be project or commission based, some combination of all three, or something entirely different. The protocol does not enforce any form of arrangement, but it does allow the bloc or the member to register the labor and provides methods for compensating it with credits.

This is how `labor_hours` and `labor_wages` are created and tracked in the protocol: agents get paid for their work by creating labor transactions.

For details on when labor transactions are created versus when credits are paid, see the [section on cost staging][cost-staging].

### Orders

So far we've alluded to things like "a bloc orders inventory" but we haven't talked much about orders or how they work. Basis defines a transactional fabric to facilitate blocs ordering products from each other. An order is an expression of intent from one bloc to another that communicates that a resource or service is needed (and the ordering bloc is willing to take on a specific cost in exchange for that resource/service).

So if one bloc needs lumber to make their widgets, they might find another bloc that produces lumber and they create an order in Basis for what they need. If the lumber bloc approves, then the order is finalized and the lumber (and the costs associated with them) are transferred to the widget bloc. This is a simplistic example and ignores things like logistics, but the general idea is that Basis facilitates the expression of intent and movement of costs between blocs.

Orders are the drivers of flows of value through the system. Orders move resources and costs between blocs and facilitate production.

Orders in Basis are a direct extension of [Agreements][ext-vf-agreement] and [Intents][ext-vf-intent] in ValueFlows.

### Transparency

All orders, processes, resources, and transactions that affect all of these things are completely transparent to any agent of the network. 

This means that the order book for any given bloc can be viewed by any other company. While this might seem radical to the casual reader, the reasoning behind it is that if demand can be sensed directly (via knowing the incoming orders for any given products), profit becomes vestigial. One primary goal of Basis is to eliminate the profit mechanism in production, and what better way than to allow for reading demand directly?

