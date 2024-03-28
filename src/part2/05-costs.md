## Chapter 5: Costs

Cost tracking is the core of Basis; its primary function. How costs are interpreted or acted upon is important, but secondary to the actual gathering of costing information.

When people think of a cost, they often think of a number, and usually a price. Basis expands this concept of cost into something much more dimensional than a number. It tries to answer the question "what did it cost to make something?" How much labor? How much raw materials? What transformational processes did resources/raw materials go through to become the final product?

Costs in Basis are not a number, but rather separate collections of individual values bundled together. By default, this includes:

- `labor-hours`: Hours of labor, grouped by occupation. Occupations are managed as [network-global parameters][network-parameters].
- `labor-wages`: Credits paid to workers, grouped by occupation. Occupations here are the same dataset as those managed in `labor-hours`.
- `resources`: Generally raw/semi-raw materials, grouped by the resource/raw material type, and optionally broken down further by extraction location. Resources are extensions of [ValueFlows resources][ext-vf-resources].
- `processes`: Economic processes that transform resources (such as "refine oil"). Processes are extensions of [ValueFlows processes][ext-vf-processes].

Note that this is known as in-kind [^cik] cost tracking.

Resources and processes tracked as part of costs objects are not exhaustive. While it generally makes sense to track every kind of labor that goes into building something (so workers can be remunerated), it doesn't necessarily make sense to track every single resource type or every single economic process. The general idea is to track resources that are scarce so their consumption rates are known and to track resources/processes *that have known externalities* so their costs can be internalized.

For instance, we know that burning gasoline has externalities relating to carbon output. We could track the *process of burning gasoline*, however doing so would require a lot of overhead. Instead it might make sense to track just gasoline, assume that it's in almost all cases going to be burned, and assign the externality cost to the resource itself.

But how do we get gasoline? This is where processes come in. Not only can processes in Basis be assigned an externality cost, but they can also transform resources: crude oil is *consumed* by the process of "refining" and becomes gasoline, jet fuel, etc.

#### Distinction between tracked processes and bloc processes

It's worth noting that the [processes blocs use when producing][bloc-processes] are determined by the bloc and are independent of the concept of *tracked processes* described above.

For instance, a bloc might have the process `build chair` it uses when producing chairs, but `build chair` does not need to be a network-wide tracked process in order for that bloc to use it. In other words, blocs are free to track internal processes in any way they deem fit, *however* cost resource transformations (crude oil -> gasoline, jet fuel, etc) can *only* happen through network-tracked processes which are created and maintained via democratic process.

### Cost representation

Here's an example of what a cost object might look like:

```
{
    labor_hours: {
        president: 0.2578
        accountant: 0.0965
        miner: 1.0363
    }
    labor_wages: {
        president: 15.468
        accountant: 3.86
        miner: 36.2705
    }
    resources: {
        iron: 8.5
        gasoline: 2.9
        silicon: 0.03
    }
    processes: {
        refine: {crude oil: 5.8, methane: 0.5}
    }
}
```

The actual labels like `president`, `iron`, `refine`, etc will generally not be names, but rather be network-specific uniform resource identifiers that point to a part of the network's [global storage][network-parameters]. We use names here for the purposes of simplicity to show how costs are structured.

### Cost extensions

The protocol allows cost objects to be extended to track other types of costs. For instance, `currency` (covered in [part 3][currency-tracking]).

This accomplishes two things: it keeps the Basis cost tracking core lean and simple, and it allows tracking cost types that the protocol's creators couldn't think up at the time of creation.

### Mathematical operations on costs

Even though costs are not single numeric values, they can still have a number of mathematical operations performed on them. Here are a few examples:

#### Addition

Costs can be added to each other. As an example:

```
  {
      labor_hours: {
          lumberjack: 3
      }
      labor_wages: {
          lumberjack: 60
      }
      resources: {
          wood: 500
          gasoline: 4
      }
      processes: {
          refine: {oil: 8, methane: 0.7}
      }
  }
+ {
      labor_hours: {
          trucker: 1.5
      }
      labor_wages: {
          trucker: 37.5
      }
      resources: {
          diesel: 3
      }
      processes: {
          refine: {oil: 9, methane: 0.8}
      }
  }
----------------------------------------
  {
      labor_hours: {
          lumberjack: 3
          trucker: 1.5
      }
      labor_wages: {
          lumberjack: 60
          trucker: 37.5
      }
      resources: {
          wood: 500
          gasoline: 4
          diesel: 3
      }
      processes: {
          refine: {oil: 17, methane: 1.5}
      }
  }
```

#### Subtraction

Costs can also be subtracted from each other, although costs with negative values are invalid and *will result in an error*.

```
  {
      labor_hours: {
          lumberjack: 3
          trucker: 2
      }
      labor_wages: {
          lumberjack: 60
          trucker: 50
      }
      resources: {
          wood: 500
          gasoline: 4
      }
      processes: {
          refine: {oil: 8, methane: 0.7}
      }
  }
- {
      labor_hours: {
          lumberjack: 1
      }
      labor_wages: {
          lumberjack: 20
      }
      resources: {
          wood: 200
          gasoline: 1
      }
      processes: {
          refine: {oil: 2, methane: 0.1}
      }
  }
----------------------------------------
  {
      labor_hours: {
          lumberjack: 2
          trucker: 2
      }
      labor_wages: {
          lumberjack: 40
          trucker: 50
      }
      resources: {
          wood: 300
          gasoline: 3
      }
      processes: {
          refine: {oil: 6, methane: 0.6}
      }
  }
```

As mentioned, costs with negative values are invalid, so

```
  { labor_hours: { carpenter: 15 } }
- { labor_hours: { carpenter: 5, trucker: 3 } }
-----------------------------------------------
  { labor_hours: { carpenter: 10, trucker: -3 } }
```

would result in a protocol error. To allow this would be unfair to our friend, the trucker, whose labor would be negated.

#### Multiplication

Costs can be multiplied by a decimal value:

```
  { labor_hours: { carpenter: 2 }, resources: { wood: 5 } }
* 3.5
-----------------------------------------------------------
  { labor_hours: { carpenter: 7 }, resources: { wood: 17.5 } }
```

#### Division

Costs can be divided by a decimal value as well:

```
  { labor_hours: { carpenter: 16 }, resources: { wood: 12 } }
/ 4
-------------------------------------------------------------
  { labor_hours: { carpenter: 4 }, resources: { wood: 3 } }
```

### Incentives

We've talked about what types of costs can be tracked, but we haven't covered *why a bloc would bother tracking costs*. What do they get out of it?

In essence, the system pays them to track the costs. For instance, how does Basis know to pay a worker their wage? Well, they would track that cost in the system, and as a result they would get payment. The same principle works for all cost tracking: when a bloc tracks the cost, the workers of that bloc get paid.

The exact mechanisms at play here are covered in the [cost staging section][cost-staging], which balances the incentive to create new costs with the base goal of meeting a social need.
