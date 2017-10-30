jslife/osc

This is an oscillator collection for Conway's Game of Life, edited by Jason 
Summers.

Web site: <http://entropymine.com/jason/life/>


This is intended to be a "unified" oscillator collection, containing most every 
interesting oscillator that is known for each period. It is not as carefully 
compiled as most other collections, and quantity is often emphasized over 
quality.

An oscillator is any pattern that repeats exactly (in the same position and 
orientation) after a certain number of generations (its "period"), and not 
before then.

Each oscillator period is kept in a separate file or files. A period is 
sometimes split up into multiple files for space reasons. True-period guns are 
always put in separate "...-gun.lif" files if they are <= p62 (period 62), or I 
consider them to be "fundamental" (which sort of means: not based on technology 
that works at a lower period). Thus, the "...-gun.lif" files form their own 
sub-collection.

No oscillators are known for the following periods: 19, 23, 31, 34, 
38, 41, 43, 53. (To be pedantic, trivial p34 oscillators can be made by 
placing a p17 oscillator next to a p2 oscillator, but I'm not counting 
that.) All other periods either appear in this collection, or can be 
constructed using "herschel loop" methods similar to those used in the p61 
oscillator.

Many of the patterns and arrangements in this collection were copied, with 
permission, from Dean Hickerson's "Stamp Collection" and "New billiard 
tables" collections. Patterns have also been copied from other collections,
such as David Bell's and Alan Hensel's, as well as my own.

I am sorry that this collection is mostly not annotated as to who 
discovered what. The problem is that keeping track of this would require 
several times more time than it took to make the collection in the first 
place. It doesn't help that the usual Life file formats and/or 
applications do not have any good, widely-supported, way to annotate files 
containing multiple Life objects. Maybe the situation will improve in 
future editions -- or not. In the meantime, you'll have to look through 
other collections to try to find out where an oscillator came from. I've 
included several extraneous files (x-osc-new*), that used to be in my main 
pattern collection, as a reference for who discovered some of the newer 
oscillators.

I am not very happy with the low-period collections (roughly period 1 through 
6). A very large number of such oscillators have been found, or can easily be 
found with computer search programs. Plus, there are a myriad variations and 
ways to connect them together. This makes for a difficult editing task to decide 
which oscillators make the cut.


What types of oscillators are included here
-------------------------------------------

Up to about p60, almost everything is included. For higher periods, I'm much 
more selective.

Usually included:
- typical oscillators
- billiard tables
- sparkers
- stabilized wicks
- true period glider guns (guns can easily be turned into oscillators)
- "trivial" spark combination oscillators, if they are smaller than, or
   about the same size as, the smallest "interesting" oscillators for
   that period
- Some patterns that otherwise might not qualify have been "grandfathered
   in" because they appear in other, older, collections.

Usually not included:
- "uninteresting" low-period (<=6) oscillators. Actually, there are a lot of 
   such oscillators included right now, but it's too easy to find new ones.
- anything that is not an oscillator (or gun): A period-N oscillator must recur
   exactly, at period N, and at no period prior to N.
- "mixed" period oscillators, in which no cell's period equals the full period
   (with rare exceptions)
- (for periods over about 60) most herschel loop-based oscillators
- (for high periods) most "artificial" oscillators: glider relays and loops,
   thinned-out glider guns, various period multiplication techniques, etc.
- wicks and near-oscillators that have been stabilized using glider guns

Things I'm undecided about:
- artificial guns, up to maybe p500 or so (e.g. p150 guns based on p30 technology)
- low-period billiard tables that aren't particularly interesting
- extremely large oscillators
- "trivial" spark combination oscillators for moderately large periods
- various types of glider relays
- 90-degree glider reflection reactions (these can always be turned into
   an oscillator, but for some periods there are a large number of them)
