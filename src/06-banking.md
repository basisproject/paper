# Chapter 6: Banking

Banking acts as the barrier between the member companies and the outside market by limiting their exposure to handling capital. While the bank still engages in loans, the loans are structured somewhat differently than they are traditionally.

The bank is also the region's investment vehicle. It has a democratically-set investment plan that matches the region's needs. For instance, a region might want more green energy, manufacturing, or agriculture. In addition to setting the bank allocations preferences, the region will have to budget things like asset acquisition (housing, office space, etc), a basic income for members, etc.

## Loans

When a member company interacts with the outside market, it necessarily needs capital to start. The regional bank can supply this via a loan, and the application process would look much like a traditional loan might: showing a business plan and market viability. However, after the company's loan is approved, things look different.

Rather than the bank giving the company the money and letting them do what they will (while charging interest), the bank will rather *keep* the money and authorize the company to spend up to the loan amount on the bank's behalf (without charging interest). All purchases made into the outside market will come from this account, and any incoming orders, whether from the market or from other member companies, will replenish the account.

The real difference here is the bank is sectioning off a portion of its capital (which is owned by all regional members) such that the company is allowed to spend out of in order to provide their products and services.

## Currency tracking

Here's where things get more interesting. As mentioned when talking about the loans, if a company gets an incoming order, that order replenishes the capital account the company spends from. This is easy to imagine if selling into the market (the revenue from the sale goes back into the account), but what about if another member company orders the products? Since no money changes hands, how is the account replenished?

The answer is with currency tracking. Just as products and services can track labor content and resource content, they can also track currency content. So when a member company orders a product that requires spending of capital, that order replenishes the producing company's capital account, but the products that were ordered now have that currency value tracked with them.

Let's say a widget company buys steel for their widgets from the market. They can make ten widgets for every $5 they spend on market steel, and each widget takes 15 minutes to make. So we know the cost of each widget is 0.25L + $0.50. When a member company orders a widget, they don't just get the costs of the labor (0.25L)...they also take on the currency cost. So the amount of currency imbued in each product moves through the economy just like the cost of labor does.

Eventually, either the product is consumed by a member, or sold back into the market.

If the product is consumed by a member, the final cost of the product is:

```
labor + resources + (currency * (regional_active_credits / regional_active_capital))
```

Where `regional_active_credits` is how much credits are in current circulation for a region and `regional_active_capital` is how much capital the region has in its credit budget. This converts the currency value into a labor credit value, so the member can use their labor credits to purchase it.

## Market pricing

So what happens when a market company *does* buy a member company's product? How much do we sell it for? Given that we've tracked the currency value of the product at its inception, using the following equation we know the at-cost market price:

```
(labor * (regional_active_capital / regional_active_credits)) + currency
```

What we're doing is converting the labor imbued in the product into a currency value, which we can then add to the tracked value of the currency in the product.

So if we wanted to make no profit off the sale, that's our price. But we *do* want to make profit. In fact, we want to make as much profit as possible off of any non-member interaction! This goes back to our principle of duality. So we have two options:

1. sell at a flat profit rate: `cost * (1 + profit_rate)`
1. sell at a price that undercuts the current market value: `market_rate * (1 - undercut_rate)`. 

For the first, we have to be careful we don't exceed the market value of the product (or it won't sell at all), and for the second we have to be careful we don't sell for less than the cost to make the product. If we know the market price of the product, we can automate this to a large extent. However, we cannot assume we have market price information for every product.

Really, it makes sense for the time being to allow companies to set the market price of their own products, while giving them a hard pricing floor (at-cost).

## Internal commerce

Given the point of the network is to grow over time, it makes sense to incentivise member companies to order from each other.

Outline:

- Count internal orders at a 1.0 order value and market orders as a 0.8 order value

## Incentivising market profit

Because member companies don't actually *get* the profit from market sales, some level of incentive is needed to encourage them to sell at rates as high above at-cost as possible.

Outline:

- UBI
- The higher the profit, the closer the order value goes from 0.8 to 1.0

## Inter-region currency tracking

So what happens if Jerry's Widgets in the San Francisco region orders steel from Sandra's Steel Mfg in Oakland and the steel has currency costs in it? Ultimately, the San Francisco region would have to send the currency value from their regional bank to the Oakland regional bank to cover the cost.

Essentially, one regional bank would be buying the currency-value of the products at-cost from the other regional bank *on behalf of the member companies*.

Given that the banks would be on the same federated network, the transfer happens in the same transaction that finalizes the order between the two companies.

## Investment plan

TODO: Outline:

- Loan allocation plan
  - Sector percentage (10% energy, 20% agriculture, 5% manufacturing, etc)
  - Risk allocation (50% stable, 20% risky, 5% venture, etc)
- Asset acquisition
- Basic income

