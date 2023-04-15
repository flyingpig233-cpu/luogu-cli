use super::arg::Args;
use crate::menus::*;

pub struct LuoguCLI {
    args: Args,
}

impl LuoguCLI {
    pub fn from(args: Args) -> Self {
        Self { args }
    }
    
    pub fn run(self) {
        main_menu::menu();
    }
}
