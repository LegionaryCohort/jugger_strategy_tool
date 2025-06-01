# Jugger Strategy Tool
A rewrite of the [original strategy tool](https://neverdowells.com/JuggerStrategyTool/) developed by neverdowells.


## Goals

This rewrite aims to target two primary use-cases:

### 1) Support tool for training exercises

The requirements for this use-case are:
- Touch-only input (for tablet usage)
- Positioning players on the field
- Drawing simple arrows

It should be as simple as possible to quickly "draw" a scenario including positions and, using arrows, movement indicators. The aim is to allow a trainer to use a tablet to clearly communicate a situation to players, without having to fidget around with the tool a lot.

### 2) Visualization tool for complex game analysis

The requirements for this use-case are:
- Detailed display of complex information (remaining downtime, pins, etc.)
- Displaying movements and events by animating player movement along arrows

It should be possible to accurately reconstruct game situations in full detail, including animating movements of players and events (e.g. a player getting hit). The aim is to visually support detailed analysis and discussion of game situations, e.g. for video content. It is acceptable for this mode to have more complex input controls.


## Tools & structure

The tool is written entirely in Rust. It uses the [Leptos](https://leptos.dev/) web framework for the main page and control components and the [Bevy engine](https://bevyengine.org/) for the managing the contents of the canvas.

The tool is designed as a client-side only single page web-app.