min2phase-ui
============

An egui-powered desktop UI for exploring the min2phase Rubik's cube solver.

<img width="904" height="905" alt="image" src="https://github.com/user-attachments/assets/fbc7a068-c157-4589-8b66-ddead7dd942c" />


What this is
------------
- A small Rust app that uses the min2phase library and exposes common operations via UI.
- Designed to help you tinker: scramble cubes, apply custom move sequences, and solve with adjustable move limits.

Getting started
---------------
Prerequisites:
- Rust and Cargo installed (`https://rustup.rs`)

Run the app:
```bash
cargo run
```

Using the UI
------------
1) Top section
- Facelet: paste or edit a 54-character facelet string. If empty or invalid, the UI uses the current painted state.
- Rotations/Moves: paste any sequence matching `([URFDLB][123'] ?)*`.
- Buttons:
  - Solve Cube: solves the facelet; the solution is written into the moves field and logged (with length).
  - Random Cube: generates a new cube and updates the facelet field and editor.
  - Scramble: creates a random scramble (25 moves) and applies it.
  - Apply Moves: applies the moves field onto the current facelet.
  - Reset: restores the previous facelet and clears text fields.

2) Face editor
- Pick a color from the palette, then click a tile to paint it.
- Center tiles are labeled (U, R, F, D, L, B) to help orientation.

3) Options
- Move Limit: drag to change the search depth cap (1–24). 21 is a good default.

Facelet format refresher
------------------------
Order is: `U1..U9 R1..R9 F1..F9 D1..D9 L1..L9 B1..B9` using letters `U, R, F, D, L, B`.
Example (solved):
```
UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB
```

Acknowledgements
----------------
- min2phase_rust https://github.com/cs0x7f/min2phase_rust
- Rust `egui`/`eframe`
