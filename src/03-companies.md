# Chapter 3: Companies, workers, and wages

Companies are started by region members and operated by their workers. They are autonomous economic units that build and produce things as they see fit. In order to be considered for regional membership, companies must:

- Be worker-owned such that all employees have some controlling share in the company
- Use the Basis system for all accounting (incoming orders, outgoing orders, and worker time tracking)
- Meet some social need, defined as their final products being purchased by consumers or used as the productive input to other companies that have final products purchased by consumers

## Ownership

Companies are separate entities from the region and are owned by their workers. The region itself does not have any membership or ownership in the worker-owned companies. In other words, the region has no control over member companies, other than ultimately deciding whether they (and their workers) are members or not.

How companies decide to set up their ownership model is immaterial as long as *all employees are owners*. They might decide to distribute ownership by number of hours worked in a given year, or perhaps by overall wages paid out. The goal is not to micromanage company structure or operations, but instead to ensure that all workers have a say in the operation of their workplace.

It's worth mentioning though that if a company does deviate from the economic framework, the company, and its employees, will lose access to regional services. This applies not only to subsidized office space, but also the subsidized housing, education, social dividend, and all other services. Essentially, the companies play by the rules, or membership is lost and the region switches to profit generation mode for the company *and* its workers.

## Wages and wage calculations

Worker wages are partially set by the members of the region. This is more complicated than it seems. There are a number of different factors that affect wages, and a number of different categories that wages fall under.

For instance, a company might have a president, and a region might set the president wage to $60/hour. Now all presidents make $60/hour. However, let's say that a region decides to invest in its growing aeronautics industry. It wants the president of an aeronautics firm to make more than the president of a towing company, but a data-entry clerk at the aeronautics firm should make the same wage as a data-entry clerk at the towing company. Not so simple!

Let's imagine another situation. A region has a doctor shortage, so raises the wages for doctors, which causes doctors to move into the region to fill the available positions. Great! However, more doctors moved in that there are positions. Too many doctors! The region decides to lower the doctor wage back to a previous rate to stop the influx. Does this mean the current doctors we worked hard to get will leave for greener pastures? Not necessarily. Wage reductions for existing workers can be handled such that they are delayed and/or are reduced linearly (decayed) over a regionally-set amount of time. The amount of delay (days until the wage starts changing) and rate of decay (days until the new wage takes full effect, not counting delay) are decided on a per-reduction basis. New workers who take on these jobs will immediately get the new lower wages, and workers who had the higher paying jobs before will have some amount of delay and slow decline before the wage reductions take full effect (and it's important to note that the delay can be set arbitrarily high to effectively grandfather-in previous wages). New downward changes to wages will essentially override the previous delay/decay parameters, but the new parameters will be applied to the wage at its current rate of decay. In other words, delay and decay only act to change the current wage, and cannot arbitrarily set the wage lower (unless delay and decay are both set to 0, which will force an immediate wage reduction).

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
- Danger: A value between 0 and 1 that sets the physical danger (risk of death or permanent injury) of an occupation. A teacher might be close to 0, and a construction worker might be close to 1.
- Productivity range: A value between 0 and 1 that sets the percentage the base wage can change given a worker's productivity level. In other words, with a base wage of $100/hr and a productivity range of 0.2 (or, 20%), the wage can fluctuate between $80 (at a productivity level of 0.0) and $120 (at a productivity level of 1.0). At a productivity level of 0.5, the base wage remains unchanged ($100).

It's important to note that many of these parameters may be set once and only adjusted infrequently. `Need` will probably be adjusted the most, and things like skill and danger may be adjusted infrequently. However, if the miner occupation has a high danger level, the introduction of a machine that lets the miner do their job remotely will significantly reduce the danger, and the danger level might be adjusted. A robotic-assist arm may reduce the stress level of surgeons quite a bit.

A worker's productivity level is set by the company she works for. If a worker is the only employee of a company, she can set her own productivity rate arbitrarily. In a larger company, productivity rates will likely be set by peer or managerial review. The process of how productivity rates are set for each worker is left up to the companies and their workers.

