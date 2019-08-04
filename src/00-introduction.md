---
title: "Factor: a blockchain implementation of a socialist mode of production"
subtitle: "Or, how to save the world by getting rid of markets, capital, and profits"
author:
  - "Andrew M. Lyon"
keywords: "distributed, socialism, anarchism, economics, software, blockchain"
---

# Introduction

Capitalism is destroying us. It relies on markets to price things, but it is terrible at setting prices *to their actual cost*. If you buy a phone, the phone has a price, and that price reflects the market value of the phone (which in a perfectly competitive system is the cost of materials + labor + a slim margin of profit). However, if the phone was built in a factory using caustic chemicals that should be disposed of properly (but are likely dumped in a river), that's not included in the price. If the phone is transported across the globe, the carbon being blasted into the atmosphere by the trucks and boats shipping it is not included in the price. When I throw the phone away six months after buying it to get a slightly updated model, the phone sits in a landfill, and the cost to actually deconstruct and recycle the parts it contains *is not included in the price*.

Markets are very good at pricing things based on desire, but are incapable of pricing things based on ecology. On top of this, moneyed interests spend large sums of their profits convincing governments to *not* force them to price in the externalities of their products through taxation and regulation. The end result is we're poisoning ourselves in the long term to give those in the short term more wealth than anyone could ever want.

On top of this, capitalism doesn't just commodify products and services. It also commodifies things like debt as well. But why stop there? It builds entire financial empires on debt and uses it as a printing press to make more wealth. It commodifies our emotions and personal connections through social media. It commodifies our free time with pocket distraction devices that have been successfully engineered to steal our attention every few moments. It commodifies our housing: the more productivity increases, *the more the market value of housing goes up*! How convenient for the mortgage lenders, banks, and landlords.

While all this is happening, wealth is becoming more and more concentrated in the hands of the few. We're told that wealth is not zero-sum, but in fact it is if you're not already swimming in wealth. Band-aid fixes are proposed: basic income, wealth tax, inheritance tax. Yet the system perpetuates, and the wealthy feed off the productive like a tick that has become larger than its host.

So what's the alternative? Central planning? Stalinism? Maoism? Gulags? Famine? People will tell you there are no real alternatives to capitalism. Markets are the only way to produce things, and the means of production cannot *possibly* be socialized!

Let's break down this notion of impossibility, though.

## The economics of socialism

What is an economy? It's a network. Each node is a person or organization. Each connection between any number of nodes is a transaction. That is all. It doesn't require money or capital, it doesn't necessitate social or private ownership of factories or warehouses. It's a simple network, and all it requires is the ability for two or more nodes to exchange. A blank slate!

Given this, let's start from scratch and discuss some possible ideas:

1. The system of exchange used to facilitate transactions
1. The system of ownership exercised over the productive instruments

## The system of exchange used to facilitate transactions

Our real problem here is supply and demand. A market is essentially a distributed algorithm which finds a value that solves for the current supply and demand of a particular commodity. Price is that value. In essence, given supply and demand, find price. This is an oversimplification, but for our purposes this is fine.

Given the above, what if given demand and price, we solve for supply? The solution is a bit more temporal...supply must be adjusted over time. But then again, if you think of supply not as "the current supply" but instead "the rate at which supply is replenished" (and similarly with demand) then supply *can* be adjusted almost as instantaneously as price. This can also be done in a distributed way. How?

By measuring orders. Orders are the precursor to the transaction. They are a marker of economic intent. "I want this." If you have a system where orders for widgets (and other products) are publicly available, it's would be very easy to measure how many orders for widgets there are and how backlogged those orders are in aggregate. If there is a backlog of two weeks, perhaps then it makes sense to open a new widget factory.

So in this system of transparency, orders become the signals of demand. In fact, demand becomes much easier to measure, and given demand, and a system of fixed (or at least not rapidly fluctuating) pricing, supply can be adjusted to meet demand. Thus, we do not need a market system of arbitrary prices to facilitate a system of primary production (that said, arbitrary prices may still come in handy when two individuals are transacting between each other outside of this transparent economic system and both supply and demand are fixed).

