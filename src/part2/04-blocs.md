## Chapter 4: Blocs

A bloc is a grouping of [agents] and [resources]. The agents that are part of a bloc are known as "members" and the relationship between a bloc and a resource is known as [stewardship].

All blocs are cooperative entities controlled by their members. Leadership (if any) must ultimately be selected by individual agents, and all policies regarding membership of blocs must be ratified by the members. As such, Basis defines a bloc as a bottom-up power structure that puts individual agents in ultimate control of the groups they are members of.

Blocs and agents are freely-associating entities: any agent can join any bloc as long as the entities involved consent to the association.

Blocs are free to use whatever organization structure they see fit. There is not any predetermined pattern or framework blocs must follow, with the caveat that the members in a bloc have ultimate power regarding the bloc. Blocs can use a traditional top-down structure where members select a board/council that determines overall direction and leadership, or blocs can be run as a collective where everyone is involved in making every decision. Blocs can have vesting schedules for new members. They can be set up as multi-stakeholder entities (for instance, shared control between workers, customers, vendors, and/or the public).

Membership in a bloc is completely decided by the bloc itself. Membership might be determined by geographical location, interest or skill in a particular sector of production, and can even depend on things like compulsory ordering of a service every month (which could be structured as a membership fee or even a tax in the case of a municipal bloc). Some blocs might require membership in sister blocs: perhaps you cannot be a member of the Duluth Housing bloc if you are not also a member of the Duluth Municipal bloc.

### Bloc structure

Blocs are necessarily controlled by their members. How this control is shaped is determined by the bloc itself, however this structure must be formally defined via Basis' sister protocol, [Stamp][stamp-protocol]. Stamp provides a cryptographic system for defining group control of an entity.

In essence, each bloc maintains a group identity defined by Stamp, and that identity is used to formalize membership and also control of the bloc. In other words, agents become members of a bloc by the bloc blessing them with some form of cryptographic control of the bloc.

Stamp also allows group members to *act on behalf of the bloc* in ways predetermined by the bloc itself, making it so responsibilities and powers can be delegated to an individual agent.

The relationships and powers Stamp can express in the context of group control of an entity are effectively endless.

### Bloc voting

TODO:

- [#60 - Voting](https://github.com/basisproject/tracker/issues/60)

### Bloc allocation

Every bloc has an allocation. This is a variable number that determines the upper limit on costs that the bloc can assume. If the bloc's total costs (the sum of all the bloc's pending wage costs, resources, and processes) reaches this number, the protocol prevents the bloc from taking on any new costs, be it ordering inventory, paying wages, or any other ways that blocs can take on new costs. The allocation can be thought of as the total amount of societal debt that a bloc may take on.

The total costs of a bloc can be determined by taking the costs of all the bloc's labor costs, resources, and all the bloc's processes and [summing them together][cost-addition]. This gives a final cost object, which can be [converted into a final number][cost-conversion].

Once a bloc's total costs reaches its allocation, a bloc must find a way to unload its costs. This would happen via other blocs or consumers [ordering products][orders] or through direct member taxation.

The actual allocation value is determined by the [investment] and [cybernetics] systems defined by the protocol.

#### Allocation overage

TODO:

- [#135 - Allocation overage](https://github.com/basisproject/tracker/issues/135)

### Cost staging

TODO:

- [#139 - Cost staging](https://github.com/basisproject/tracker/issues/139)

