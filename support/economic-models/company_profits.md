# Company-profit pricing

The idea of this model is that companies are free to charge whatever price they
want for the goods they produce. It's up to companies to decide how to price
goods/services such that they remain in business. Companies can be in
competition, but it would be a competition to improve processes/outputs and
satisfy worker needs (instead of a competition for profit).

One caveat is that company profits are not redistributable to workers. Profits
must be re-invested into hiring more people and/or increasing production.
Profits-at-rest are taxed by some regionally-decided variable amount.

This model can be much more lax about the planning aspect of the economy, in
fact I'm not sure to what extent Basis would even be needed.

## Extension of base

- equations:
  - WorkWeekHours: number of hours a full week of work entails (ie, 40 hours)
    - decided per-region/per-industry
  - CompanyWorkerHours:
    - for each worker at a company:
      - sum the number of hours, capped at WorkWeekHours
    - tracks how many workers a company gainfully employs
  - CompanyWorkerInjection: Max(CompanyWorkerHours - WorkWeekHours, 0) * EmploymentInjection
    - EmploymentInjection: a regionally-decided value controlling employment-based capital injection

- companies
  - initial funding: regional bank
  - product/service pricing:
    - companies sell products/services for arbitrary amounts (cost to make + profit margin)
      - workers paid via transfer from company pool to workers (not state pool)
      - given capital injections based on CompanyWorkerHours
        - incentivizes hiring more workers if capital allows
      - companies given monthly capital injection per-month based on number of worker hours
      - pros
        - allows natural expansion of production
        - allows repayment of state loans
        - gives a sense of worker control...good for morale
        - allows weathering lulls in demand
        - more decentralized/market-based
        - incentivizes productivity (nobody wants to pay for dead weight workers)
        - cost of things much easier to determine (company sets price, is expected to manage their own accounting)
        - more natural transition from capitalism
      - cons:
        - products can cost more than their value
        - boom and bust cycles based on greed or poor planning
        - companies hoarde "capital" (although, see profits-at-rest taxing below)
        - companies can buy "bottle caps" and distribute to workers as currency
          - profit distribution workarounds
          - promotes secondary markets
        - workers can "clock out" even when working...exploitation
          - "if you all don't clock half time, the company will fail"
      - notes:
        - does not incentivize overworking, because everyone is paid hourly
      - is profit transferrable to workers via dividends (based on work-hours/productivity)?
        - no. rationale:
          - people should be paid in labor-hours, not "market value" of their labor or arbitrage
          - would create wage inequalities not democratically decided
          - decentivizes reinvestment
          - incentivizes useless things like arbitrage and resource hoarding
        - yes. rationale:
          - better performing enterprises (worker incentives)
          - profitability attracts good workers, creates positive feedback loops
            - better enterprises "win", overall benefit to society
      - profits-at-rest taxed N% yearly (taken monthly)
        - tax rates set per-region, per-industry
        - decentivizes hoarding, incentivizes reinvestment
  - performance:
    - determined by profits (ie, no need to involve planning)


