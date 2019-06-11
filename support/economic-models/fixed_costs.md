# Fixed-cost pricing

The idea of this model is that things cost exactly what it costs to make them,
taking into account all inputs (materials and labor).

Companies have freedom of who they buy from, methods of production, and who they
sell to. Company performance (ie social "profit") tracking is a huge WIP.

This model requires extensive tracking of costs along the supply chain. Note
that this model, if desired, would likely come *after* the company-profit model.

## Extension of base

- equations:
  - cost of labor
    - CostOfUnskilledLabor: BaseSalary
    - CostOfSkilledLabor: BaseSalary

- companies:
  - product pricing:
    - companies sell products for exactly the cost the make them
      - WIP: this was the original idea, and has not been fully thought out
      - production expands via reinvestment from bank/conductor based on output numbers (automated feedback loop)
        - (paid for by some form of conductor-adjustable sales tax)
        - allows community-controlled expansion/contraction of enterprises
        - reinvestment value MUST be high enough to pay back loans
        - workers paid directly from region/state pool, not company pool
      - can possibly be implemented later on
      - pros:
        - products cost exactly what they cost to make, allowing exact value assessment
        - removes artificial profit motives/greed aspect
        - economy more "controlled" in the sense that excess/boom/bust less likely
      - cons:
        - removes control from workers of enterprise (bad for morale?)
        - WIP: requires loans to weather lulls in demand
        - more "centralized"/controlled, feels forced
        - reeks of overcomplication
        - cost of things much harder to determine (conductor calculates final price, pricing needs exact calculations)
  - services:
    - services needed listed in conductor
    - services provided listed in conductor
    - bookings happen via conductor
      - allows tracking 
  - performance:
    - WIP

