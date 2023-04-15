use rust_i18n::t;
use super::problem_menu;
use crate::luogu::problem::LuoguProblem;

fn send_request(keyword: String) -> Result<serde_json::Value, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://www.luogu.com.cn/problem/list?type=P&keyword={}", keyword))
        .header("x-luogu-type", "content-only")
        .send()?.json()?;
    Ok(response)
}

pub fn menu() {
    let keyword = inquire::Text::new(t!("search_problem").as_str()).prompt().unwrap();
    let result = send_request(keyword);
    
    match result {
        Ok(body) => {
            let problems_data = body["currentData"]["problems"].as_object().unwrap();
            let mut problems: Vec<LuoguProblem> = vec![];
            for problem in problems_data["result"].as_array().unwrap() {
                let luoguProblem: LuoguProblem = serde_json::from_value(problem.clone()).unwrap();
                problems.push(luoguProblem);
            }
            let problem_titles: Vec<&str> = problems.iter().map(|elem| elem.title.as_str()).collect();
            let problem_titles_n = problem_titles.clone();
            let ans = inquire::Select::new("Here are the list of problems", problem_titles).prompt();
            match ans {
                Ok(choice) => {
                    let selected_problem = &problems[problem_titles_n.iter().position(|elem| elem.eq(&choice)).unwrap()];
                    problem_menu::menu(selected_problem);
                    
                }
                Err(_) => println!("There was an error, please try again"),
            }
            
        }
        Err(err) => {
            eprintln!("Failed to send request: {:?}", err);
        }
    }
    
}