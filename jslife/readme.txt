jslife
Version 2012.12.30

This is Jason's collection of patterns for Conway's Game of Life.

The files contained in this archive are in a format called "RLE", which is 
widely used for Life patterns. Most Life programs should recognize it.

Comments and questions may be sent to Jason Summers <jason1@pobox.com>.
<http://entropymine.com/jason/life/>

-----

The collection is subdivided into several separate directories (folders):

applications
   (1) Patterns whose population over time is unusual.
   (2) High-level patterns; patterns designed to calculate or
       emulate something.
   (I guess these two things may not have a lot in common, but
   I don't want to have too many separate directories.)

c2-extended
   A by-period collection of c/2 spaceships, puffers, and (especially)
   rakes.

fuses
   A collection of some ways that linearly-repeating patterns can decay.

guns
   Spaceship guns. Most ordinary glider guns would be put into
   other collections instead of this one, so most of the guns here
   are unusual in some way -- they may have an adjustable period,
   or an unusual shape, or fire multiple spaceships or unusual
   kinds of spaceships.

interactions
   (For lack of a better name.)
   Moving reactions that are supported by one or more guns or
   puffers, which move at a different velocity than the reaction
   itself.

misc
   Anything that doesn't fit into another directory. Mostly,
   reactions that are interesting or useful.

osc
   A by-period collection of oscillators.

osc-supported
   "Oscillators" for which the only known way to support them is to use
   glider guns, or the moral equivalent of glider guns.

slideguns
   "Guns" that fire each glider on a separate path. See the section
   below for more information.

synthesis
   Patterns of gliders that collide to form complex objects.

velocity-...
   Spaceships, puffers, stretchers, etc. Separated by velocity.

wicks
   Extensible oscillators, mostly untamed.


----- Additional information about the "c2-extended" directory -----

This used to be a separate "rakes" collection, but I've incorporated
it into this collection, and expanded it.

True-period rakes are known for *all* multiples of 8, and all multiples
of 4 higher than 564. The files "var-072+8n.lif" and "var-???+24n.lif"
are the starting points for building them.


----- Additional information about the "osc" directory -----

Refer to the readme-osc.txt file.


----- Additional information about the "slideguns" directory -----

The objects here are separated into two basic types: "slide guns" and 
"tethered rakes". The difference is that the rate of outward movement of a 
tethered rake is fixed, while a slide gun's outward movement can be adjusted 
by changing the period of the stationary part.

(I may be abusing the term "tethered rake" when I use it to include patterns
like "teth-o-014b-090.lif" that aren't based on a spaceship or puffer.)


File naming convention:

TYPE-D-XXd-YYY.lif

TYPE: slide=slide gun, teth=tethered rake

D: Direction of spaceship salvos, o=orthogonal, d=diagonal

XX: Outward movement, in cells per period

d: Direction of output relative to the pattern. Not always applicable.
    (b=backward gliders, f=forward gliders)

YYY: Apparent period of the stationary part (gun)


----- Additional information about the "wicks" directory -----

A "wick" is an infinite linearly repeating oscillator.

A "fencepost" is an object that can stabilize one end of a wick. Most of 
the wicks in this collection have no known fenceposts, so the patterns 
will blow up pretty quickly. If a fencepost is known, it is usually 
included with the pattern. Uninteresting fenceposts for some of the high 
period wicks can be built by firing gliders at the ends -- some of these 
would be very large, and are not included.

The period of a wick can be expressed in the form A/B, where A is the 
period, and A/B (divided out) is the number of generations it takes for a 
congruent pattern to appear (the same pattern but possibly shifted and/or 
flipped). If A=B, the wick is called a "barber pole". Wicks where B=1 are 
usually the most interesting, since they are typically the easiest to 
stabilize.

This collection does not attempt to include every known wick. In fact, an 
infinite number are known. Consider the following wicks:

**..**..**..**    **..**.**..**.**.    **..**..**..**..**..
**..**..**..**    **..**.**..**.**.    **..**..**..**..**..
..............    .................    ....................
**************    *****************    ********************
..............    .................    ....................
.***..***..***    .****..****..****    .*****..*****..*****
**...**...**..    **....**....**...    **.....**.....**....
.***..***..***    .****..****..****    .*****..*****..*****
..............    .................    ....................
**************    *****************    ********************
..............    .................    ....................
**..**..**..**    **..**.**..**.**.    **..**..**..**..**..
**..**..**..**    **..**.**..**.**.    **..**..**..**..**..

  Period 5/5          Period 6/6              Period 7/7

The series can be continued to produce a wick of any desired period.

Any spaceship "tagalong" that can be extended in the direction of the 
spaceship's movement also constitutes a wick. For example:

.*.........*.........*
**......****......****
*.....***.*.....***.*.
*....**...*....**...*.
**...*....**...*....**
.***.*.....***.*.....*
...****......****.....
......*.........*.....

    Period 20/10

Wicks of this type are usually not include in this collection.

Also not included are:

 * wicks that have been made finite, and are more interesting when 
considered to be an oscillator than a wick

 * wicks of period 3 or less


Most of the wicks in this collection were found independently by me, using a 
random torus search program. However, a substantial fraction of them were 
discovered by other people. I have not gone to much effort to determine and give 
credit to the first discoverer. In some cases that would be almost impossible 
anyway, since many of these wicks would not have been considered interesting 
enough to make public.
