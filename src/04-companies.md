# Chapter 4: Companies, workers, and wages

Companies are started by region members and operated by their workers. They are autonomous economic units that can set prices for their products and services however they see fit, as long as their income matches the inputs to their production (including regionally-defined wages).

## Profit

Companies can make profit. The difference with market capitalism, however, is that company profit cannot be arbitrarily distributed to workers. Profit must be spent, via the Conductor system, on more inputs to production, hiring more workers, or buying products or services to make production more efficient. Profits which sit too long (using FIFO accounting) in a company's capital pool are taxed at a regionally-defined rate.

This separation of profit from the workers ensures a few things:

- Capital is distributed based on regional wages, not based on arbitrary market forces
- Profit is used mainly as an economic feedback mechanism

Over time, this mechanism redistributes capital based on regionally-decided wages. There is no longer a scenario where an absentee-owner gets rich while the workers get paid poverty wages.

Profit as a feedback mechanism is also important. It's extremely difficult to efficiently allocate resources and match supply with demand using any form of economic planning. On top of questions of planning, you get into situations where perhaps society's needs are met (food, shelter, clothing) but certain wants are completely unmet. You could construct a planned economy based on Marx's use-values, but who determines use-values? Who determines what is important or needed and what isn't?

Profit answers all these questions. It allows economic planning in a purely-decentralized manner, allocating resources to exactly where society desires them. Further, splitting the profit from from capital distribution allows profit to be used as an economic feedback mechanism without distributing wealth in absurdly uneven ways. Because profit can only be used for reinvestment and not personal gain, the endless drive for excess profit is significantly reduced along with the negative side-effects that come with it (such as pollution and regulatory capture).

This method of economics

- foregoes the need for central or detailed planning
- tames profit-driven greed and reduces pressure for negative externalities (such as pollution)
- distributes wealth to workers based on the value and amount of work they do
- incentivizes hiring and reinvestment into production
- allows consumers to individually decide what goods and services they deem useful

Regional socialism takes the good aspects of markets (decentralized planning using profit as feedback) and merges them with sane wealth distribution mechanisms (no absentee ownership, means of production commonly owned, workers are paid regionally-set wages) while severely dampening the negative side-effects of profit-driven culture.

## Ownership

Companies are all regionally-owned. It's important to distinguish between regionally-owned and regionally-operated. Regionally-operated companies are run by a director or a board of directors that is appointed by members of the region. They are run with transparent accounting and often are subsidized in some manner by the region's capital pool.

All other companies are worker-operated. Worker-operated companies are still regionally-owned, but the region has no direct control over a company's existence, operation, or capital pool. All these things are controlled by the workers of a company.

All companies in the region are owned by the region, and the region itself is owned by its members. Therefor, every member is a part owner of every regional company. This ownership does not afford members any kind of special control over specific companies they are not employees of, but does allow making collective decisions on matters that affect the region as a whole (for instance when to buy or sell housing, office building, various machinery for production, etc).

It's important to mention legality and ownership. In the final chapter, a method of birthing regional socialism is described. It will likely be set up as a co-op structure that is a legal entity which owns the companies operating inside of it. In the event of a regional company acting in an illegal manner, the region may investigate and decide whether or not to close the company and/or fire the workers involved from the co-op.

Under this co-op structure, companies that reach a particular number of employees may be spun off as subsidiary companies to act as a legal barrier to protect the co-op while still maintaining regional control of the company.

## Wages and wage calculations

Worker wages are set by the members of the region. This is more complicated than it seems. There are a number of different factors that affect wages, and a number of different categories that wages fall under.

For instance, a company might have a CEO, and a region might set the CEO wage to $40/hour. Now all CEOs make $40/hour. However, let's say that a region decides to invest in its growing aeronautics industry. It wants the CEO of an aeronautics firm to make more than the CEO of a towing company, but a data-entry clerk at the aeronautics firm should make the same wage as a data-entry clerk at the towing company. Not so simple!

