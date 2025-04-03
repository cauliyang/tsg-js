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
    alert("Hello, tsg-core-js!");
}

// create a fun to add two numbers
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function to load a graph from a raw string (TSG format)
#[wasm_bindgen]
pub fn load_graph(raw_content: &str) -> Result<String, JsValue> {
    console::log_1(&"Loading graph from raw string...".into());

    // Load the graph from the raw string
    let graph = match TSGraph::from_str(raw_content) {
        Ok(g) => g,
        Err(e) => {
            let error_msg = format!("Failed to parse graph: {}", e);
            console::error_1(&error_msg.clone().into());
            return Err(JsValue::from_str(&error_msg));
        }
    };

    // Create a summary of the loaded graph
    let mut summary = String::new();

    for (id, graph) in graph.graphs.iter() {
        let graph_summary = format!(
            "Graph ID: {} with {} nodes and {} edges",
            id.to_string(),
            graph.nodes().len(),
            graph.edges().len()
        );

        console::log_1(&graph_summary.clone().into());
        summary.push_str(&graph_summary);
        summary.push_str("\n");
    }

    Ok(summary)
}
