// The point of this file is to generate most of the game logic so it can be easily called via a functional interface.

pub trait Game {

}

pub struct Game {
    pub teamA: Team,
    pub teamB: Team,
    pub dayNum: i32,
    pub startTime: i128,
    pub season: i32,
}
