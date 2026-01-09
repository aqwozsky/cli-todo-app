use serde_json;
use std::fs;
use std::io;
fn main() {
    // Read existing ToDos from JSON file
    let data = fs::read_to_string("src/todos.json").expect("Unable to read file");
    let mut json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    loop {
        println!("Your ToDos Are:");
        // List current ToDos
        println!("{:#?}\n", &json);
        println!("Commands:\n[1] Add ToDo\n[2] Remove ToDo\n[3] View ToDos\n[4] Finish ToDo");
        let choice = get_input();

        // Instead of using if-else statements, I use match for better clarity
        match choice.as_str() {
            "1" => {
                println!("Enter your new ToDo:");
                let new_todo = get_input();
                // Logic to add new ToDo
                json.as_array_mut().unwrap().push(serde_json::json!({
                    "Todo": new_todo,
                    "Completed": "Incomplete"
                }));
                println!("Added new ToDo: {}", new_todo);

                // Save updated ToDos to file
                fs::write(
                    "src/todos.json",
                    serde_json::to_string_pretty(&json).unwrap(),
                )
                .expect("Unable to write file");
            }
            "2" => {
                println!("Enter the ToDo number to remove:");
                let remove_todo = get_input();
                // Logic to remove ToDo
                let index: usize = remove_todo.parse().unwrap_or(0);
                if index > 0 && index <= json.as_array().unwrap().len() {
                    json.as_array_mut().unwrap().remove(index - 1);

                    // Save updated ToDos to file
                    fs::write(
                        "src/todos.json",
                        serde_json::to_string_pretty(&json).unwrap(),
                    )
                    .expect("Unable to write file");
                } else {
                    println!("Invalid ToDo number.");
                }
            }
            "3" => {
                println!("Your current ToDos are:");
                // Logic to view ToDos
                println!("{:#?}\n", json);
            }

            "4" => {
                println!("Enter the ToDo number to mark as finished:");
                let finish_todo = get_input();
                // Logic to finish ToDo
                let index: usize = finish_todo.parse().unwrap_or(0);
                if index > 0 && index <= json.as_array().unwrap().len() {
                    if let Some(todo) = json.as_array_mut().unwrap().get_mut(index - 1) {
                        todo["Completed"] = serde_json::json!("Complete");
                    }

                    // Save updated ToDos to file
                    fs::write(
                        "src/todos.json",
                        serde_json::to_string_pretty(&json).unwrap(),
                    )
                    .expect("Unable to write file");
                } else {
                    println!("Invalid ToDo number.");
                }
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