Why bother with this, though? Because if you have consensus on setting prices in terms of *costs to society* (labor, ecology), then the easiest way to put downward pressure on prices is to raise productivity, as opposed to paying off government officials to relax environmental restrictions on your industry. In other words, if you account for the *actual costs* to build something into its price, then you make it more expensive to buy things that are damaging or difficult to dispose of.

Because this method is still decentralized, you can also sidestep dreadful things like central planning. The planning process would still be necessary, but be limited to scoring different raw materials and compounds based on their abundance, pollution potential, and recyclability (as opposed to controlling all production inputs and outputs). In the end, prices would be based on cost of raw material plus the labor hours required to produce something.

## The system of ownership exercised over the productive instruments

Capitalism is a market system with privatized ownership of the means of production. Socialism necessitates the commonly-owned means of production.

How is this ownership exercised? What things are owned, in particular? Is my toothbrush really mine? If I lend my hammer to a friend, and she uses it at work, is my hammer then socialized?

Let's talk about two things: effectiveness and intent.

Effectiveness in this case simply means "is it effective to socialize this?" Would socializing a toothbrush make sense? Would it make sense to track who is using it, schedule its repair and maintenance, arrange for its storage when not in use? By socializing something, there is necessarily some level of process and overhead it goes through...a cost to society. On top of this, if we were to socialize toothbrushes, and I order 100 of them for my toilet-bowl-cleaning-using-only-toothbrushes company, I might realize that there are only *soft* bristled toothbrushes available. I want *firm* bristles, or my fellow workers and I will have to scrub *twice as hard*. Do we socialize every form of toothbrush now? Or does it maybe make sense for our company to simply order the toothbrushes it needs, and for those toothbrushes be owned by our company, and accounted for in the costs of our services?

Now, contrast that with a factory, warehouse, or office building. Whatever overhead cost there is in administration is a fraction of its value to society. Productive land, factories, warehouses, office buildings, heavy machinery, airports, seaports...these are the things it is *effective* to socialize.

Secondly, intent. There are things that *can* be used in the productive process, and there are things that are *made* to be used in a productive capacity. Do people buy semi trucks for a hobby? Or are they almost exclusively used in production? What about factories? Now, what about our toothbrush from earlier? It's not made for a productive use, but rather personal. It *can* be used in a productive capacity, but it's not *made* for that.

The point being, there is a clear divide between things that are made for the productive process, and things that are more general. Socialization, at least for the purposes of this paper, concerns itself with those things that are *intended* solely for production.

Along the lines of intent, things do not automatically *become* socialized just because they were used in a productive capacity. Instead, there must be a clear intent and effort to socialize it in the first place.

## Why socialism seems difficult

The reason why capitalism seems a much better solution than any form of socialism is because capitalism is *simple*. The pricing mechanism is simple. The ownership mechanism is simple. Anyone can price anything however they want. Anyone can own just about anything they want. Socializing the means of production, as well as using a non-market pricing system, takes more effort. It raises more questions. It changes our relationships to things. However, the simplicity of capitalism is not worth the problems that come with it.

This paper will define a system that makes implementing and maintaining a socialist mode of production as simple as possible. Its goals are to use incentives, as opposed to coercion, to grow naturally and eat capitalism slowly over time, replacing it with a system that pays people the full value of their labor, distributes based on contribution, and accounts for the costs that capitalism externalizes. It will do this without sacrificing autonomy or individual choice.

## What is Factor?

Factor is a software implementation of the system described above. It facilitates orders and transactions between companies, allows for the managing of housing and means of production, facilitates democratic participation, and defines a currency that can be used by members.

It will be built on top of a blockchain system which will make it resilient against both tampering and corruption. The nature of this blockchain will also make all economic and civic operations transparent, while preserving privacy of individual purchases and votes.

Factor's goal, beyond providing economic structure, is to be so good at what it does that not using it isn't just inconvenient, it's expensive.

