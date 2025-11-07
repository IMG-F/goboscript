use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Proc {
    pub name: SmolStr,
    pub span: Span,
    pub warp: bool,
    pub is_native: bool,
}

impl Proc {
    pub fn new(name: SmolStr, span: Span, warp: bool) -> Self {
        Self { name, span, warp, is_native: false }
    }

    pub fn new_native(name: SmolStr, span: Span, warp: bool) -> Self {
        Self { name, span, warp, is_native: true }
    }
}
