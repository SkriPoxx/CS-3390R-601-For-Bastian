use crate::tasks::Task;
use std::io::{self, Write};

/// Prints a prompt, reads a line from standard input, and returns it trimmed.
///
/// This helper is used throughout the UI layer to collect user input
/// in a consistent way.
pub fn read_line_trimmed(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if let Err(err) = io::stdin().read_line(&mut input) {
        eprintln!("Error reading input: {err}");
        return String::new();
    }

    input.trim().to_string()
}

/// Waits until the user presses ENTER.
///
/// Useful for pausing the UI before returning to a previous menu.
pub fn wait_for_enter() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
}

/// Clears the terminal screen using ANSI escape codes.
///
/// This improves readability by showing only the current menu or view.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect("Failed to flush stdout");
}

/// Prints a simple decorated header with a title.
///
/// Used to separate sections visually in the CLI.
pub fn print_header(title: &str) {
    println!("───────────────────────────────────────");
    println!("{title}");
    println!("───────────────────────────────────────");
    println!();
}

/// Prints a numbered list of tasks with a simple [ ] / [x] status indicator.
pub fn print_task_list(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("(no tasks)");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "[x]" } else { "[ ]" };
        println!("  {}. {} {}", index + 1, status, task.title);
    }
}

/// Asks the user to select a task by its 1-based index and returns a 0-based index.
///
/// If the input is empty, invalid, or out of range, this function prints
/// an error message and returns `None`.
pub fn choose_task_index(prompt: &str, tasks_len: usize) -> Option<usize> {
    println!();
    let input = read_line_trimmed(prompt);

    if input.is_empty() {
        println!("No task selected.");
        println!("Press ENTER to continue...");
        wait_for_enter();
        return None;
    }

    let idx_user = match input.parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number.");
            println!("Press ENTER to continue...");
            wait_for_enter();
            return None;
        }
    };

    if idx_user == 0 || idx_user > tasks_len {
        println!("Number out of range.");
        println!("Press ENTER to continue...");
        wait_for_enter();
        return None;
    }

    Some(idx_user - 1)
}

/// Describes how a group of tasks (recurring or daily) should be presented.
///
/// Each section has a name, a header, an empty-state message, and some examples.
pub struct TaskSection<'a> {
    pub name: &'a str,
    pub header: &'a str,
    pub empty_msg: &'a str,
    pub examples: &'a [&'a str],
}

pub const RECURRING_SECTION: TaskSection<'static> = TaskSection {
    name: "recurring",
    header: "🔁 RECURRING TASKS",
    empty_msg: "You don't have any recurring tasks yet.",
    examples: &["Go to school", "Gym", "Read Bible"],
};

pub const DAILY_SECTION: TaskSection<'static> = TaskSection {
    name: "daily",
    header: "📅 DAILY TASKS",
    empty_msg: "You don't have any daily tasks yet.",
    examples: &["Doctor appointment", "Call mom", "Job interview"],
};

/// Prints all tasks in a section, or an empty-state message with examples.
///
/// This is used in the "Add task" menu to show what the user currently has
/// and suggest potential tasks to add.
pub fn print_tasks_or_examples(section: &TaskSection, tasks: &[Task]) {
    if tasks.is_empty() {
        println!("{}", section.empty_msg);
        if !section.examples.is_empty() {
            println!("Examples you might add:");
            for ex in section.examples {
                println!("  • {ex}");
            }
        }
    } else {
        println!("You currently have {} task(s):", tasks.len());
        println!();
        for (index, task) in tasks.iter().enumerate() {
            let status = if task.completed { "[x]" } else { "[ ]" };
            println!("  {}. {} {}", index + 1, status, task.title);
        }
    }
}
