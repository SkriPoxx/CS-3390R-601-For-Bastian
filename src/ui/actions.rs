use crate::tasks::{Task, TaskManager};

use super::common::{
    choose_task_index, clear_screen, print_header, print_task_list, read_line_trimmed,
    wait_for_enter, TaskSection, DAILY_SECTION, RECURRING_SECTION,
};

/// Distinguishes between recurring and daily tasks when performing actions.
enum TaskCategory {
    Recurring,
    Daily,
}

/// Represents the type of action to perform on a selected task.
enum TaskAction {
    Complete,
    Delete,
}

fn get_tasks<'a>(manager: &'a TaskManager, category: &TaskCategory) -> &'a [Task] {
    match category {
        TaskCategory::Recurring => manager.recurring_tasks(),
        TaskCategory::Daily => manager.daily_tasks(),
    }
}

/// Performs an action (complete or delete) on a task selected by the user.
///
/// This function:
/// - prints the tasks in the chosen category
/// - asks the user for an index
/// - calls the corresponding `TaskManager` method
/// - prints the result or an error message
fn perform_task_action(
    manager: &mut TaskManager,
    category: TaskCategory,
    section: &TaskSection,
    action: TaskAction,
    prompt: &str,
) {
    let len = match category {
        TaskCategory::Recurring => manager.recurring_len(),
        TaskCategory::Daily => manager.daily_len(),
    };

    if len == 0 {
        println!("You don't have any {} tasks yet.", section.name);
        println!("Press ENTER to continue...");
        wait_for_enter();
        return;
    }

    clear_screen();
    print_header(section.header);

    {
        let tasks = get_tasks(manager, &category);
        print_task_list(tasks);
    }

    if let Some(index) = choose_task_index(prompt, len) {
        match action {
            TaskAction::Complete => match category {
                TaskCategory::Recurring => match manager.mark_recurring_completed(index) {
                    Ok(()) => {
                        let title = &manager.recurring_tasks()[index].title;
                        println!("✅ Task '{}' marked as completed.", title);
                    }
                    Err(err) => println!("Error: {}", err),
                },
                TaskCategory::Daily => match manager.mark_daily_completed(index) {
                    Ok(()) => {
                        let title = &manager.daily_tasks()[index].title;
                        println!("✅ Task '{}' marked as completed.", title);
                    }
                    Err(err) => println!("Error: {}", err),
                },
            },
            TaskAction::Delete => match category {
                TaskCategory::Recurring => match manager.delete_recurring(index) {
                    Ok(removed) => {
                        println!("🗑️  Task '{}' has been deleted.", removed.title);
                    }
                    Err(err) => println!("Error: {}", err),
                },
                TaskCategory::Daily => match manager.delete_daily(index) {
                    Ok(removed) => {
                        println!("🗑️  Task '{}' has been deleted.", removed.title);
                    }
                    Err(err) => println!("Error: {}", err),
                },
            },
        }

        println!("Press ENTER to continue...");
        wait_for_enter();
    }
}

/// Shows a menu that allows the user to mark tasks as completed.
///
/// The user can choose between recurring and daily tasks and then
/// select the specific task to mark as done.
pub fn handle_mark_completed_menu(manager: &mut TaskManager) {
    loop {
        clear_screen();
        print_header("✅ MARK TASK AS COMPLETED");
        println!("Choose which type of task you want to mark as completed:");
        println!();
        println!("  1) 🔁 Recurring task");
        println!("  2) 📅 Daily task");
        println!("  3) ⬅️  Back to main menu");
        println!();

        let choice = read_line_trimmed("Choose an option: ");

        match choice.as_str() {
            "1" => perform_task_action(
                manager,
                TaskCategory::Recurring,
                &RECURRING_SECTION,
                TaskAction::Complete,
                "Enter the number of the recurring task to mark as completed (or empty to cancel): ",
            ),
            "2" => perform_task_action(
                manager,
                TaskCategory::Daily,
                &DAILY_SECTION,
                TaskAction::Complete,
                "Enter the number of the daily task to mark as completed (or empty to cancel): ",
            ),
            "3" => {
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

/// Shows a menu that allows the user to delete tasks.
///
/// The user can choose between recurring and daily tasks and then
/// select the specific task to delete.
pub fn handle_delete_tasks_menu(manager: &mut TaskManager) {
    loop {
        clear_screen();
        print_header("🗑️  DELETE TASK");
        println!("Choose which type of task you want to delete:");
        println!();
        println!("  1) 🔁 Recurring task");
        println!("  2) 📅 Daily task");
        println!("  3) ⬅️  Back to main menu");
        println!();

        let choice = read_line_trimmed("Choose an option: ");

        match choice.as_str() {
            "1" => perform_task_action(
                manager,
                TaskCategory::Recurring,
                &RECURRING_SECTION,
                TaskAction::Delete,
                "Enter the number of the recurring task to DELETE (or empty to cancel): ",
            ),
            "2" => perform_task_action(
                manager,
                TaskCategory::Daily,
                &DAILY_SECTION,
                TaskAction::Delete,
                "Enter the number of the daily task to DELETE (or empty to cancel): ",
            ),
            "3" => {
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
