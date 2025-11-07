use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{type_::Type, ConstExpr, ExtMenuType};
use crate::misc::SmolStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    pub name: SmolStr,
    pub span: Span,
    pub type_: Type,
    pub default: Option<ConstExpr>,
    pub is_used: bool,
    pub ext_menu_type: ExtMenuType,
}

impl Arg {
    pub fn new(name: SmolStr, span: Span, type_: Type, default: Option<ConstExpr>) -> Self {
        Self::new_with_menu(
            name,
            span,
            type_,
            default,
            ExtMenuType::NONE,
        )
    }

    pub fn new_with_menu (name: SmolStr, span: Span, type_: Type, default: Option<ConstExpr>, ext_menu_type: ExtMenuType) -> Self {
        Self {
            name,
            span,
            type_,
            default,
            is_used: false,
            ext_menu_type,
        }
    }
}