Productivity level, along with productivity rate, allows creating some range of wages within the same occupation. This allows companies a certain level of autonomy in setting their own wages while at the same time keeping the wishes of the community as a whole in mind. The goal of this system is to balance the needs of the overall region while still giving companies a certain level of control. The higher the regional `productivity_range` parameter, the more autonomy companies have in setting their own wages.

## Product and service pricing

All product and service pricing will be internally automated such that given the operating costs of the company, the products and services offered are priced at exactly the average cost of operation over time. The idea here is to eliminate all profit entirely and price final products at the exact cost to make them.

Since companies will be using Basis for their accounting as well as the medium of exchange for their products and services, pricing products can be automated.

It's important to note that companies can still give their products and services a market price (and are encouraged to do so), so companies *outside* of the Basis economy buying their products will be paying the market price (as opposed to the at-cost value provided to in-system members). The market currency for outside purchases goes directly to the regional bank (so companies never deal directly with any currency). The market price for products and services *must be set at or above the at-cost value of the product* as determined by the current exchange ratio (and Basis will help automate this).

The actual algorithm for pricing products is fairly in-depth, so it won't be listed here. However, as a quick description: each product is given an `effort` value, which is essentially how much time it takes to produce. It also has the inputs to productions it requires associated with it. Using these values we can determine given the total costs of the inputs to production, the total operating costs of the company, and the number of items produced under each product type, the breakdown of costs as they are associated with each product.

See here for an standalone model of the product pricing system: <https://gitlab.com/basis-/product-costs>

## Accounting between member companies

Member companies will have no system of accounting between them. No currency will exchange when orders between companies are completed.

That said, *costs* will still be accounted for in the final products. Let's take an example. A chair maker needs wood to make her chairs. She orders the wood (and remember, there is no bank account she uses to buy the wood from, she simply makes the order) at a cost equal to the raw-material cost of trees plus the cost of the labor to cut, mill, and ship them. Then she spends five hours making each chair. Ultimately her final product is the cost of her labor, plus the cost of the milled wood, plus the cost of the rent of her workshop etc, averaged over some long operational period. When another member company orders her chairs, the cost of the chairs are added to *their* average costs, and whatever things they produce will include the cost of the chairs in them.

So if there's no system of accounting between member companies, what keeps the chair maker from clocking in and twiddling her thumbs all day? If there is no company bank account tracking expenses, she can get paid for sitting around doing nothing! There are a few things at play here that keep people from taking advantage. First, because the system is entirely transparent, anybody can see that she's not fulfilling orders and her company costs are ballooning. Even if her and another company colluded to just order each other's products ceaselessly, it would be fairly easy to see that nothing useful was being produced, and they are therefor not meeting any social need. Secondly, because all costs are accounted for, if she did only make one chair per week while collecting all of her pay, the costs of her chairs would be astronomically high, because one chair takes `(40 * hourly_wage) + weekly_operational_costs`.

In the extreme case where a number of companies collude and never actually produce anything that meets some social need, they could be found by their interrelated transactions and ever-growing cost pool. They would simply lose membership, and their workers would either need to seek employment at other member companies or they would need to find their fortune in the outside market system.

## Accounting with non-member companies

Until all regions acting together are, as a whole, self-sufficient between themselves, it will be necessary for them to interact with the outside market system. This entails buying products and services from outside the region (and we covered selling products at market prices in the last section). If member companies do not actually manage any capital, then how does this happen?

TODO. Ideas:

- monthly/yearly voucher pool stipend which can be spent out of only for market expenses?
- would need to be dependent on the region's incoming capital vs outgoing capital
- systemic incentives to strongly favor intra-region purchasing
- essentially, regional bank needs to have more incoming than outgoing =]
  - this is where regional allocation comes into play
  - likely smaller regions will have to be very selective about which companies can participate


## Loss of membership

Companies that refuse to participate within the rules of the region may lose membership. When a company loses membership, it loses all benefits:

- at-cost pricing for productive assets
- at-cost pricing for housing, education, and other services for the company's worker-members

Loss of membership will be determined by the Regional Council. It will likely issue warnings before a company permanently loses membership.

