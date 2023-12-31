use std::{collections::HashMap, process::ExitCode};

use clap::Parser;
use days::day_1;
use days::day_2;
use days::day_3;
use days::day_4;
use days::day_5;

pub mod prelude;
mod utils;
use crate::cli::TaskOverCli;
mod days;

mod cli;
type TaskHandlers = HashMap<(usize, usize), Handler>;
type Handler = fn(String) -> String;
fn main() -> ExitCode {
    let args = TaskOverCli::parse();
    let mut tasks_handlers: TaskHandlers = Default::default();
    register_handler(&mut tasks_handlers, 1, 1, day_1::handle);
    register_handler(&mut tasks_handlers, 1, 2, day_1::handle_task_2);
    register_handler(&mut tasks_handlers, 2, 1, day_2::handle_task);
    register_handler(&mut tasks_handlers, 2, 2, day_2::handle_task_2);
    register_handler(&mut tasks_handlers, 3, 1, day_3::handle_task);
    register_handler(&mut tasks_handlers, 3, 2, day_3::handle_task_2);
    register_handler(&mut tasks_handlers, 4, 1, day_4::handle_task);
    register_handler(&mut tasks_handlers, 4, 2, day_4::handle_task_2);
    register_handler(&mut tasks_handlers, 5, 1, day_5::handle_task);
    register_handler(&mut tasks_handlers, 5, 2, day_5::handle_task_2);
    register_handler(&mut tasks_handlers, 5, 3, day_5::handle_task_3);

    match tasks_handlers.get(&(args.day, args.task)) {
        Some(handler) => {
            let result = handler(args.input.content);
            println!("{}", result);
            ExitCode::SUCCESS
        }
        None => {
            eprintln!(
                "No function registered to handle task {} under day {}",
                args.task, args.day
            );
            ExitCode::FAILURE
        }
    }
}
fn register_handler(handlers: &mut TaskHandlers, day: usize, task: usize, handler: Handler) {
    assert!(
        handlers.insert((day, task), handler).is_none(),
        "Registered more than one task hanlder to task {} under the day {}",
        task,
        day
    );
}
