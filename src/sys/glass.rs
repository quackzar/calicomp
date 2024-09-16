use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Glassware {
    Lowball,
    Highball,
    Martini,
    NickAndNora,
    Hurricane,
    Tiki,
    Wine,
}
