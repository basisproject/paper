# Chapter 5: Banking

Banking acts as the membrane between the network and the capitalist markets. It provides seamless transactions for member companies into and out of the market, tracking currency costs for purchases and evaporating internal costs for sales into the market.

Banking also allows simple conversion of credits to local currency so individual members can purchase things from the market that are not available in-network.

## Investment

TODO:

- [#124 - Investment system](https://github.com/basisproject/tracker/issues/124)

## Currency tracking

When a company needs inputs to production that are not produced in-network, they need to interact with the market. The banking system allows a company to spend out of a [capital pool](#capital-pools) to buy these inputs. When this purchase happens, the cost of that purchase is added to the company's costs in the form of tracked currency.

So just like we track labor and resources as separate costs, we also track various currencies. Then that company can assign those currency costs to its products just like it would any other costs. When another company orders one of those products, the capital pool of the ordering company transfers the total currency cost of the order to the capital pool of the producing company. This way, currency costs are repaid immediately as they flow from company to company (or rather, capital pool to capital pool).

When a product is finally either consumed or sold into the market, the currency cost will be factored into the total cost in credits of the product. If the product is consumed by a member, the currency is converted into a credit value using the network's credit-to-currency conversion rate. For more information, see the section on [currency conversion](#currency-conversion). If the product is sold into the market, the optimal target price will include the currency cost imbued in the product (along with whatever other internal costs there are, converted into a currency value).

## Cashing out

Members of the system earn credits for their labor. However, in the beginning the network might be very small and it will be impossible for members to meet even basic needs internally. They might have to turn to the capitalist markets for purchases.

Basis defines a process for a one-way conversion between the internal credit currency and external market currencies. The one-way distinction is important: we only ever want to create credits when labor is completed. This eliminates many forms of speculation and manipulation. When credits are converted to outside currency, they are burned (as they are with purchases).

The conversion rate between internal credits and external currency is defined in the [currency conversion section.](#currency-conversion)

## Currency conversion

The project defines a method to peg the internal credit currency to an external market currency (likely USD in the beginning). The ultimate goal would be that 1₡ = $1.

TODO:

- [#88 - Details on the dollar peg](https://github.com/basisproject/tracker/issues/88)

