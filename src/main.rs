use clap::{Parser, Subcommand};
use env_logger;
use log::info;
use pulldown_cmark;
use std::io::Read;

#[derive(Debug, Parser)]
#[command(version)]
#[command(next_line_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Initializes a new `todo.md` file in the current directory.")]
    Init,
    #[clap(about = "Adds a new task.", visible_alias("a"))]
    Add,
    #[clap(about = "Edits an existing task.", visible_alias("e"))]
    Edit {
        #[clap(help = "The number of the task to edit.")]
        number: usize,
        #[clap(short, long, help = "Set the state of the task")]
        state: Option<String>,
        #[clap(short, long, help = "Set the priority of the task")]
        priority: Option<String>,
        #[clap(short, long, help = "Add a description to the task")]
        note: Option<String>,
        #[clap(short, long, help = "Add a tag to the task")]
        tag: Option<String>,
        #[clap(short, long, help = "Add a dependency to the task")]
        depend: Option<usize>,
        #[clap(
            short,
            long,
            help = "Assign a user, or change the user assigned to the task"
        )]
        assign: Option<String>,
    },
    #[clap(
        about = "Set the status of a task to the next state.",
        visible_alias("t")
    )]
    Toggle {
        #[clap(help = "The number of the task to toggle.")]
        number: usize,
    },
    #[clap(about = "Lists all tasks.")]
    Ls,
    #[clap(about = "Show a dashboard of tasks", visible_alias("db"))]
    Dashboard,
    #[clap(about = "Search through tasks by text.", visible_alias("fd"))]
    Find {
        #[clap(help = "The text to search for.")]
        text: String,
    },
    #[clap(about = "Filter tasks.", visible_alias("f"))]
    Filter {
        #[clap(short, long, help = "Filter by state")]
        state: Option<String>,
        #[clap(short, long, help = "Filter by priority")]
        priority: Option<String>,
        #[clap(short, long, help = "Filter by tag")]
        tag: Option<String>,
        #[clap(short, long, help = "Filter by dependency")]
        depends: Option<usize>,
        #[clap(short, long, help = "Filter by user")]
        user: Option<String>,
    },
    #[clap(about = "Fix changes made by hand to the task list.")]
    Fix,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    info!("Hello, world! #{:?}", cli);

    let file = std::fs::read_to_string("local/test.md").unwrap();
    let events = pulldown_cmark::Parser::new_ext(&file, pulldown_cmark::Options::all());
    for event in events {
        println!("{:?}", event);
    }
}
