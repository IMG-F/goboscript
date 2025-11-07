use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::misc::SmolStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtMenuType {
    Menu,
    MenuInput { menu_name: SmolStr },
    NONE
}

impl ExtMenuType {
    pub fn is_menu(&self) -> bool {
        matches!(self, Self::Menu)
    }

    pub fn is_menu_input(&self) -> bool {
        matches!(self, Self::MenuInput { .. })
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::NONE)
    }

    pub fn new_menu_input(menu_name: SmolStr) -> Self {
        Self::MenuInput { menu_name }
    }
}

impl Display for ExtMenuType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExtMenuType::Menu => write!(f, "Menu"),
            ExtMenuType::MenuInput { menu_name: _ } => write!(f, "MenuInput"),
            ExtMenuType::NONE => write!(f, ""),
        }
    }
}