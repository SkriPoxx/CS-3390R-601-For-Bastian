mod storage;
mod tasks;
mod ui;

/// Entry point for the Rusty Tasks CLI application.
///
/// This function:
/// - loads tasks from disk,
/// - runs the main menu loop,
/// - dispatches to UI handlers,
/// - and saves tasks before exiting.
fn main() {
    // Load existing tasks or start empty if none exist
    let mut task_manager = storage::load_tasks("tasks.json");

    loop {
        ui::clear_screen();

        let pending_tasks = task_manager.pending_count();
        let completed_tasks = task_manager.completed_count();

        println!("═══════════════════════════════════════");
        println!("     🦀  RUSTY TASKS - CLI TODO APP");
        println!("═══════════════════════════════════════");
        println!();

        println!("Summary:");
        println!("  • Pending tasks   : {}", pending_tasks);
        println!("  • Completed tasks : {}", completed_tasks);
        println!();

        println!("Main menu:");
        println!("  1) 📋 View tasks");
        println!("  2) ➕ Add task");
        println!("  3) ✅ Mark task as completed");
        println!("  4) 🗑️  Delete task");
        println!("  5) 💾 Save and exit");
        println!();

        let choice = ui::read_line_trimmed("Choose an option: ");

        match choice.as_str() {
            "1" => ui::handle_view_tasks(&task_manager),
            "2" => ui::handle_add_tasks_menu(&mut task_manager),
            "3" => ui::handle_mark_completed_menu(&mut task_manager),
            "4" => ui::handle_delete_tasks_menu(&mut task_manager),
            "5" => {
                // Save tasks before exiting
                match storage::save_tasks("tasks.json", &task_manager) {
                    Ok(()) => {
                        println!("Tasks saved successfully.");
                    }
                    Err(err) => {
                        eprintln!("Failed to save tasks: {err}");
                    }
                }
                println!("Exiting program...");
                break;
            }
            _ => {
                println!("───────────────────────────────────────");
                println!("Invalid option.");
                println!("Press ENTER to return to the main menu...");
                ui::wait_for_enter();
            }
        }
    }
}
