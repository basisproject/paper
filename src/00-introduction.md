---
title: "Basis: a distributed-ledger implementation of a socialist mode of production"
subtitle: "Or: How I learned to stop worrying and produce without profits and private property<br><br>**(Last update: Nov 22, 2020)**"
author:
  - "Andrew M. Lyon"
keywords: "distributed, socialism, communism, economics, software, blockchain, dlt, cybernetics"
---

# Introduction

Let's start with a premise: we can make all the things we want and need without bowing to profit as our king and without ceding control of our productive instruments to private owners (and we can do these things without an authoritarian government).

If socialism and capitalism are languages, Basis is a translator that speaks both and allows members to exit the relations of capitalism without being subject to a life of puritanical simplicity.

Basis defines a distributed network of co-ops that produce things for use (as opposed to for-profit). This network eats capitalism over time, business by business, property by property, becoming more and more capable of producing the needs of its members internally.

Often socialists are asked "Why not just start a commune?" This is our commune. It has banking, it has housing, it has farm land, factories, and warehouses, and it has a militant business plan.

## The economics of socialism

What is an economy? It's a network. Each node is a person or organization. Each connection between any number of nodes is a transaction. That is all. It doesn't require money or capital, it doesn't necessitate social or private ownership of factories or warehouses. It's a simple network, and all it requires is the ability for two or more nodes to exchange. A blank slate!

Given this, let's start from scratch and discuss some possible ideas:

1. The system of exchange used to facilitate transactions
1. The system of ownership exercised over the productive instruments

## The system of exchange used to facilitate transactions

The biggest question here is supply and demand. Many believe that socialism operates outside the bounds of supply and demand, but this is ridiculous: why would anybody make things that there is no need (demand) for? A market is essentially a distributed algorithm which finds a value that solves for the current supply and demand of a particular commodity. Price is that value. In essence, given supply and demand, find price. This is an oversimplification, but for our purposes this is fine.

What if instead of given supply and demand and finding a price, we start with demand and cost and we solve for supply? The solution is a bit more temporal...supply must be adjusted over time. But then again, if you think of supply not as "the current supply" but instead "the rate at which supply is replenished" (and similarly with demand) then supply *can* be adjusted almost as instantaneously as price. This can also be done in a distributed way. How?

By measuring orders. Orders are the precursor to the transaction. They are a marker of economic intent. "I want this." If you have a system where orders for widgets (and other products) are publicly available, it would be very easy to measure how many orders for widgets there are and how backlogged those orders are in aggregate. If there is a backlog of two months, perhaps then it makes sense to open a new widget factory.

So in this system of transparency, orders become the signals of demand. In fact, demand becomes much easier to measure, and given demand, and a system of fixed (or at least not rapidly fluctuating) pricing, supply can be adjusted to meet demand. Thus, we do not need a market system of arbitrary prices to facilitate a system of primary production.

Why bother with this, though? Because if you have consensus on setting "price" in terms of *costs to society* (labor, resources, externalities), then it becomes much easier to measure the actual costs of things. Prices abstract and obfuscate the costs. If a TV costs $100, I don't know how much of that paid laborers, how much went into machine maintenance, how much into regulatory compliance, how much in profit, marketing, advertising, and so on and so forth. Not only this, but if you can measure all the inputs to production back to the raw materials, you can know which products used more fossil fuels. Which products used chemicals that are known pollutants. Suddenly a TV isn't $100 anymore, but a few grams of silicone, 32 labor hours, 800 miles in shipping, 1.5 kilos of CO2, etc. How would our purchases change if we knew exactly what went into our products, and what byproducts they had? Whether we like it or not, the things we do affect those around us and it's time to stop pretending they don't.

This method of measuring orders via a transparent network requires no central bureaucracy, meaning you can sidestep things like a command economy (although Basis does not preclude economic planning). Production would be measured based not on some obscured market process, but the actual costs to society.

In this sense Basis is neither a capitalist market nor a centrally-planned system: it lies somewhere in-between. It allows for distributed production like a market, but without the pricing and relations of a market. It's what a market looks like if you take away money, profit, and private ownership.

## The system of ownership exercised over the productive instruments

Capitalism is a market system with privatized ownership of the means of production. Socialism necessitates the commonly-owned means of production.

How is this common ownership exercised? What things are owned, in particular? Is my toothbrush really mine? If I lend my hammer to a friend, and she uses it at work, is my hammer then socialized?

Let's talk about two things: *effectiveness* and *intent*.

Effectiveness in this case simply means "is it effective to socialize this?" Would socializing a toothbrush make sense? Would it make sense to track who is using it, schedule its repair and maintenance, arrange for its storage when not in use? By socializing something, there is necessarily some level of process and overhead it goes through...a cost to society. On top of this, if we were to socialize toothbrushes, and I order 100 of them for my toilet-bowl-cleaning-using-only-toothbrushes company, I might realize that there are only *soft* bristled toothbrushes available. I want *firm* bristles, or my fellow workers and I will have to scrub *twice as hard*. Do we socialize every form of toothbrush now? Or does it maybe make sense for our company to simply order the toothbrushes it needs, and for those toothbrushes be owned by our company, and accounted for in the costs of our services?

Now, contrast that with a factory, warehouse, or office building. Whatever overhead cost there is in administration is a fraction of its value to society. Productive land, factories, warehouses, office buildings, heavy machinery, airports, seaports...these are the things that are *effective* to socialize.

Secondly, intent. There are things that *can* be used in the productive process, and there are things that are *made* to be used in a productive capacity. Do people buy semi trucks for a hobby? Or are they almost exclusively used in production? What about factories? Now, what about our toothbrush from earlier? It's not made for a productive use, but rather personal. It *can* be used in a productive capacity, but it's not *made* for that.

The point being, there is a clear divide between things that are made for the productive process, and things that are more general. Socialization, at least for the purposes of this system, concerns itself with those things that are *intended* mainly for productive use.

Along the lines of intent, things do not automatically *become* socialized just because they were used in a productive capacity. Instead, there must be a clear intent and effort to socialize it in the first place.

## Why socialism seems difficult

The reason why capitalism often seems a much better solution than any form of socialism is because capitalism appears *simple*. The pricing mechanism is simple. The ownership mechanism is simple. Anyone can price anything however they want. Anyone can own just about anything they want. Socializing the means of production, as well as employing profitless production, takes more effort. It raises more questions. It changes our relationships to things. However, the simplicity surrounding capitalism is precisely because of the one thing capitalism does best: externalize. Capitalism *appears* simple, but only because the complexity required to manage it is forced onto communities, governments, and the environment.

This paper will define a system that makes implementing and maintaining a socialist mode of production as simple as possible. Its goals are to use incentives, as opposed to coercion or force, to naturally grow a system that pays people the full value of their labor, distributes based on contribution, allows unrestricted access to the means of production, and accounts for the costs that capitalism externalizes. It will organize production around social usefulness instead of individual profit, and do so without sacrificing autonomy or individual choice.

