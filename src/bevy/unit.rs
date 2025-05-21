struct Position {
    x: usize,
    y: usize,
}

enum UnitType {
    Positional(PositionalUnitType),
    Runner { has_jugg: bool },
    Player(PlayerUnitType),
    Jugg,
}

enum PositionalUnitType {
    One,
    Two,
    Three,
    Four,
    Five,
}

enum PlayerUnitType {
    Chain,
    Long,
    Staff,
    QTip,
    Shield,
    DoubleShort,
}

enum UnitState {
    Active,
    Inactive { downtime: u8, pin_stone: bool },
    Pinned { downtime: u8 },
}

struct Unit {
    team: u8, //TODO
    position: Position,
    unit_type: UnitType,
    state: UnitState,
}
