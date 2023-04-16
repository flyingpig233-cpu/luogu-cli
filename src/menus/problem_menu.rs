use std::io::Read;

use crate::luogu::problem::LuoguProblem;
use comfy_table::Table;
use rust_i18n::t;

pub fn menu(problem: &LuoguProblem) {
    loop {
        let options = vec![t!("create_solution"), t!("more_info"), t!("back")];
        let ans = inquire::Select::new(t!("What do you want to do?").as_str(), options).prompt();
        match ans {
            Ok(choice) => {
                if choice == t!("create_solution") {
                    todo!()
                } else if choice == t!("more_info") {
                    let mut table = Table::new();
                    table
                        .set_header(vec![t!("name"), t!("value")])
                        .add_row(vec![t!("title"), problem.title.clone()])
                        .add_row(vec!["PID", &problem.pid.clone()])
                        .add_row(vec![t!("difficulty"), format!("{}", problem.difficulty)])
                        .add_row(vec![t!("full_score"), problem.full_score.to_string()])
                        .add_row(vec![t!("total_submit"), problem.total_submit.to_string()])
                        .add_row(vec![
                            t!("total_accepted"),
                            problem.total_accepted.to_string(),
                        ]);
                    println!("{}", table);
                    let buffer = &mut [0u8];
                    std::io::stdin().read_exact(buffer).unwrap();
                } else if choice == t!("back") {
                    break;
                }
            }
            Err(_) => println!("There was an error, please try again"),
        }
    }
}
