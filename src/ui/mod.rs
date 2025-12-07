pub mod actions;
pub mod common;
pub mod views;

pub use actions::{handle_delete_tasks_menu, handle_mark_completed_menu};
pub use common::{clear_screen, read_line_trimmed, wait_for_enter};
pub use views::{handle_add_tasks_menu, handle_view_tasks};
