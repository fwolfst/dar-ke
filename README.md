# Dar-Ke

An experimental game written in (order to learn) [Rust](rust-lang.org) with the [Bevy](bevyengine.org) game engine. Probably trying gitlab CI/CD as build and test service at some point. Very low resolution. Very little interaction. No content.

> There is no light

> The horizon lies far

> What do you see

> You must wander

## Tooling

Create inline bitmaps like so:
```
cargo run --bin ppm_rs_util giant.ppm
   # ...
const ˑ: bool = true;
const Ø: bool = false;
const BITMAP: [[bool; 19]; 24] = [
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,Ø,Ø,ˑ,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,Ø,Ø,Ø,Ø,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,Ø,Ø,Ø,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,Ø,Ø,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,Ø,Ø,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,Ø,Ø,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,Ø,ˑ,Ø,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,Ø,Ø,ˑ,ˑ,ˑ,Ø,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
  [ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,ˑ,],
];
```

## Design Doc

Inspired by Herbert Wolversons book ["Hands-on Rust"](https://hands-on-rust.com/) (I think its available free, but if you can: support the press and creators - buy content/books).

### Story

You find yourself in a mysterious dark pretty void place and discover it together with snippets of your past.

### Basic Game Loops

Player tries to orient and align herself and the entities in the world around herself, picking up clues and ultimatively finding an end (a beginning) to the story, after which a tiny animation and the credits are shown.
Player sees the world through ego-perspective. Game can be played with one hand.

#### The three stages

- Finding space itself
- Finding trees, tree and fire

### MVP

- [X] Player can move, upon which the horizon gives indication of movemnt (by objects and via head bobbing)
- [X] Other entities exist (structures and individuals), some of which can move through the world on their own
- [ ] There are three stages / riddles to progress through
- [ ]  (Narrative) Text is shown
- [ ] Text can show location based, too
- [ ] There are at least two endings
- [ ] An ending is an animation and the credits
- [ ] A narrative guides through the whole experience

### Stretch goals

- [ ] WASM
- [ ] Minimap for debugging
- [ ] Changing skies
- [ ] Animated sky/ clouds
- [ ] Atmospherics that are not approachable / move away
- [ ] Lights
- [ ] Rogue-aspect
- [ ] Sound

### Constraints I'd like to keep

- No Z sorting
- Playable with a single hand
- No classical UI
- No movement restriction via collision detection

## Derived ToDos

- [ ] Head bobbing when looking up
- [ ] Trees at position (vs at horizon)
- [ ] Birds landing at tree
- [X] Has Intro
  - [X] which blends over into gameplay
  - [ ] which triggers first narrative
- [X] Moving forward has a visible effect (besides head movement)
  - [ ] horizon and ground have some texture
  - [ ] objs appear bigger when close
- [ ] giants can move
- [ ] certain areas can trigger certain events or enable systems, e.g.
  - [ ] going down (to ground)
  - [X] slight cam shake
- [ ] certain events trigger certain narratives
- [ ] there is a trigger for the credit screen
- [ ] there is a credit screen
- [ ] fix fullscreen toggle in macos
- [ ] hide mousepointer and egui when started with certain arg (later make this default)
  - [ ] (dupe) optionally disable mousepointer in fullscreen (then no egui)
- [ ] there is a global darkness
- [ ] Screen and Window sizing is messed up. Use a proper camera and fixed ratios.
- [ ] Use bevy Color::Mix or colorgrad-rs to interpolate between colors (or interpolate() or blend())
- [X] player sees objs on horizon
- [X] interesting horizon
- [X] player can approach objs on ground
- [X] player sees points on horizon
- [X] horizon and ground can have some circle gradients ("light" with color)
- [X] Head bobbing

### Other ToDos

- [ ] Have a street
- [ ] need to test/jump to stages, skip intro etcpp
- [ ] a minimal init screen/objs (in game)
- [ ] deterministic RNG with seed in params
- [ ] Learn and straighten the imports
- [ ] use radian or radians crate
  - [ ] switch polar coord to -pi..pi
- [ ] settle on separation between renderer and projector
- [ ] projector tests
- [ ] Use Vec2 more often
- [ ] One prototype per feature?

## Game Ideas/Notes

- Narrative comes animated (flash in, degrade over time)
- Narrative is accompanied by sound
- Breathing moves horizon/height anyway (even when not moving)
- light cone moves when moving (even when turning)
- different "abilities" are added gradually (e.g. in the beginning no light)
- Stars blink on sky
- Things fall down (rain, asteroid, ash, fire, ...)
- When walking back something happens, e.g. a display of something - but this might be a dimension too much. I might not want anything onscreen
- Fall to ground (looking down)
- Screenshakes
- Things can glow and project light
- Falling thins have afterglow/smoke
- A thunder strucks at the middle every x seconds, destroying a part of the horizon.

## (Code) Design desicions

In [./src/README.md](./src/README.md).

## Lessons learnt

### General

Live-Parameterization (via egui) helps to flesh out some variants.

HSL might be the better color space.

Math is 20 years old.

### Rust

- Test framework brings the basics, but I miss some convenience from rspec/minitest. Trying some other crates for a fine layer of sugar.
- Inline doc with examples/tests sounds great but did not work (out of the box) yet.
- Range/RangeBounds do not allow ranges where start > end :(

### API/Bevy

* There is bevy::Color::Mix and ColorCurve to interpolate between colors
* SubStates might help to model levels/stages.
* You cannot have two queries for the same mutable component

## Licence, Copyright

Released under the GPLv3 or any later version.
Copyright 2025, Felix Wolfsteller
