use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Spirit {
    Gin,
    Mezcal,
    Vodka,
    Rum,
    Whiskey(Whiskey),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Whiskey {
    Scotch,
    Irish,
    Bourbon,
}
