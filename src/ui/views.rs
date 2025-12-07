use crate::tasks::TaskManager;

use super::common::{
    clear_screen, print_header, print_task_list, print_tasks_or_examples, read_line_trimmed,
    wait_for_enter, DAILY_SECTION, RECURRING_SECTION,
};

/// Shows all recurring and daily tasks to the user.
///
/// This function only reads from the `TaskManager` and does not modify it.
pub fn handle_view_tasks(manager: &TaskManager) {
    clear_screen();
    print_header("📋 YOUR TASKS");

    let recurring_tasks = manager.recurring_tasks();
    let daily_tasks = manager.daily_tasks();

    if recurring_tasks.is_empty() && daily_tasks.is_empty() {
        println!("There are no tasks yet.");
        println!("Add a task and then come back to this menu.");
    } else {
        print_header(RECURRING_SECTION.header);
        print_task_list(recurring_tasks);
        println!();
        print_header(DAILY_SECTION.header);
        print_task_list(daily_tasks);
    }

    println!();
    println!("Press ENTER to go back to the main menu...");
    wait_for_enter();
}

/// Displays the "Add task" menu and lets the user:
/// - view existing recurring or daily tasks
/// - add a new recurring or daily task
///
/// This function mutates the `TaskManager` based on user choices.
pub fn handle_add_tasks_menu(manager: &mut TaskManager) {
    loop {
        clear_screen();

        let recurring_count = manager.recurring_len();
        let daily_count = manager.daily_len();

        print_header("➕ ADD TASK MENU");

        println!("Task types:");
        println!("  🔁 Recurring tasks");
        println!("     • These tasks repeat regularly.");
        println!("     • Example: Go to school (Mon–Fri), Gym, Read Bible, etc.");
        println!();
        println!("  📅 Daily tasks");
        println!("     • These tasks are one-time or specific to a day.");
        println!("     • Example: Doctor appointment, Call mom, Job interview, etc.");
        println!();

        println!("Current status:");
        println!("  🔁 Recurring tasks: {}", recurring_count);
        println!("  📅 Daily tasks    : {}", daily_count);
        println!();

        println!("What would you like to do?");
        println!();
        println!("  1) 🔍 View recurring tasks");
        println!("  2) 🔍 View daily tasks");
        println!("  3) ➕ Add a recurring task");
        println!("  4) ➕ Add a daily task");
        println!("  5) ⬅️  Back to main menu");
        println!();

        let choice = read_line_trimmed("Choose an option: ");

        match choice.as_str() {
            "1" => {
                clear_screen();
                print_header(RECURRING_SECTION.header);
                print_tasks_or_examples(&RECURRING_SECTION, manager.recurring_tasks());
                println!();
                println!("Press ENTER to return to the Add Task menu...");
                wait_for_enter();
            }
            "2" => {
                clear_screen();
                print_header(DAILY_SECTION.header);
                print_tasks_or_examples(&DAILY_SECTION, manager.daily_tasks());
                println!();
                println!("Press ENTER to return to the Add Task menu...");
                wait_for_enter();
            }
            "3" => {
                clear_screen();
                print_header("➕ ADD RECURRING TASK");
                println!("Examples:");
                println!("  • Go to school");
                println!("  • Gym");
                println!("  • Read Bible");
                println!("  • Study Rust");
                println!();

                let title = read_line_trimmed("Enter the recurring task title (empty to cancel): ");

                if title.is_empty() {
                    println!("No task added (empty title).");
                } else {
                    manager.add_recurring(title);
                    println!("✅ Recurring task added successfully.");
                }

                println!();
                println!("Press ENTER to return to the Add Task menu...");
                wait_for_enter();
            }
            "4" => {
                clear_screen();
                print_header("➕ ADD DAILY TASK");
                println!("Examples:");
                println!("  • Doctor appointment");
                println!("  • Call mom");
                println!("  • Job interview");
                println!("  • Pay bills");
                println!();

                let title = read_line_trimmed("Enter the daily task title (empty to cancel): ");

                if title.is_empty() {
                    println!("No task added (empty title).");
                } else {
                    manager.add_daily(title);
                    println!("✅ Daily task added successfully.");
                }

                println!();
                println!("Press ENTER to return to the Add Task menu...");
                wait_for_enter();
            }
            "5" => {
                println!("Returning to main menu...");
                println!();
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
                println!("Press ENTER to continue...");
                wait_for_enter();
            }
        }
    }
}
