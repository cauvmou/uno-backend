pub mod incoming;
pub mod outgoing;

pub trait Packet {
    fn serialize(&self) -> [u8;1024] {
        [0; 1024]
    }
    fn deserialize(&self) -> Self;
}