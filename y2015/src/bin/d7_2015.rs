use utils::get_input_data;

use std::{collections::HashMap, cell::RefCell};
use regex::Regex;

// Possible Gate Types with their values
#[derive(Clone)]
enum Gate {
  Not(Value),
  And(Value, Value),
  Or(Value, Value),
  Lshift(Value, Value),
  Rshift(Value, Value),
}

// Possible signal value type
#[derive(Clone)]
enum Value {
  Wire(String),
  Int(i32)
}

// A Wire's signal is either the result of a gate or a value type
#[derive(Clone)]
enum Signal {
  Gate(Gate),
  Value(Value),
}

type Cache = RefCell<HashMap<String, i32>>;

// A cirucit is a mapping of wires with their corresponding signal
type Circuit = HashMap<String, Signal>;

#[tokio::main]
async fn main() {
  // Get input data
  let data = get_input_data("2015", "7").await.trim().to_string();
  
  let circuit = build_the_circuit(data);

  // Get part 1 answer
  let part1_cache = RefCell::new(HashMap::new());
  let part1_answer = determine_output_at(&circuit, &part1_cache, "a");
  println!("Part 1 Answer: {} output at 'a'", part1_answer);

  // Get part 2 answer (override b signal with the value from a)
  let part2_cache = RefCell::new(HashMap::new());
  part2_cache.borrow_mut().insert(String::from("b"), part1_answer);
  println!(
    "Part 2 Answer: {} output at 'a'",
    determine_output_at(&circuit, &part2_cache, "a")
  );
}

/// Determines the output of a circuit at the specificied wire identifier
/// by traversing through the circuit's structure. Updates the circuit to cache
/// the outputs of each wire as it is determined
fn determine_output_at(circuit: &Circuit, cache: &Cache, wire_ident: &str) -> i32 {
  // Check the cache for a deterministic output
  if let Some(cached) = cache.borrow().get(wire_ident) {
    return *cached;
  }
  // Evaluate signal output
  let output = match circuit.get(wire_ident).unwrap() {
    Signal::Gate(gate) => {
      match gate {
        Gate::Not(val) => !val_out(circuit, cache, val),
        Gate::And(l, r) => val_out(circuit, cache, l) & val_out(circuit, cache, r),
        Gate::Or(l, r) => val_out(circuit, cache, l) | val_out(circuit, cache, r),
        Gate::Lshift(l, r) => val_out(circuit, cache, l) << val_out(circuit, cache, r),
        Gate::Rshift(l, r) => val_out(circuit, cache, l) >> val_out(circuit, cache, r),
      }
    },
    Signal::Value(val) => val_out(circuit, cache, val),
  };
  // Cache output
  cache.borrow_mut().insert(String::from(wire_ident), output);
  output
}

// Get a value's output
fn val_out(circuit: &Circuit, cache: &Cache, value: &Value) -> i32 {
  match value {
    Value::Int(val) => *val,
    Value::Wire(wire_ident) => determine_output_at(circuit, cache, wire_ident),
  }
}

// Parses the input data and generates a hashmap of all wires representing the circuit
fn build_the_circuit(data: String) -> Circuit {
  let mut circuit: Circuit = HashMap::new();
  let re = Regex::new(r"^(((?P<left>[a-z0-9]+) )?(?P<gate>[A-Z]+) )?(?P<right>[a-z0-9]+) -> (?P<wire>[a-z]+)$").unwrap();
  for line in data.lines() {
    // Parse line and determine the wire's values
    let caps = re.captures(line).unwrap();
    let wire_str = String::from(caps.name("wire").unwrap().as_str());
    let right = str_to_value(String::from(caps.name("right").unwrap().as_str()));
    // Handle the gate logic
    if let Some(gate_match) = caps.name("gate") {
      let gate_str = gate_match.as_str();
      // Handle not gate
      if gate_str == "NOT" {
        circuit.insert(wire_str, Signal::Gate(Gate::Not(right)));
      } else {
        // Handle all other gates
        let left = str_to_value(String::from(caps.name("left").unwrap().as_str()));
        let gate = match gate_str {
          "AND" => Gate::And(left, right),
          "OR" => Gate::Or(left, right),
          "LSHIFT" => Gate::Lshift(left, right),
          "RSHIFT" => Gate::Rshift(left, right),
          _ => panic!("Unhandled gate detected: {}", gate_str),
        };
        circuit.insert(wire_str, Signal::Gate(gate));
      }
    } else {
      // Signal is just a value
      circuit.insert(wire_str, Signal::Value(right));
    }
  }

  circuit
}

// Converts a raw string into a value
fn str_to_value(raw: String) -> Value {
  // Check if value is numeric
  if raw.chars().all(|c| c.is_numeric()) {
    Value::Int(raw.parse().unwrap())
  } else {
    Value::Wire(raw)
  }
}