pub mod utils;

use tsg_core::graph::TSGraph;
use wasm_bindgen::prelude::*;
use web_sys::console;

// Initialize panic hook in a function that gets called
fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen(start)]
pub fn start() {
    init();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tsg-js2!");
}

// create a fun to add two numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// create a function to load a graph from a file
#[wasm_bindgen]
pub fn load_graph(file: &str) -> Result<(), JsValue> {
    // Load the graph from the file
    let graph = TSGraph::from_file(file).map_err(|e| JsValue::from_str(&e.to_string()))?;

    for (id, graph) in graph.graphs.iter() {
        console::log_1(
            &format!(
                "Graph ID: {} with {} nodes and {} edges",
                id.to_string(),
                graph.nodes().len(),
                graph.edges().len()
            )
            .into(),
        );
    }

    Ok(())
}
