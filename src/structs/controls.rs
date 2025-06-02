#[derive(Clone, Debug)]
pub enum Instruction {
    Forward,
    TurnLeft,
    TurnRight,
    CallFunction(usize),                    // Appelle la fonction numérotée
    ConditionalRed(Box<Instruction>),       // Exécute si tuile rouge
    ConditionalGreen(Box<Instruction>),     // Exécute si tuile verte
    ConditionalBlue(Box<Instruction>),      // Exécute si tuile bleue
    Noop,                                   // Instruction vide/placeholder
}

#[derive(Default, Clone, Copy)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn get_offset(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    // Parse depuis string (pour fichier config)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NORTH" | "N" => Some(Direction::North),
            "EAST" | "E" => Some(Direction::East),
            "SOUTH" | "S" => Some(Direction::South),
            "WEST" | "W" => Some(Direction::West),
            _ => None,
        }
    }

    pub fn to_rotation(self) -> f32 {
        match self {
            Direction::North => 0.0,
            Direction::East => std::f32::consts::PI / 2.0,
            Direction::South => std::f32::consts::PI,
            Direction::West => -std::f32::consts::PI / 2.0,
        }
    }
}