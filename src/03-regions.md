# Chapter 3: Function of regions

Regions are the economic containers of regional socialism. They are managed by the people participating in the region, and define and automate various aspects of the economy.

Management of regional parameters and operations can be decided by direct democratic process, through elected representatives, or by various other methods. The exact process for governing regions won't be defined here, and will likely vary from region to region.

## Regional parameters

The regional parameters define various aspects for how the economy will work within the region. They will be defined in the Conductor system and managed either directly by the region's participants or by elected representatives.

All regional parameters will always be publicly available for review by any member of the region. The full list of regional parameters is as follows:

- Worker hourly wages (more on how this is defined in [Chapter 4](#wages-and-wage-calculations))
- Worker minimum average weekly hours to be considered members
- Wage reduction dampening rate/delay
- Productivity rate value range (eg, between 0.5 and 1.5, or 0.8 and 1.2, etc)
- Sales tax percentage (both in-region and extra-region sales tax)
- Income tax percentage
- Profits-at-rest taxation delay and percentage
- Regional bank investment allocation
- Healthcare subsidization (or in many cases, health insurance subsidization)
- Higher education subsidization
- Higher education wait-list prioritization
- Percentage of-cost to charge for housing (90% would be a 10% subsidy, 110% would be a 10% apartment tax)
- Threshold of housing acquisition (such that new housing is acquired when the housing wait-list is N% of the total population)
- Board of directors position appointments for various regionally-owned companies
- Monthly social dividend percentage

## Governance

Since each region will be set up as a co-op, some form of governance needs to be defined. Although the goal is to have many decisions made by democratic vote, there will still need to be high-level leadership to manage certain things such as region splitting, continued development of the software driving the region's functions, oversight of regionally-owned companies, and overall maintenance of the system.

The governing board of the regional co-op corporation will be known as the High Council. The members of the region will democratically elect two-thirds of this board, with the other third are elected by direct employees of the co-op.

The High Council will select the overall leadership and will be ultimately responsible for the oversight of all things that are not democratically decided by the members. The High Council may also guide development of the overall system to have more democratic decisions, if the members wish.

## Duties of the region

The mission of the regional co-op company will be to:

- Coordinate and mediate between the regionally-owned companies
- Develop and maintain the software systems used to power the region's economy (Conductor)
- Provide education and guidance to new companies
- Facilitate elections and hold meetings with members of the region
- Foster a sense of community between all the members in the region
- Facilitate regional splitting
- Coordinate and cooperate with other regions on issues relating to housing and services

## Membership

In order to be a member of the region, you must be a worker for a company that has regional membership, and you must work at least an average of N hours a week (N being regionally decided, of course). It's important to note that this value should take into account time off (vacation, sick days, etc).

It's important to note that this takes into account total hours worked. In other words, if the minimum is 25 hours a week, then someone who works 25 hours a week is a member even if they split their hours between two or more regional companies via part-time employment.

WIP: need a membership grace period for unemployment (quit, fired, injured, etc).

### Regionally-owned companies

The region will operate a small amount of various types of companies. The main candidates for regional operation are:

- A central bank, which invests in companies based on regionally-desired allocations
- A housing company, tasked with acquisition and maintenance of various forms of housing
- Hospitals
- Colleges and trade schools (and K-12 if there are no public options)

The idea is that the region will trend toward operating companies where parasitic profit-seeking is detrimental to society as a whole, focusing on regional welfare instead of amassing of capital. This serves two purposes: to allow members to adjust their own cost of living and to attract outsiders to participate.

All accounting for regionally-owned companies happens through Conductor, allowing detailed information on costs of running the company. This makes it easy to offer services at-cost (or some percentage of cost, with the region subsidizing the difference). It also allows ultimate transparency: any regional member can view the accounting of any regionally-owned company.

Regionally-owned companies are not necessarily restricted to servicing only regional members. For instance, if a region has excess space in an apartment complex, it may rent out the rooms to non-regional members at market rate (instead of at-cost). The regional college may enroll students who are not members and charge them much higher tuition.

Each regionally-owned company will have an elected board of directors governing it. The boards of each individual company will be democratically selected as follows:

- All members of the region will select one-third of the board
- The workers of the regionally-owned company will elect another third of their board
- The High Council will elect the last third of the board

Seats will be filled in the above order. In other words, if there are seven seats, the 7th member would be selected by the regional members. If there are are eight seats, the 8th seat will be selected by the workers of the regional company.

With this setup, regionally-owned companies will be accountable to members of the region, to the workers in the company, and to the leadership of the region.

## Board details

Board elections for all boards (including the High Council) will be held every three years.

Board members will be paid a regionally-decided hourly wage when they are meeting, and will have all board-related expenses reimbursed (for travel or lodging, if required) through the company they govern. Boards will handle the appointment of company leadership and will oversee that the company faithfully executes the will of the region.

The president of each company will sit on the board of the company as a non-voting, advisory member.

Boards will self-govern using consensus such that:

- The board meets *at least* once every quarter.
- Any director can propose an idea or change (ideally advance notice is given before meetings).
- Proposals are discussed, along with their pros and cons, and updated based on feedback.
- Proposals are updated and modified until everyone reaches an agreement. If agreement cannot be reached, the proposal returns to discussion.
- Even if just one director dissents, the proposal cannot move forward. Ultimately, after full discussion, disagreement must satisfy one of the following conditions:
  - The proposal will materially harm the company in the short or long term.
  - The proposal is in material opposition to the goals or spirit of the company.
  - The proposal is in material opposition to the wishes of the region/community.
  - The proposal is morally or legally unjustifiable.
- If a proposal is seen as immediately essential for a company's continued operation, then a vote can be called and if over 75% agree to the proposal, it is accepted even if full consensus is not reached.

This consensus mechanism, while intricate and lengthy, makes sure that various members of the board are able to have their expertise shared and acted on. This is in contrast to majority rule, which tends to silence certain important inputs to discussion.

Board members should be selected using a certain set of criteria, which will vary depending on the type of company.

## Capital, banking, and transparency

Regional taxation (income, sales, profits-at-rest) all generates regional income and is put into the regional bank. The regional bank allocates funds based on a regionally-decided set of allocations. See the [chapter on Banking](#chapter-5-banking-and-investment) for more information.

The regional bank is operated under the principles of transparency, which allows making highly informed decisions on just how much the region should subsidize or tax various operations. For instance, if the members of a region know it costs $4M a month to operate the regional hospital, they know it will cost $3M a month out of the region's capital pool to subsidize 75% of healthcare.

By default, the wages of all workers are stored in the public bank. Workers are allowed to withdraw their funds at any time, and can even set up recurring transfers to send their money to another bank of their choice (which would essentially be direct deposit at any other company).

## Federation

While many parameters of the economy are regionally-defined, regions are able to engage in commerce and share much of their information via federation. For instance job postings, wages, purchasing of products and services, education, and housing will all be advertised on the federated network shared by connected regions.

This allows companies and people in various regions to engage in commerce with each other without barriers. It also allows someone to find a region that has the highest wages for work they might be interested in, and to see if in-region housing is available there.

Federation of regions allows transparency and freedom of movement.

### Coordination with other regions

Let's say someone lives in the Saint Paul region, but they get a new job in the Minneapolis region. In other words, they are not truly a member of the Saint Paul region because they no longer work in that region. Should they lose access to their housing? The answer is nuanced.

If the Saint Paul region subsidizes housing by 50% and the Minneapolis region does not subsidize at all, then the worker is getting all the benefits of a low-tax job with the benefits of subsidized housing. While this situation is most likely extreme, it's not outside the realm of possibility. How is it solved?

The answer is for regions to coordinate with each other. They might come to a deal where a worker in the Minneapolis region can get housing in Saint Paul, but only at a 5% subsidy instead of a 50%.

The Conductor system would allow setting rules like this (if they live in X region but work in Y region, housing costs Z). This would be transparent to both the worker (who would see the different cost of housing when they applied for the new job) and to the Saint Paul housing company who will be able to see the full list of housed members and at what cost their rent is.

The premise is, membership is a function of where someone works, but regions can coordinate between themselves to offer services to members of other regions if there is a need to do so.

