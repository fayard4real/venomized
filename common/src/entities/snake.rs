pub enum Direction {
    North,
    South,
    West,
    East,
}

/// The main entity in the game,
/// which is stored on both the
/// client and server sides with the same structure.
pub struct Snake {
    /// id for identifying clients on the server and among themselves
    pub id: u64,

    /// A variable that is necessary in many cases,
    /// indicating the direction in which our snake is moving.
    ///
    /// 1. To add a tail when eat an apple
    /// 2. To remove the tail
    /// 3. Prediction??
    pub direction: Direction,
    // TODO: add how to contain the Snake position
    // TODO: make methods for add tail, remove tail
    // and something like that
}
