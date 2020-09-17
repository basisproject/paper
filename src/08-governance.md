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