Let's imagine another situation. A region has a doctor shortage, so raises the wages for doctors, which causes doctors to move into the region to fill the available positions. Great! However, doctors keep moving in to try to get jobs. Too many doctors! The region decides to lower the doctor wage back to a previous rate to stop the influx. Does this mean the current doctors we worked hard to get will leave for greener pastures? Not necessarily. Wage reductions for existing workers can be handled such that they are delayed and/or are reduced linearly (decayed) over a regionally-set amount of time. The amount of delay (days until the wage starts changing) and rate of decay (days until the new wage takes full effect, not counting delay) are decided on a per-reduction basis. New workers who take on these jobs will immediately get the new lower wages, and workers who had the higher paying jobs before will have some amount of delay and slow decline before the wage reductions take full effect (and it's important to note that the delay can be set arbitrarily high to effectively grandfather-in previous wages). New downward changes to wages will essentially override the previous delay/decay parameters, but the new parameters will be applied to the wage at its current rate of decay. In other words, delay and decay only act to change the current wage, and cannot arbitrarily set the wage lower (unless delay and decay are both set to 0, which would force an immediate wage reduction).

Wages are broken out into two categories: occupation and industry. Every occupation is part of an industry, usually as a function of the company the occupation exists in. For instance a lawyer at a bank would be a lawyer by occupation in the finance industry. Let's look at the full calculation for a wage:

```
base_wage = STARTING_WAGE *
  (1 + (industry.need * occupation.industry_weight)) *
  (1 + occupation.skill) *
  (1 + occupation.need) *
  (1 + occupation.stress) *
  (1 + occupation.danger)
final_wage = base_wage +
  ((2 * (worker.productivity -- 0.5)) * occupation.productivity_rage * base_wage)
```

Every industry has a need parameter which defines how much a region values that industry, and each occupation has a number of parameters that affect the overall hourly rate. This lets a region adjust wages both for an entire industry and for individual occupations. This is a list of occupational parameters:

- Industry weight: A value between 0 and 1 that sets how much this occupation is affected by the corresponding industry's need. A data entry occupation that takes minimal training might have this value set close to 0 (so data entry in finance makes the same as data entry in education), and a lawyer might have it set higher (so if the region values the healthcare industry more than advertising, a lawyer in healthcare would make somewhat more than a lawyer in advertising).
- Need: A value between 0 and 1 that shows how much a region values this particular occupation. For instance, if there is a shortage of accountants, this value might go up, and if there is a surplus, it might go down.
- Skill: A value between 0 and 1 that determines the amount of skill an occupation has. This should ideally take into account the number of years of training a profession takes (as well as difficulty of training).
- Stress: A value between 0 and 1 that determines the physical and mental stress an occupation has. A dog walker would probably be 0. A surgeon might be close to 1.
- Danger: A value between 0 and 1 that sets the physical danger (risk or death or permanent injury) of an occupation. A teacher might be close to 0, and a construction worker might be close to 1.
- Productivity range: A value between 0 and 1 that sets the percentage the base wage can change given a worker's productivity level. In other words, with a base wage of $100/hr and a productivity range of 0.2 (or, 20%), the wage can fluctuate between $80 (at a productivity level of 0.0) and $120 (at a productivity level of 1.0). At a productivity level of 0.5, the base wage remains unchanged ($100).

It's important to note that many of these parameters may be set once and only adjusted infrequently. Need will probably be adjusted the most, and things like skill and danger may be adjusted infrequently. However, if the miner occupation has a high danger level, the introduction of a machine that lets the miner do their job remotely would significantly reduce the danger, and the danger level would be adjusted. A robotic-assist arm may reduce the stress level of surgeons quite a bit.

A worker's productivity level is set by the company she works for. If a worker is the only employee of a company, they can set their own productivity rate arbitrarily. In a larger company, productivity rates would likely be set by peer or managerial review. The process of how productivity rates are set for each worker is left up to the companies and their workers.

Productivity level along with productivity rate allows creating some range of wages within the same occupation. This allows companies a certain level of autonomy in setting their own wages while at the same time keeping the wishes of the community as a whole in mind. The goal of this system is to balance the needs of the overall region while still giving companies a certain level of control.

