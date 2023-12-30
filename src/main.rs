mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

/// Simple Todo tasks manager from cli
fn find_default_journal_file() -> Option<PathBuf> {
    // map work only if Some. If none -> do nothing
    home::home_dir().map(|mut path| {
        path.push(".rusty_journal.json");

        path
    })
}

fn main() {
    // cli::CommandLineArgs::from_args();
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // uppack the journal file
    let journal_file = journal_file
        // work only if None
        .or_else(find_default_journal_file)
        .expect("Journal file not found");

    // perform the actions in tasks
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perform action")
}
