use utils::get_input_data;

use std::collections::HashMap;
use regex::Regex;

// Possible Gate Types with their values
enum Gate {
  Not(Value),
  And(Value, Value),
  Or(Value, Value),
  Lshift(Value, Value),
  Rshift(Value, Value),
}

// Possible signal value type
enum Value {
  Wire(String),
  Int(i32)
}

// A Wire's signal is either the result of a gate or a value type
enum Signal {
  Gate(Gate),
  Value(Value),
}

// A cirucit is a mapping of wires with their corresponding signal
type Circuit = HashMap<String, Signal>;

#[tokio::main]
async fn main() {
  // Get input data
  let data = get_input_data("2015", "7").await.trim().to_string();
  
  let circuit = build_the_circuit(data);

  println!("Part 1 Answer: {} output at 'a'", determine_output_at(&circuit, "f"));
}

/// Determines the output of a circuit at the specificied wire identifier
/// by traversing through the circuit's structure. Updates the circuit to cache
/// the outputs of each wire as it is determined
fn determine_output_at(circuit: &Circuit, wire_ident: &str) -> i32 {
  let signal = circuit.get(wire_ident).unwrap();
  // Evaluate signal output
  match signal {
    Signal::Gate(gate) => {
      match gate {
        Gate::Not(val) => !val_out(circuit, val),
        Gate::And(l, r) => val_out(circuit, l) & val_out(circuit, r),
        Gate::Or(l, r) => val_out(circuit, l) | val_out(circuit, r),
        Gate::Lshift(l, r) => val_out(circuit, l) << val_out(circuit, r),
        Gate::Rshift(l, r) => val_out(circuit, l) >> val_out(circuit, r),
      }
    },
    Signal::Value(val) => val_out(circuit, val),
  }
}

// Get a value's output
fn val_out(circuit: &Circuit, value: &Value) -> i32 {
  match value {
    Value::Int(val) => *val,
    Value::Wire(wire_ident) => determine_output_at(circuit, wire_ident),
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