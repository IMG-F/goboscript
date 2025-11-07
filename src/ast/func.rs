use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::Type;
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Func {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub is_native: bool,
}

impl Func {
    pub fn new(name: SmolStr, span: Span, type_: Type) -> Self {
        Self { name, span, type_, is_native: false }
    }

    pub fn new_native(name: SmolStr, span: Span, type_: Type) -> Self {
        Self { name, span, type_, is_native: true }
    }
}
