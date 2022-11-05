# [str]
A technical smart storage real life organisational system for the perfectionist, written in Rust.
> ⚠️ In heavy development - doesn't work properly yet. I may also choose to abandon this project if it isn't worthwhile as it's complicated.

## How does it work?
- Areas are numbers seperated by dots going further down the hierarchy into more and more sub areas (i.e. 3.1.2)
  - For example, A set drawers is the area 1. Each drawer is therfore 1.1, 1.2, 1.3, etc.
- Areas must have a set description i.e. 5 is a shelf which holds my collectables and decor (games, figures, etc.)
- Areas have to be real life physical objects or containers (i.e. a box, a shelf or a desk)
  - I.e. they cannot be half of a windowsil or the left side of a desk, etc.
 - The root 0 is reccommended to be reserved this for your person, (i.e. 0.1 is left pocket 0.2 is right pocket)
 - Areas ending with 0 are used to say "all items which are in the area but not in any sub areas" 

## How do we use str?
- For adding something to the system: `str add [area] [item] [type]`
  - E.g. `str add 2.1 Nintendo DS +charging cable`
- For reading the system: `str show`
