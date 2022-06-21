use crate::comp::{EntityId, Comp, CompIter};


#[derive(Clone, Debug, PartialEq, Default)]
struct GameResults {
    pub id: Vec<EntityId>,
    pub season: Season,
    pub day: Comp<u32>,

    pub kind: Comp<Kind>,
    pub away: Comp<Team>,
    pub home: Comp<Team>,
    pub weather: Comp<Weather>,
    pub logs: Comp<Logs>,
}

impl GameResults {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn create_game_results(&mut self, season: Season, day: u32, kind: Kind, away: Team, home: Team, weather: Weather, logs: Logs) -> EntityId {
        let entity_id = self.id.len() as EntityId;
        self.id.push(entity_id);
        self.season.insert(entity_id, season);
        self.day.insert(entity_id, day);
        self.kind.insert(entity_id, kind);
        self.away.insert(entity_id, away);
        self.home.insert(entity_id, home);
        self.weather.insert(entity_id, weather);
        self.logs.insert(entity_id, logs);

        return entity_id;
    }
}

pub enum Kind {
    /// This game affects regular season standings
    Regular,
    /// This is the first set of postseason games
    Postseason1,
    /// This is the second set of postseason games
    Postseason2,
    /// This is the third set of postseason games
    Postseason3,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Regular
    }
}