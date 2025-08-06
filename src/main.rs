use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("Welcome to the CLI based To-Do List");
        println!("___________________________");
        println!("Please select the operation you want to do:");
        println!("Enter + to add a task");
        println!("Enter - to mark a task as done");
        println!("Enter exit to quit the program");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();

        match choice {
            "+" => task_add(&mut tasks),
            "-" => task_done(&mut tasks),
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("You have entered an invalid choice."),
        }
    }
}

fn task_add(tasks: &mut Vec<String>) {
    println!("Please enter the name of the task:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read task");
    let task = task.trim();
    if !task.is_empty() {
        tasks.push(task.to_string());
        println!("Task added: {}", task);
    } else {
        println!("Empty task not added.");
    }
}

fn task_done(tasks: &mut Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks to mark done!");
        return;
    }

    println!("Current tasks:");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {}", i + 1, task);
    }

    println!("Enter the task number to mark as done:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    match input.parse::<usize>() {
        Ok(num) if num > 0 && num <= tasks.len() => {
            let removed = tasks.remove(num - 1);
            println!("Task '{}' marked as done and removed.", removed);
        }
        _ => println!("Invalid task number."),
    }
}
