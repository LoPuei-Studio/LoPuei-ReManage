use serde::{Serialize, Deserialize};
#[derive(Serialize,Deserialize,Debug)]
struct RoomDefinition{
    id: String,
    name: String,
    width: u32,
    length : u32,
    height: u32,
    power: f32,
}

fn main() {
    let room = RoomDefinition{
        id: String::from("01"),
        name: String::from("habitat01"),
        width: 4,
        length: 4,
        height: 2,
        power: 15.0
    };

    let serialized = serde_json::to_string_pretty(&room).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: RoomDefinition = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
