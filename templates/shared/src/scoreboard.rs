use crate::*;
use serde::{Serialize, Deserialize};
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ScoreBoard{
  pub a: u32,
  pub b: u32
}