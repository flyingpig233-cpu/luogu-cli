use crate::luogu::problem::LuoguProblem;
use rust_i18n::t;
use comfy_table::Table;

pub fn menu(problem: &LuoguProblem) {
    let options = vec![t!("create"), t!("more_info")];
    let ans = inquire::Select::new(t!("What do you want to do?").as_str(), options).prompt();
    match ans {
        Ok(choice) => {
            if (choice == t!("create")) {
                todo!()
            } else if (choice == t!("more_info")) {
                let mut table = Table::new();
                table.set_header(vec!["Title", "PID", "Difficulty", "Full Score", "Total Submit", "Total Accepted"]);
                table.add_row(vec![problem.title.clone(), problem.pid.clone(), format!("{}", problem.difficulty), problem.fullScore.to_string(), problem.totalSubmit.to_string(), problem.totalAccepted.to_string()]);
                println!("{}", table);
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
