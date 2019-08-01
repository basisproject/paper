# Chapter 4: Companies, workers, and wages

Companies are started by region members and operated by their workers. They are autonomous economic units that can set prices for their products and services however they see fit, as long as their income matches the inputs to their production (including regionally-defined wages).

## Profit

Companies can make profit. The difference with market capitalism, however, is that company profit cannot be arbitrarily distributed to workers. Profit must be spent, via the Factor system, on more inputs to production, hiring more workers, or buying products or services to make production more efficient. Profits which sit too long (using FIFO accounting) in a company's capital pool are taxed at a regionally-defined rate.

This separation of profit from the workers ensures a few things:

- Capital is distributed based on regional wages, not based on arbitrary market forces
- Profit is used mainly as an economic feedback mechanism

Over time, this mechanism redistributes capital based on regionally-decided wages. There is no longer a scenario where an absentee-owner gets rich while the workers get paid poverty wages.

Profit as a feedback mechanism is also important. It's extremely difficult to efficiently allocate resources and match supply with demand using any form of economic planning. On top of questions of planning, you get into situations where perhaps society's needs are met (food, shelter, clothing) but certain wants are completely unmet. You could construct a planned economy based on Marx's use-values, but who determines use-values? Who determines what is important or needed and what isn't?

Profit answers all these questions. It allows economic planning in a purely-decentralized manner, allocating resources to exactly where society desires them. Further, splitting the profit from from capital distribution allows profit to be used as an economic feedback mechanism *without distributing wealth in absurdly uneven ways*. Because profit can only be used for reinvestment and not personal gain, the endless drive for excess profit is significantly reduced along with the negative side-effects that come with it (such as pollution and regulatory capture).

This method of economics

- foregoes the need for central or detailed planning
- tames profit-driven greed and reduces pressure for negative externalities (such as pollution)
- distributes wealth to workers based on the value and amount of work they do
- incentivizes hiring and reinvestment into production
- allows consumers to individually decide what goods and services they deem useful

Regional socialism takes the good aspects of markets (decentralized planning using profit as feedback) and merges them with sane wealth distribution mechanisms (no absentee ownership, means of production commonly owned, workers are paid regionally-set wages) while severely dampening the negative side-effects of profit-driven culture.

## Ownership

Companies are separate entities from the region and are owned by their workers. The region itself does not have any membership or ownership in the worker-owned companies. In other words, the region has no control over member companies.

How companies decide to set up their ownership model is immaterial as long as *all employees are owners*. They might decide to distribute ownership by number of hours worked in a given year, or perhaps by overall wages paid out. The goal is not to micromanage company structure or operations, but instead to ensure that all workers have a say in the operation of their workplace.

It's worth mentioning though that if a company does deviate from the economic framework, the company, and its employees, will lose access to regional services. This applies not only to subsidized office space and favorable loan conditions from the regional bank, but also the subsidized housing, education, social dividend, and all other services. Essentially, the companies play by the rules, or membership is lost and the region switches to profit generation mode.

### Investment (and ownership classes)

Companies are owned by their workers. This is essential. However, this precludes outside investment by selling ownership shares of a company, which bars companies from raising capital by selling shares.

However, because venture capital is an effective way of testing riskier enterprises, preserving this mechanism is important for economic success. This can be set up as follows: companies will have governance shares which *must be distributed to the employees* and which allow voting for board members or referenda but *companies will also have dividend shares*. The dividend shares *cannot* be distributed to the workers, and *can only be sold to the public bank*. The shares entitle the bank to a portion of the company's profit, and how many shares the bank buys and at what price is up to whatever arrangement the bank and company come to during negotiations.

This setup essentially allows venture capital *without private ownership*. Absentee ownership is allowed, but only by an institution owned by the region (and, thus, the people of the region). In this way, absentee ownership is socialized, and the profits of successful ventures can be used to purchase more housing or means of production.

## Wages and wage calculations

Worker wages are set by the members of the region. This is more complicated than it seems. There are a number of different factors that affect wages, and a number of different categories that wages fall under.

For instance, a company might have a CEO, and a region might set the CEO wage to $60/hour. Now all CEOs make $60/hour. However, let's say that a region decides to invest in its growing aeronautics industry. It wants the CEO of an aeronautics firm to make more than the CEO of a towing company, but a data-entry clerk at the aeronautics firm should make the same wage as a data-entry clerk at the towing company. Not so simple!

Let's imagine another situation. A region has a doctor shortage, so raises the wages for doctors, which causes doctors to move into the region to fill the available positions. Great! However, doctors keep moving in to try to get jobs. Too many doctors! The region decides to lower the doctor wage back to a previous rate to stop the influx. Does this mean the current doctors we worked hard to get will leave for greener pastures? Not necessarily. Wage reductions for existing workers can be handled such that they are delayed and/or are reduced linearly (decayed) over a regionally-set amount of time. The amount of delay (days until the wage starts changing) and rate of decay (days until the new wage takes full effect, not counting delay) are decided on a per-reduction basis. New workers who take on these jobs will immediately get the new lower wages, and workers who had the higher paying jobs before will have some amount of delay and slow decline before the wage reductions take full effect (and it's important to note that the delay can be set arbitrarily high to effectively grandfather-in previous wages). New downward changes to wages will essentially override the previous delay/decay parameters, but the new parameters will be applied to the wage at its current rate of decay. In other words, delay and decay only act to change the current wage, and cannot arbitrarily set the wage lower (unless delay and decay are both set to 0, which will force an immediate wage reduction).

