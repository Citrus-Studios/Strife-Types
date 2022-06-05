use serde::{Serialize, Deserialize};

use crate::ready::Ready;

#[derive(Debug, Serialize, Deserialize)]
pub struct Op0 {
    pub t: String,
    pub s: i32,
    pub d: Ready
}