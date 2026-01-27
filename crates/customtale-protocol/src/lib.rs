pub mod serde;

#[path = "generated/packets.rs"]
pub mod packets;

#[path = "generated/tests.rs"]
#[cfg(test)]
mod tests;
