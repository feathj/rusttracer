Rusttracer
==========
A port of my c++ raytracer into rust (for cultural learnings for make benefit) 

I have no idea what I am doing in rust, so I ported my c++ raytracer so I could learn. This is a pet project so don't take it too seriously.

Completely software rendered, but drawn to screen using SDL.

![scene preview 1](https://github.com/feathj/rusttracer/blob/master/images/preview1.png)


To build
========

osx
---
`brew install sdl2 sdl2_gfx`

Add the following to your path:
`export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"`

`cargo build`
`cargo run`
