## Chapter 4: Blocs

A bloc is a grouping of [agents] and [resources]. The agents that are part of a bloc are known as "members."

All blocs are cooperative entities controlled by their members. Leadership (if any) must ultimately be selected by individual agents, and all policies regarding membership of blocs must be ratified by the members. As such, Basis defines a bloc as a bottom-up power structure that puts individual agents in ultimate control of the groups they are members of.

Blocs and agents are freely-associating entities: any agent can join any bloc as long as the entities involved consent to the association.

Blocs are free to use whatever organization structure they see fit. There is not any predetermined pattern or framework blocs must follow, with the caveat that the members in a bloc have ultimate power regrading the bloc. Blocs can use a traditional top-down structure where members select a board/council that determines overall direction and leadership, or blocs can be run as a collective where everyone is involved in making every decision. Blocs can have vesting schedules for new members. They can be set up as multi-stakeholder entities (for instance, shared control between workers, customers, vendors, and/or the public).

Membership in a bloc is completely decided by the bloc itself. Membership might be determined by geographical location, interest or skill in a particular sector of production, and can even depend on things like compulsory ordering of a service every month (which could be structured as a membership fee or even a tax in the case of a municipal bloc). Some blocs might require membership in sister blocs: perhaps you cannot be a member of the Duluth Municipal Housing bloc if you are not also a member of the Duluth Municipal bloc.

### Costs

Every bloc has a running total of costs it has incurred, generally through ordering inventory or by paying wages. Costs in Basis are not a number, but rather separate collections of individual costs bundled together.

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

#### Breaking down costs

Blocs in Basis make extensive use of the ValueFlows protocol for their accounting. In ValueFlows, there are two concepts that are related to costs within blocs: resources and processes. As described in the [chapter on resources][resource-costs], a resource can have a cost attached to it.

Processes

#### Summing costs

Although costs are tracked as separate collections of distinct values, it's important that they are able to be summed into one aggregate number: a total cost. This allows blocs to get a quick sense for their overall cost amounts, and as we'll see later, something called a [cost allowance].

Deriving this total value is simple in the case of `labor` and `labor_hours`, we simply sum the values together. For instance, in the object above, we would have `12.89 + 3.38 + 41.45 + 0.2578 + 0.0965 + 1.0363` or `59.1106`.

The tricky bit comes when we convert our resource values into a cost, and this is where [trackers] come in: they allow categorizing a [resource][resources] within the cost tracking system such that a standard unit can be assigned a cost value. Thus, if in our example above, `iron` is tracked in grams, and the cost-per-gram is `0.3` then the iron cost would be `8.5 * 0.3` or `2.55`. If done for all `resources` in the costs set, we can add this to our labor costs and find our total aggregate cost value.

### Cost allowance

Every bloc has a cost allowance. This is a number that determines the upper limit on costs that the bloc can assume. If the bloc's costs go above this number, the protocol prevents the bloc from taking on any new costs, be it ordering inventory, paying wages, or any other ways that blocs can take on new costs. The cost allowance can be thought of as the total amount of societal debt that a bloc may take on.

The actual cost allowance value is determined by the [investment] and [cybernetics] systems defined by the protocol.

#### Allowance exceptions

TODO:

- [Cost allowance overage](https://github.com/basisproject/tracker/issues/135)

### Wages

Agents can perform labor for the blocs they are members of and if they track this labor within the protocol then they will receive wages in the form of [credits]. The exact arrangement of how the wage is arranged is between the member and the bloc: it could be hourly, it could be salary, it could be project or commission based. The protocol does not confine any form of arrangement, but it does allow the bloc or the member to register the labor and provides methods for compensating it with credits.

When the credits are paid (not when the labor is tracked), the cost of the labor is added to the bloc's costs in the same transaction. In other words, an agent receiving credits is always matched exactly by a bloc assuming new costs.

[agents]: #chapter-2-agents
[resources]: #chapter-3-resources
[resource-costs]: #resource-costs
[trackers]: #BROKEN-trackers
[part3]: #part-3-the-real-world
[cost allowance]: #cost-allowance
[investment]: #BROKEN-investment
[cybernetics]: #BROKEN-cybernetics
[credits]: #labor-credits
[economics]: #chapter-6-economics
