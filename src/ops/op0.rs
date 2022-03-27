use serde::{Serialize, Deserialize};

use crate::ready::Ready;

#[derive(Debug, Serialize, Deserialize)]
pub struct Op0 {
    t: String,
    s: i32,
    d: Ready
}