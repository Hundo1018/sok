# sok
Sok is a toy game engine using rust, a easy wasm game engine to help me understand how game engine work and made, include render pipeline, resource management, game-loop.

> Warning: This Project is made by following tutorials, so may not keeping maintenment, DO NOT USE IN PRODUCTION thanks.

## Tech Stacks
What I Use in this project.

| Lib/Frame                              | Use-For       |
| -------------------------------------- | ------------- |
| Wasm                                   | Target        |
| Wgpu                                   | Render        |
| CPAL / Kira                            | Audio         |
| Rapid                                  | Physics       |
| Spacetimedb                            | Networking    |
| Egui                                   | UI            |
| React(or any platform supporting WASM) | Test Platform |

## Getting Started
How do you use this project.
1. Clone Project
2. Install environment dependencies (Rust toolchain, etc.)
3. Create a hello world project
4. Debug & Build & Run

## Roadmap
What should I do and what I shouldn't do.
### Basic Features

 - [x] Rust-Wasm-Vite Build
 - [x] Render
   - [ ] 2D
     - [x] Triangle
     - [ ] SVG
     - [ ] WebP
   - [ ] 3D 
     - [ ] Model (Gltf)
     - [ ] Animation Handling
 - [ ] Audio
   - [ ] WAV
 - [ ] Gizmos
 - [ ] Physics
   - [ ] Rapid
 - [ ] Networking
   - [ ] Spacetimedb
 - [ ] UI
   - [ ] egui
### Advance
 - [ ] Physics-based Render
 - [ ] Physics-based Audio
 - [ ] Custom Physics Enigne
 - [ ] Reactive Design
 - [ ] Entity-Component-System
 - [ ] Write a better document

### I Promise this Project woudn't:
 - [x] A Nice Editor
 - [x] Very High performace
 - [x] Very Small runtime
 - [x] Always spelling English correct
 - [ ] Letting your dog control your fish (immediately)
## References
- [Learn Wgpu](https://sotrh.github.io/learn-wgpu/)

---

By the way, Sok is "Shrink" in Taiwanese, When a engine overheated and Thermal expansion, it will let the stuck the engine.