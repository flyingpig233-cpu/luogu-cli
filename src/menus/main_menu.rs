use super::search_menu;
use inquire::*;
use rust_i18n::t;

#[derive(Debug)]
enum MenuItems {
    View,
    Search,
    Exit,
}

pub fn menu() {
    let options: Vec<String> = vec![t!("view"), t!("search"), t!("exit")];
    let enum_options: Vec<MenuItems> = vec![MenuItems::View, MenuItems::Search, MenuItems::Exit];
    let prompt = t!("What do you want to do?");
    loop {
        let options_copy = options.clone();
        let options_copy = options_copy.iter().map(|elem| elem.as_str()).collect();
        let select_prompt = Select::new(&prompt, options_copy);
        let ans: Result<&str, InquireError> = select_prompt.prompt();
        match ans {
            Ok(choice) => {
                let choice = &enum_options[options
                    .iter()
                    .position(|r| r.as_str().cmp(&choice).is_eq())
                    .unwrap()];
                match choice {
                    MenuItems::View => {}
                    MenuItems::Search => {
                        search_menu::menu();
                    }
                    MenuItems::Exit => {
                        return;
                    }
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }
}
