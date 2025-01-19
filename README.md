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

### MVP

- [ ] Player can move, upon which the horizon gives indication of movemnt (by objects and via head bobble)
- [ ] Other entities exist (structures and individuals), some of which can move through the world on their own
- [ ]  (Narrative) Text is shown
- [ ] Text can show location based, too
- [ ] There are at least two endings
- [ ] An ending is an animation and the credits

### Stretch goals

- [ ] WASM
- [ ] Changing skies
- [ ] Animated sky/ clouds
- [ ] Atmospherics that are not approachable / move away
- [ ] Lights
- [ ] Rogue-aspect
- [ ] Sound

## Game Ideas/Notes

- Narrative comes animated (flash in, degrade over time)
- Narrative is accompanied by sound
- Breathing moves horizon/height anyway

## Derived ToDos

- [ ] Head bobble
- [ ] Screen and Window sizing is messed up. Use a proper camera and fixed ratios.
- [ ] player sees points or objs on horizon
- [ ] horizon and ground color changes

## (Code) Design desicions

In [./src/README.md](./src/README.md).

## Lessons learnt

- Test framework brings the basics, but I miss some convenience from rspec/minitest. Trying some other crates for a fine layer of sugar.
- Inline doc with examples/tests sounds great but did not work (out of the box) yet.

## Licence, Copyright

Released under the GPLv3 or any later version.
Copyright 2025, Felix Wolfsteller
