# Design decisions

Document decisions I took, often not very informed.

## Math

- Use polar coordinates and **radians** for stuff that isnt ultimatively approachable (e.g. lights on horizon)
- I started out representing the circle range going from -󰏿 to +󰏿
    - conceptually easier, you have "left of middle" and "right of middle"
    - the minuses gave me headaches
    - decided that a full circle is `0..2󰏿`
- The `radians` and `radian` crates look cool, but lack examples and I cannot read and understand that code yet.
- Lets see how far f32 brings us.
- Nonetheless, the direction of the player points to the middle of its viewfield. This makes calculations more cumbersome (need to substract view angles etc) and probably computationally more expensive, but is another minus-headache tradeoff.
- render and project from left to right, like this we dont underrun when setting pixels in frame
