use std::io;
#[allow(unused_variables)]
struct Task {
    task_desc: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new(); // tasks is a vector of type Task

    println!("To-Do List");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Finish Task");
    println!("4. Delete Task");
    println!("5. Exit");

    let mut input: String = String::new();

    let _ = io::stdin().read_line(&mut input);

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return
        }
    };

    match input {
        1 => add_task(&mut tasks),
        2 => list_task(&tasks),
        3 => finish_task(&mut tasks),
        4 => delete_task(&mut tasks),
        5 => println!("Goodbye!"),
        _ => println!("Invalid Choice!"),
    }
}

fn add_task(tasks: &mut Vec<Task>) -> () {
    let mut desc: String= String::new();
    println!("Enter task:");

    let _ = io::stdin().read_line(&mut desc).unwrap();

    // let mut input: String = desc.trim();

    tasks.push(Task {
        task_desc: desc,
        completed: false,
    });
}

fn list_task(tasks: &[Task]) -> () {
    for (i, task) in tasks.iter().enumerate() {
        match task.completed {
            true => println!("{}. [x]: {}", i + 1, task.task_desc),
            false => println!("{}. [ ]: {}", i + 1, task.task_desc),
        }
    }
}

fn finish_task(tasks: &mut Vec<Task>) -> () {
    list_task(&tasks);

    println!("Enter task number to mark as completed:");

    let mut input: String = String::new();

    let _ = io::stdin().read_line(&mut input);

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return
        }
    };

    if input > 0 && (input as usize) <= tasks.len() {
        if let Some(task) = tasks.get_mut(input as usize - 1) {
            task.completed = true;
        }
    } else {
        println!("Invalid");
    }
}

fn delete_task(tasks: &mut Vec<Task>) -> () {
    list_task(&tasks);

    println!("Enter task number to delete:");

    let mut input: String = String::new();

    let _ =  io::stdin().read_line(&mut input);

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return
        }
    };

    if input as usize <= tasks.len() && input>0{
        tasks.remove(input as usize - 1);
        println!("Task removed");
    } else {
        println!("Invalid");
    }
}