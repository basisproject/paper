## Chapter 3: Blocs

A bloc is a recursive grouping of [members](#chapter-2-members), [resources](#chapter-4-resources), and other blocs. The members and blocs that are part of a block are known as "associates."

All blocs are controlled by their constituent *members* (not *associates* mind you but individual protocol members). Leadership (if any) must be selected by individual members, and all policies regarding membership of blocs must be ratified by individual members. As such, Basis defines a bloc as a bottom-up power structure that puts individual members in control of the things that affect them the most.

Blocs and members are freely-associating entities: any member or bloc can join any bloc as long as the entities involved consent to the association.

Blocs are free to use whatever organization structure they see fit. There is not any predetermined pattern or framework blocs must follow, with the caveat that the members in a bloc have ultimate power over the bloc. Blocs can use a traditional top-down structure where associates select board members who determine overall direction and leadership, or blocs can be run as a collective where everyone is involved in making every decision. Blocs can have vesting schedules for new associates. They can be set up as multi-stakeholder entities (for instance, shared control between workers, customers, vendors, and/or the public).

### Voting and governance

### Costs

Every bloc has a running total of costs it has incurred, generally through ordering inventory or by paying wages. Costs in Basis are not a number, but rather separate numbers of individual costs bundled together.

Costs are bucketed by a few different types: `labor`, `labor_hours`, and `resources`.

- `labor` tracks how many wages were paid to a particular occupation in a set of costs. Occupations are tracked globally in the protocol and are standard across all blocs.
- `labor_hours` tracks how many hours were worked by a particular occupation in a set of costs. Occupations here are the same as in the `labor` object (standard and tracked globally).
- `resources` holds the quantity of the standard unit of [tracked resources](#tracked-resources) that exist in a set of costs.

These types can be extended to track other cost types (for instance `currency`, which will be [covered in part 3](#part-3-the-real-world)).

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

### Accounting and REA

Basis extends the [ValueFlows](https://valueflo.ws/) protocol for its accounting within and between blocs. ValueFlows is an REA (Resource, Agent, Event) accounting system that allows tracking detailed flows of resources through an economy.

### Resource stewardship

### Wages

### Orders

### Investment

