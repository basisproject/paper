# Part 3: The Real World

TODO: This is going to be kind of a dumping ground for all the random transitional garbage from the old paper until I organize it.

---------

## UBI

- UBI can *only* be spent on non-currency costs of products and services. In other words, if an apple has a cost of `3₡ + $2`, the UBI can pay for the apple's three credits, but the $2 of currency value will have to be covered by credits obtained through labor. The reason for this is that currency costs are an obligation to the market that must be paid back by value creation. If this obligation is not met, the network will go bankrupt and fail. This has another interesting side effect though: producers are incentivized to obtain their inputs to production in-network because it means their outputs will have less currency and consumers will be able to afford them at a higher rate. In other words, a systemic incentive on productive self-sufficiency.

---------

#### Currency tracking

When a company needs to buy something from capitalist markets, it spends currency out of a [capital pool](#capital-pools) to do so. The cost of this currency, like all other costs, is tracked by Basis. This is covered in more detail in [the banking section](#currency-tracking-1).

---------

# Chapter 8: Governance

TODO:

- [#60 - Voting](https://github.com/basisproject/tracker/issues/60)

## Network self-sufficiency

A very important aspect to the Basis network is self-sufficiency: how well is it producing the things its member's need both in inputs to production and products for consumption? The more we rely on the outside market, the lower the sufficiency, and the more vulnerable the network is.

We use this measurement of sufficiency as a way to automate phasing different states of the network in and out. The idea is that while the network is young and vulnerable, certain aspects are "fixed" and as the network grows and becomes more self-reliant, more democratic control is used as a governance mechanism. For instance, see the section on [UBI governance]

### Definition of self sufficiency

The basic idea here is that network self-sufficiency can be measured by how much people need to go out-of-network to get things. This can be measured on the consumer side via

```
consumer_sufficiency = (credits_printed_over_time - withdraws_over_time) / credits_printed_over_time
```

Here, `credits_printed_over_time` is the *total* measurement of labor credits we have printed over some period of time (excluding UBI), and `withdraws_over_time` is a measurement of how many credits were converted to currency over the same period of time.

Consumers converting credits -> currency means that they wish to buy or pay for something that is not available in network, whether this is housing, gadgets, vehicles, etc. It means the network is not providing something they want.

We can measure sufficiency on the producer side via

```
producer_sufficiency = 1 - (order_credit_value_in_currency_over_time / order_credit_value_total_over_time)
```

Effectively `order_credit_value_in_currency_over_time` is the amount of currency *leaving the system* into the market over a period of time, measured in its value in *credits* (if we maintain a USD peg then this ratio would be 1:1 for USD, and would fluctuate for other currencies). `order_credit_value_total_over_time` is the total *credit value* of all orders of producers (in-network or otherwise) over the same time period. So what we're determining here is the rate of total *currency* leaving the system (measured in credits) vs credit value of all productive (not consumer) orders in the system. This gives us a ratio which measures how much our productive system depends on the market (which requires currency to participate in).

So our final sufficiency would be a simple average:

```
sufficiency = (consumer_sufficiency + producer_sufficiency) / 2
```

This value, which starts at 0 and as the network relies less and less on the market approaches 1, is used to smoothly transition network states. This removes the need for human controls or governance of certain mechanisms and allows the network to shift operations as it grows.

---

# Chapter 10: Duality

TODO



