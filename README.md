# Overview

Helper tool to support Fantasy F1 pool.

# Goals

- [x] Calculate scores for:
    - [x] Race
    - [x] P10
    - [x] Sprint
    - [x] Tabular output for easy copy & pasting
    - [x] Consistent ordering for easy copy & pasting

- [ ] Send out reminders for P10 guesses
    - Especially important for weird timed races and sprint weekends (China >.>)
    - options: 
        - remind me - generate calendar?
        - remind others through discord, need to support multiple servers channels
        - email?
        - other?
        
- [ ] Remind me to make/send results
    - Low priority, as it gets easier lower barrier to entry and I watch races so already know when

- [ ] Output to DB or other form for long term storage in easily readable form
    - Current: copy & paste to existing Google sheet (Shift + Alt for Columnar copying!)
    - options: Google Sheets, DB, file

- [ ] Complete team tracking and scoring
    - [ ] Leaderboard
        - [ ] Grouping of teams (eg work/friends)
    - [ ] Trade support
    - [ ] Trigger to lookup results

- [ ] Send out scores after suitable spoiler free window
    - options: ???

# Rules

These are the rules this is made to support.

## How it works:	
This is designed to be a pool where all drivers matter, it doesn't require a lot of weekly work, and makes the race interesting even if there is an obvious leader.	

You will create a team by picking drivers in predefined categories. There are 3 categories: top, mid, and bottom, defined by Constructors scores from last year.	

Each race week you will also pick one driver that you think will come P10. You can submit a default P10 guess for the year, in case you don't get your guess in on time.	

P10 guesses and trades (see below) must be submitted before qualifying (or Sprint Qualifying) for a given GP.	

I try to put out reminders for P10 guesses for a given race weekend.	

I will try to post leaderboard updates after each race weekend.	
	
## Scoring	
Race scoring is determined by race result and which catagory a driver is in.  	

Base score is linear from P1 to P15 with P1 being worth 15 points and P15 being worth 1. Sprint race scoring will work the same way but only up to P12.	

On top of the base score the categories act as a multiplier. Top is 1x, Mid is 2x, and bottom is 3x.	
Rookie drivers, noted by a (R) also receive a 2x multipler.	

P10 scoring works as the difference from P10, so P10 is worth 10 points, P9 and P11 are worth 9 points. This is the multiplied by 5, to keep it on a similar order of magnitude as the race scoring. P10 guesses for Sprints will use your P10 guess for the GP.	

For both race and P10 scoring a DNF is worth zero points. In the last few years there have never been enough DNFs to affect the race scoring, but it can affect the P10 scoring. 	
	
### Scoring examples: 	
Norris finishes 1st: gets 15 points for position, 1x multiplier for being in top category, and no rookie bonus, for a total of 15 points

Lawson finishes 2nd: gets 14 points for position, 1x multiplier for being in top category, and 2x rookie bonus, for a total of 28 points

Alonso finishes 6th: gets 10 points for position, 2x multiplier for being in mid category, and no rookie bonus, for a total of 20 points

Bortoleto finishes 10th: gets 6 points for position, 3x multiplier for being in mid category, and 2x rookie bonus, for a total of 36 points

	
## Trades: 	
This year, I am experimenting with trades.  	

Each person will get 1 season trade and 2 GP trades.

A season trade is a swap for the rest of the season. You will keep the score the departing driver has accumulated thus far and get the score from the new driver from then on.	

A GP trade is a swap for a single GP weekend (including sprint races).	

Trades must be made in the same catagory, you can swap between rookie and non-rookie.	

P10 guesses and trades must be submitted before qualifying for a give GP.	

In the case that a driver on your team is no longer on the grid, you will get a free season trade for that driver.	

In the case that a driver on your team will not be competing for a given weekend, you will get a free GP trade for that driver. Because trades must be submitted before qualifying, this only triggers if they are announced as not competing prior to qualifying, ie crashing during qualifying does not get you a free GP trade.

	
## Constructors Placement (optional)	
Constructors championship gets shaken up each year, so lets also guess where each constructor will finish.	

This will be a separate score and is more for looking back and seeing how you thought the year was going to shake out before anything started.	

Scoring is calculated the same as P10 scoring, the difference from the estimated finishing position, eg if you guess Ferrari P1 and they get P3 you get 8 points.  	
	
# Got it, what do I actually do?
Send me the following

## Team picking: 	(R) = 2x Rookie bonus

### Top 1x - **Pick 2**
| Driver        | Team     | Pick (mark with X) |
| ------------- | -------- | ------------------ |
| Piastri       | McLaren  |                    |
| Norris        | McLaren  |                    |
| Leclerc       | Ferrari  |                    |
| Hamilton      | Ferrari  |                    |
| Verstappen    | Red Bull |                    |
| Lawson (R)    | Red Bull |                    |
| Russell       | Mercedes |                    |
| Antonelli (R) | Mercedes |                    |
	
### Mid 2x - **Pick 2**	
| Driver      | Team         | Pick (mark with X) |
| ----------- | ------------ | ------------------ |
| Stroll      | Aston Martin |                    |
| Alonso      | Aston Martin |                    |
| Gasly       | Alpine       |                    |
| Doohan (R)  | Alpine       |                    |
| Hadjar (R)  | RB           |                    |
| Tsunoda     | RB           |                    |
| Ocon        | Haas         |                    |
| Bearman (R) | Haas         |                    |
	
### Bottom 3x - **Pick 1** 	
| Driver        | Team     | Pick (mark with X) |
| ------------- | -------- | ------------------ |
| Albon         | Williams |                    |
| Sainz         | Williams |                    |
| Hulkenberg    | Sauber   |                    |
| Bortoleto (R) | Sauber   |                    |
	
## Default P10 Guess (optional)	

If I forgot to send you a P10 guess before Qualifying start, assume I want: __________
	
## Constructors Championship Guess (optional)
| Position | Constructor |
| -------- | ----------- |
| 1        |             |
| 2        |             |
| 3        |             |
| 4        |             |
| 5        |             |
| 6        |             |
| 7        |             |
| 8        |             |
| 9        |             |
| 10       |             |
