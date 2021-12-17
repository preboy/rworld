pub struct Packet {
    Len: i32,
    Code: i32,
    Data: Box<[&u8]>,
}
