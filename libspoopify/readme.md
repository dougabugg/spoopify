# Just some notes on the design

I often run into a "mental block" and stop making projects on "abstract" projects like this. Sure, I could just go to any
old random music player app, and copy their "database layout", Artist-Album-Track, whatever, and be done with it. But
when I program, my inner perfectionist comes out, and I feel like I can't move onto the next part of the implementation
until I've "perfected" the current part, even though that's literally impossible, and I'll probably end up changing the
current part later while implementing the other parts to fix bugs and improve the overall design.

ANYWAYS. I think I'll add support (to some extent) for custom fields, but the app will also have built in a standard set
of organizational groups, like the standard Album-Artist, Track-Artist, Album, Track-Title. From there, I think I'll add
support for an abstract "group" or something, for example, `Knower` is composed of `Louis Cole` and `Genevieve Artadi`,
so the "group" entry for `Knower` would have some sort of "link" to `Louis` and `Genevieve`. I would also like to add
support for specifying other performers within a single track, and associating a "role" (instrument/part?) with a "group".

This way, we can still have the "primary" navigation by Album-Artist, Track-Artist, and include other fields for specific
notes on who the original composer was, and including all the performers for a particular track. Now I just need to
hammer out the database design to support such a complicated and over-engineered system LMAO.
