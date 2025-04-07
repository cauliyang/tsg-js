pub mod utils;

use tsg_core::graph::TSGraph;
use wasm_bindgen::prelude::*;
use web_sys::console;

// Initialize panic hook in a function that gets called
fn init_wasm() {
    utils::set_panic_hook();
}

#[wasm_bindgen(start)]
pub fn start() {
    init_wasm();
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
pub fn load_graph(raw_content: &str) -> Result<Vec<String>, JsValue> {
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
    let mut graph_jsons = Vec::new();

    for (_id, graph) in graph.graphs.iter() {
        let graph_json = graph.to_json().unwrap();
        // Convert serde_json::Value to String
        let graph_json_str = match serde_json::to_string(&graph_json) {
            Ok(json_str) => json_str,
            Err(e) => {
                let error_msg = format!("Failed to convert graph to JSON: {}", e);
                console::error_1(&error_msg.clone().into());
                return Err(JsValue::from_str(&error_msg));
            }
        };
        graph_jsons.push(graph_json_str);
    }
    Ok(graph_jsons)
}
