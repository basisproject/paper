# Chapter 6: Housing

A goal of regional socialism is to provide housing to members of the region at-cost (or, depending on the region, perhaps only a percentage of cost). The idea is to attract people outside the regional economy to join and participate in the economy by providing housing at a much cheaper rate than the outside. Even if someone in the regional economy makes less money doing their job than they did outside in the capitalist world, housing (and possibly other services) will be cheaper inside the economy and will offset the difference.

Regionally-owned housing will likely be a mix of apartments, duplexes, triplexes, and single-family homes. Each property (and/or each unit in an apartment complex) will be given a luxury ranking, which will affect the per-square-footage rate.

In essence, the region will be a landlord who only raises rent if the underlying costs of the properties go up. Property management is handled by a regionally-operated company with an elected board of directors.

COUNTERPOINT: if a member gains reduced-price housing, then leaves the economy, could the region evict them?

## Property management

All properties will be owned and managed by the regional housing company. The board of this company will be decided democratically by the members of the region, and the board will hire the top-level management to execute the duties of management.

The housing company will acquire (or sell property) based on the regionally-set wait-list thresholds. In other words, the region might decide "when the housing wait list reaches 5% or more of the total population of the region, purchase more housing."

## Pricing

The cost of housing for the entire region will be averaged for managed properties. The idea behind this is that if a particular property finishes paying off a loan, the members living in that building would suddenly have much lower rent. Is that fair to the members who live in the apartments across the street in a building with 10 years left on their loan?

In essence, the housing company will want to average these costs out, such that "at-cost" means, overall, the housing company makes 0% profit on all the properties it manages over time (of course, it will likely need to make *some* profit as a buffer to handle unexpected expenses, but within a 2-3% margin would likely be acceptable). These margins would be transparent to all members.

Properties (and units, in the case of apartments or duplexes) would all get luxury indexes between 0.0 and 1.0 such that 0.0 means you live in a basement with no windows and mud floors, and 1.0 means you are living in palatial conditions fit for a queen. Of course, this factors into the cost, and might take things into account like location, appliances, view, etc. A per-unit luxury score would average against a per-property luxury score. In other words, if the property is ranked at 0.8 and a unit is ranked at 0.6, the final score that the resident would pay for is 0.7. The luxury score is essentially used to try to match the overall demand for a particular unit would it be rented on the open market.

The monthly price calculation for any unit is follows:

```
((unit_square_footage * unit_luxury) / (total_square_footage * average_luxury)) *
    (total_yearly_costs / 12)
```

In these calculations, `square_footage` *incorporates land*. In other words, if renting a house, you're not just paying for the square footage of the house itself, but also for the square footage of the land. The idea is you pay for the space you use. `unit_luxury` is the luxury index of the particular unit being occupied. `total_square_footage` is the summed square footage of *all units managed by the region*, and `average_luxury` is the average luxury index *of all managed units*. `total_yearly_costs` is the summed expenses of *all costs* of the housing company: mortgage payments, property maintenance, wages of housing company employees, etc.

The idea is that for each unit, you pay the total percentage of costs that your unit encompasses from those costs. Of course, this is *entirely* going to match because the housing company has to account for things like unit vacancies (which inflate the cost for everyone) but the idea is that year over year these costs average out such that the housing company makes little to no profit.

## Transparency

Like other regionally-operated companies, costs of housing would be transparent and accounted through the Conductor system, allowing members to see the overall costs and to see exactly how their rent prices are derived.

## Home purchasing

Regional socialism -- while possibly engaging in the purchasing of condos, duplexes, and single-family homes at its own will -- takes no position on its members buying or selling non-regionally-owned property inside or outside the geographical region. While property speculation will be a method of generating personal profit, the complications of attempting to regulate this market are much too numerous and will likely stifle participation.

It is hoped that the region buying up properties and setting rents fairly will have a calming effect on the surrounding housing markets, reducing the amount of profit that real estate speculation can deliver.

As with other secondary markets, regional socialism takes a passive stance, even with items as large as property. That said, as with other secondary markets, individuals will be competing with regional socialism and may find it more profitable to operate in the primary markets rather than secondary.

