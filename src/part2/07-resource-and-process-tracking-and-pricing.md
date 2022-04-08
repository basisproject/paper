## Chapter 7: Resource and process tracking and pricing

TODO:

- Submitting resources for tracking
- Submitting processes for tracking
- Approval of tracking
- Determining parameters for tracking (price and ratio of inputs for processes)

Essentially we're talking about a buttload of crowdsourcing and defining all the processes for doing so.

#### Converting costs to a single value

TODO: make note in wages section that 1 labor hour = 1 credit

Although costs are tracked as separate collections of distinct values, it's important that they are able to be summed into one aggregate number: a total cost. This allows blocs to get a quick sense for their overall cost amounts, which factors into determine how much of their [cost allowance][cost-allowance] is used.

Deriving this total value is simple in the case of `labor` and `labor_hours`, we simply sum the values together. For instance, in the object above, we would have `12.89 + 3.38 + 41.45 + 0.2578 + 0.0965 + 1.0363` or `59.1106`.

The tricky bit comes when we convert our resource values into a cost, and this is where [tracked resources][tracked-resources] and [tracked processes][tracked-processes] come in: they allow categorizing a [resource][resources] within the cost tracking system such that a standard unit can be assigned a cost value. Thus, if in our example above, `iron` is tracked in grams, and the cost-per-gram is `0.3` then the iron cost would be `8.5 * 0.3` or `2.55`. If done for all `resources` in the costs set, we can add this to our labor costs and find our total aggregate cost value.

