## Chapter 4: Blocs

A bloc is a grouping of [agents] and [resources]. The agents that are part of a bloc are known as "members" and the relationship between a bloc and a resource is known as stewardship.

All blocs are cooperative entities controlled by their members. Leadership (if any) must ultimately be selected by individual agents, and all policies regarding membership of blocs must be ratified by the members. As such, Basis defines a bloc as a bottom-up power structure that puts individual agents in ultimate control of the groups they are members of.

Blocs and agents are freely-associating entities: any agent can join any bloc as long as the entities involved consent to the association.

Blocs are free to use whatever organization structure they see fit. There is not any predetermined pattern or framework blocs must follow, with the caveat that the members in a bloc have ultimate power regrading the bloc. Blocs can use a traditional top-down structure where members select a board/council that determines overall direction and leadership, or blocs can be run as a collective where everyone is involved in making every decision. Blocs can have vesting schedules for new members. They can be set up as multi-stakeholder entities (for instance, shared control between workers, customers, vendors, and/or the public).

Membership in a bloc is completely decided by the bloc itself. Membership might be determined by geographical location, interest or skill in a particular sector of production, and can even depend on things like compulsory ordering of a service every month (which could be structured as a membership fee or even a tax in the case of a municipal bloc). Some blocs might require membership in sister blocs: perhaps you cannot be a member of the Duluth Municipal Housing bloc if you are not also a member of the Duluth Municipal bloc.

### Bloc costs

Every bloc has a running total of costs it has incurred, generally through ordering inventory or by paying wages. How exactly costs are modeled within the protocol is covered later in the [costs] section of the paper.

Costs are held in two different parts of blocs: [resources][resource-costs] that the bloc is in stewardship of and *processes*.

A process is an activity that transforms economic inputs to outputs. In Basis, processes extend the concept of [processes in ValueFlows](https://valueflo.ws/introduction/processes.html) by adding a set of [costs] associated with that process. A bloc can have as many processes as it wants, each with a set of associated costs.

Resources and processes are where [flows of value occur](https://valueflo.ws/introduction/concepts.html): a bloc might order lumber (a resource), consume the lumber and transform it using labor and machinery (in a process) to make chairs (a new resource). In this example, the lumber has a cost which is assumed by the bloc when it is ordered. When the lumber is used as an input into the "make chairs" process, the process takes on the cost of the lumber. When labor is performed to actually make the chairs, the cost of that labor also adds to the costs of the process, as does the amortized per-use cost of the machinery that was used to make the chairs. In effect, processes *combine* the costs of their inputs. Once the chairs are complete, the bloc might assign the costs of the process to the resulting chairs evenly: if the process assumed `X` costs, and the result was six chairs, then each chair might have a cost of `X / 6`. This is just an example though: blocs are free to move costs between internal processes and resources as they see fit.

The total costs of a bloc can be determined by taking the costs of all the bloc's resources and all the bloc's processes and [summing them together][cost-math]. This gives a final cost object, which can be [converted into a final number][cost-conversion].

### Cost allowance

Every bloc has a cost allowance. This is a variable number that determines the upper limit on costs that the bloc can assume. If the bloc's total costs (the sum of all the bloc's resources and processes) reaches this number, the protocol prevents the bloc from taking on any new costs, be it ordering inventory, paying wages, or any other ways that blocs can take on new costs. The cost allowance can be thought of as the total amount of societal debt that a bloc may take on.

Once a bloc's total costs reaches its cost allowance, a bloc must find a way to unload its costs. This would happen via other blocs [ordering products or services][orders] or through direct member taxation.

The actual cost allowance value is determined by the [investment] and [cybernetics] systems defined by the protocol.

#### Allowance exceptions

TODO:

- [Cost allowance overage](https://github.com/basisproject/tracker/issues/135)

### Wages

Agents can perform labor for the blocs they are members of and if they track this labor within the protocol then they will receive wages in the form of [credits]. The exact arrangement of how the wage is arranged is between the member and the bloc: it could be hourly, it could be salary, it could be project or commission based, some combination of all three, or something entirely different. The protocol does not confine any form of arrangement, but it does allow the bloc or the member to register the labor and provides methods for compensating it with credits.

When the credits are paid (not when the labor is tracked), the cost of the labor is added to the bloc's costs in the same transaction. In other words, an agent receiving credits is always matched exactly by a bloc assuming new costs.

[agents]: #chapter-2-agents
[resources]: #chapter-3-resources
[costs]: #costs
[resource-costs]: #resource-costs
[cost-math]: #mathematical-operations-on-costs
[cost-conversion]: #converting-costs-to-a-single-value
[trackers]: #BROKEN-trackers
[part3]: #part-3-the-real-world
[cost allowance]: #cost-allowance
[orders]: #BROKEN-orders
[investment]: #BROKEN-investment
[cybernetics]: #BROKEN-cybernetics
[credits]: #labor-credits
[economics]: #chapter-6-economics

