//! All IDs that will be used by the server and client 
//! to exchange information are stored here.

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, num_enum::TryFromPrimitive)]
pub enum LoginServerbound {
    Login = 0
}

pub enum ConfigureServerbound {

}

pub enum PlayServerbound {
    TurnSnake
}

pub enum LoginClientbound {}

pub enum ConfigureClientbound {}

pub enum PlayClientbound {}

/*
struct Login {
    username: String,
}

struct LoginSuccess;

// talk position of player
struct SynchonizePositionAndDirection {
    x: u32, // 1-4 bytes
    y: u32, // 1-4 bytes
    direction: Direction // 1 bytes always
}

struct SpawnSnake {
    id: u64 // 4-8 bytes
    x:  u32 // 1-4 bytes
    y:  u32 // 1 - 4
    direction: Direction // 1 
} // 7-17 bytes

struct RemoveSnake {
    array_of_ids: Array<id>
} // id = u64 = 4 bytes +- 

struct UpdateApple {
    pos_x: u32 //(1-4 bytes)
    pos_y: u32 //(1-4 bytes)
    state: //Eaten or Spawn 0 or 1 (1 bytes)
} // 3-9 bytes

struct AppleSpawnButch {
    pos_x: u32,
    pos_y: u32,
    directions: Vec<u8> // 2 bits = 1 flag Direction
}
*/