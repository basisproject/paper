## Chapter 7: Trackers (occupations, resources, and processes)

We've covered the [flows of costs][cost-flows] and we've covered how [labor is tracked][labor-wages], but we still need to detail *how occupations, resources, and processes are categorized* as well as *the incentives blocs have for tracking them.*

### Trackers

Anything that can be tallied as part of a [set of costs][costs] is described by what is called a "tracker." A tracker is an intent created by the agents of the network that guide costs to incorporate certain information.

Trackers come in three different classes: occupations, resources, and processes. Each class is structured as a hierarchical tree: a top-level node that can have unlimited children, with each child capable of having unlimited children, and so on.

Every tracker in the network has a set of standard fields that all trackers share, as well as any number of translations for that tracker:

```
Translatable {
  translations: {
    en-US: {
      name: <english tracker name>
      description: <english tracker description>
    }
    <localized names/descriptions> ...
  }
}

Tracker: Translatable {
  id: <unique id>
  parent: <unique id>
  children: [ <unique id> ... ]
  <type-specific data> ...
}
```

TODO:

- [#140 - Crowdsourced trackers](https://github.com/basisproject/tracker/issues/140)

### Occupations

TODO:

- [#89 - Tracked occupation governance](https://github.com/basisproject/tracker/issues/89)

### Resources

- [#17 - Tracked resource governance](https://github.com/basisproject/tracker/issues/17)

### Processes

- [#137 - Tracked process governance](https://github.com/basisproject/tracker/issues/137)

### Converting costs to a single value

TODO: make note in wages section that 1 labor hour = 1 credit

Although costs are tracked as separate collections of distinct values, it's important that they are able to be summed into one aggregate number: a total cost. This allows blocs to get a quick sense for their overall cost amounts, which factors into determine how much of their [allocation][allocation] is used.

Deriving this total value is simple in the case of `labor` and `labor_hours`, we simply sum the values together. For instance, in the object above, we would have `12.89 + 3.38 + 41.45 + 0.2578 + 0.0965 + 1.0363` or `59.1106`.

The tricky bit comes when we convert our resource values into a cost, and this is where [tracked resources][tracked-resources] and [tracked processes][tracked-processes] come in: they allow categorizing a [resource][resources] within the cost tracking system such that a standard unit can be assigned a cost value. Thus, if in our example above, `iron` is tracked in grams, and the cost-per-gram is `0.3` then the iron cost would be `8.5 * 0.3` or `2.55`. If done for all `resources` in the costs set, we can add this to our labor costs and find our total aggregate cost value.

