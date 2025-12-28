//! Device pairing and authentication
//!
//! Implements TOFU (Trust On First Use) pairing with 4-digit codes

use rand::Rng;

/// Generate a 4-digit pairing code
pub fn generate_pairing_code() -> String {
    let code = rand::thread_rng().gen_range(1000..10000);
    code.to_string()
}

// TODO: Implement pairing handshake
// TODO: Implement certificate exchange
// TODO: Implement trusted device storage
