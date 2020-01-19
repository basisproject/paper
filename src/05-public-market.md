# Chapter 5: Public market

The public market is where companies order from other companies, but also where consumers order goods as well. The public market is both a service that lists available products and services, but also facilitates the orders between companies.

Non-member companies are free to use the public market for buying or selling, but may have to pay a monthly service fee and/or per-transaction fee. Non-member users can use the public market for free.

## Offerings and pricing

Products and services that are made available solely in the Basis economy will only need to list the products and the pricing of them will be automated. However, if a member (or non-member) company wishes to sell their product for non-member companies to purchase on the open market, products will need offerings. In the simplest case, this can just be a flat price. However, multiple offerings can be attached to a product, each with different criteria. Perhaps the price drops 10% on holidays. Maybe a certain buyer gets a 5% discount.

Offerings are the interface between Basis products and the market system at large.

For member companies, automated offerings can convert the labor content of a product into a market price (and add a percentage markup for profit).

## Interface

The public market's web and mobile interfaces allow user access and a public API allowing for automatic operations (such as automated purchasing or updating stock contents).

## Anonymity

While the transactions between member companies are transparent and freely observable by any member of any region, there are a few cases where privacy is offered:

- Consumer purchases. Whether a member or a non-member, purchasing goods from a member company are anonymous. The order is still be viewable by members, but looks like it came from the Basis system directly and is not tied back to the user in any way. Only the user and the company have the full information on the order.
- Non-member company orders. Any time a non-member company is involved in an order, the order details will be available, but the non-member company will be anonymized. This is to protect the privacy of companies who do not wish for their purchasing to be transparent. Like consumer purchases, the order will look like it came from the Basis system directly, and the non-member company's identity will be anonymized.

The specifics of the anonymity algorithm are not yet decided. Please see <https://gitlab.com/basisproject/tracker/issues/4>.

## Point of sale

Point of sale integration allows companies in physical locations to create orders directly from scanned items and take payment from customers.