Wages are broken out into two categories: occupation and industry. Every occupation is part of an industry, usually as a function of the company the occupation exists in. For instance a lawyer at a bank will be a lawyer by occupation in the finance industry. Let's look at the full calculation for a wage:

```
wage_ratio = Avg(
    industry.need * occupation.industry_weight,
    occupation.skill,
    occupation.need,
    occupation.stress,
    occupation.danger
)
base_wage = STARTING_WAGE + (wage_ratio * WAGE_MULTIPLIER)
productivity_adjuster = (2 * (productivity - 0.5)) * (productivity_range * base_wage)
final_wage = Max(MINIMUM_WAGE, base_wage + productivity_adjuster)
```

Every industry has a need parameter which defines how much a region values that industry, and each occupation has a number of parameters that affect the overall hourly rate. This lets a region adjust wages both for an entire industry and for individual occupations. This is a list of occupational parameters:

- Industry weight: A value between 0 and 1 that sets how much this occupation is affected by the corresponding industry's need. A data entry occupation that takes minimal training might have this value set close to 0 (so data entry in finance makes the same as data entry in education), and a lawyer might have it set higher (so if the region values the healthcare industry more than advertising, a lawyer in healthcare will make somewhat more than a lawyer in advertising).
- Need: A value between 0 and 1 that shows how much a region values this particular occupation. For instance, if there is a shortage of accountants, this value might go up, and if there is a surplus, it might go down.
- Skill: A value between 0 and 1 that determines the amount of skill an occupation has. This should ideally take into account the number of years of training a profession takes (as well as difficulty of training).
- Stress: A value between 0 and 1 that determines the physical and mental stress an occupation has. A dog walker would probably be 0. A surgeon might be close to 1.
- Danger: A value between 0 and 1 that sets the physical danger (risk or death or permanent injury) of an occupation. A teacher might be close to 0, and a construction worker might be close to 1.
- Productivity range: A value between 0 and 1 that sets the percentage the base wage can change given a worker's productivity level. In other words, with a base wage of $100/hr and a productivity range of 0.2 (or, 20%), the wage can fluctuate between $80 (at a productivity level of 0.0) and $120 (at a productivity level of 1.0). At a productivity level of 0.5, the base wage remains unchanged ($100).

It's important to note that many of these parameters may be set once and only adjusted infrequently. Need will probably be adjusted the most, and things like skill and danger may be adjusted infrequently. However, if the miner occupation has a high danger level, the introduction of a machine that lets the miner do their job remotely will significantly reduce the danger, and the danger level might be adjusted. A robotic-assist arm may reduce the stress level of surgeons quite a bit.

A worker's productivity level is set by the company she works for. If a worker is the only employee of a company, she can set her own productivity rate arbitrarily. In a larger company, productivity rates will likely be set by peer or managerial review. The process of how productivity rates are set for each worker is left up to the companies and their workers.

Productivity level, along with productivity rate, allows creating some range of wages within the same occupation. This allows companies a certain level of autonomy in setting their own wages while at the same time keeping the wishes of the community as a whole in mind. The goal of this system is to balance the needs of the overall region while still giving companies a certain level of control.

## Taxation

Taxation of companies will be necessary to sustain growth of the region. Taxes will be held in the public bank and can be used in any number of ways:

- Paying for management of the region
- Debt forgiveness (for either regionally-owned or worker-owned companies)
- Subsidies for regional services (housing, education, etc)
- Social dividend

It's important to note that after paying for the management of the region itself (wages and maintenance of the regional co-op), the rest of the money from taxations is essentially at the disposal of the regional members' will. They can issue it back to themselves via dividend, they can grow the region by buying more housing and more offices/warehouses/etc, they can subsidize housing or education, or they can decide to just lower taxes altogether.

Also, while the types of taxes available are listed below, that does not necessarily mean a region will decide to impose them. It may only select a few and set the rest to 0. The following taxes are additive.

### Company net income

Companies pay a regionally-decided flat tax on net income.

### Reserve fund tax

Profitable companies that keep more than X% of their total expenses for a year in cash will have the amount above X% of their expenses taxed by Y%.

In other words, let's say X is 80% and Y is 10%. If a company has $114K in revenue, $92K in expenses (so a profit of $22K), and $214K in cash reserves, then 80% of its expenses would be $73.6K (`0.8 * 92K`) and it would pay `0.10 * (214K - 73.6K)`, or ~$14K.

The purpose of this tax is to incentivize reinvestment without penalizing saving for difficult times. It is essentially a wealth tax with a dynamic threshold (based on yearly expenses).

### Employee income tax

Employee incomes will be taxed using regionally-decided progressive tax brackets.

### Size tax

Company profits will be taxed based on the number of employees they have. This will be based off of regionally-decided size brackets. Here's an example of what size brackets might look like:

- 0-999 (0%)
- 1000-9999 (5%)
- 10000-19999 (10%)
- 20000-39999 (20%)
- 40000-99999 (40%)
- 100000+ (80%)

So a company with 14239 employees will pay a 10% size tax.

The purpose of this tax is is to put downward pressure on company sizes and limit the economic power of any one company. This incentivizes multiple smaller companies instead of a few large ones.

If a region has 100 companies with 50 employees or less and one company with 20000 employees, then the board of the company with 20000 employees has much more leverage and can be taxed for this economic power.

## Loss of membership

Companies that refuse to participate within the rules of the region may lose membership. When a company loses membership, it loses all benefits:

- preferential loan rates
- at-cost pricing for productive assets
- housing, education, and other services for the company's worker-members

Loss of membership will be determined by the High Council, the region's highest authority. It will likely issue warnings before a company permanently loses membership.

