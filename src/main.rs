use dioxus::prelude::*;
use dioxus_time::{use_interval};
use js_sys::Math;
use std::time::Duration;

// Static assets bundled by `asset!`
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

// Universe model
#[derive(Clone, PartialEq, Debug)]
struct Universe {
    width: usize,
    height: usize,
    /// Flattened 2‑D grid – `true` = alive, `false` = dead
    cells: Vec<bool>,
}

// Cross‑platform *fair‑coin* helper
// * `#[cfg(target_arch = "wasm32")]` — the item *below* the attribute
//   is **compiled only** when the *current* `--target` triple’s
//   `target_arch` field equals `"wasm32"` (that is, you are building
//   for `wasm32‑unknown‑unknown` or `wasm32‑wasi`).
// * `#[cfg(not(target_arch = "wasm32"))]` — the inverse: compile this
//   item for every other architecture (x86_64, aarch64, etc.).
//
// Because the two functions share the same *symbol* (`random_bool`) but
// live behind **mutually‑exclusive** `#[cfg]` gates, exactly **one** of
// them is present in the final binary; the other is discarded at compile‑time.
//
// * **Web**→ `Math.random() > 0.5`  (fast, no‑std)
// * **Native**→ `rand::Rng::gen_bool(0.5)` (OS RNG)

#[cfg(target_arch = "wasm32")]
fn random_bool() -> bool {
    js_sys::Math::random() > 0.5
}

#[cfg(not(target_arch = "wasm32"))]
fn random_bool() -> bool {
    use rand::Rng;
    rand::thread_rng().gen_bool(0.5)
}

impl Universe {
    /// Create a new universe initialised with random live/dead cells.
    fn new(width: usize, height: usize) -> Self {
        let cells = (0..width * height).map(|_| random_bool()).collect();
        Self { width, height, cells }
    }

    /// Advance one generation according to Conway's rules.
    fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                let live_neighbors = self.live_neighbor_count(row, col);
                next[idx] = match (self.cells[idx], live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.cells = next;
    }

    /// Count the eight neighbours around `(row, col)` (edges wrap).
    fn live_neighbor_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // Skip the cell itself
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = neighbor_row * self.width + neighbor_col;
                count += self.cells[idx] as u8; // Add 1 if true (alive), 0 if false (dead)
            }
        }
        count
    }
}

// Bootstrapping
fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    // reactive state
    let mut universe = use_signal(|| Universe::new(128, 128));
    let cell_size = 4;

    //milliseconds
    const BASE_DELAY: u64 = 50;
    let delay = use_signal(|| BASE_DELAY);

    // Start an interval that calls `tick` every `delay()` milliseconds.
    use_interval(Duration::from_millis(delay()), move |_| universe.write().tick());

    let restart = move |_| {
        universe.set(Universe::new(128, 128));
    };

    // view
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {
            class: "container",
            h1 { "Conway's Game of Life" }
            div { class: "controls",
                button { onclick: restart, "Restart (R)" }
                p { "Update delay: {delay()}ms" }
            }

            div {
                class: "game-board",
                style: format!(
                    "display: grid; grid-template-columns: repeat({}, {}px); grid-template-rows: repeat({}, {}px);",
                    universe().width, cell_size, universe().height, cell_size
                ),

                {universe().cells.iter().enumerate().map(|(idx, &alive)| {
                    rsx! {
                        div {
                            key: "{idx}",
                            class: if alive { "cell alive" } else { "cell dead" },
                            style: format!("width: {cell_size}px; height: {cell_size}px;")
                        }
                    }
                })}
            }
        }
    }
}