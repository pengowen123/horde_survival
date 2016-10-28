#[derive(Clone, PartialEq, Eq)]
pub enum HasGravity {
    True,
    False,
}

#[derive(Clone, PartialEq, Eq)]
pub enum HasAI {
    True,
    False,
}

#[derive(Clone, PartialEq, Eq)]
pub enum IsDummy {
    True,
    False,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Team {
    Players,
    Monsters,
}
