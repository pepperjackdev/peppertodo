use clap::{Parser, Subcommand};

use crate::manager::task::TaskStatus;

#[derive(Parser, Debug)]
#[command(name = "peppertodo (td)", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {

    /// Adds a new task
    #[command(alias = "+")]
    Add {
        /// The title of the new task to add
        #[arg(required = true)]
        #[arg(short = 't', long = "title")]
        title: String,

        /// The description of the new task to add
        #[arg(required = true)]
        #[arg(short = 'd', long = "description")]
        description: String,
    },

    /// Lists all tasks
    #[command(alias = "ls")]
    List {
        /// If specified, only the task with a certain status will be displayed
        filter: Option<TaskStatus>
    },

    /// Marks (edits) the specified task's status
    #[command(alias = "!")]
    Mark {
        /// The title of the task to mark
        #[arg(required = true)]
        #[arg(short = 't', long = "title")]
        target: String,

        /// The new status of the task
        #[arg(required = true)]
        status: TaskStatus,
    },

    /// Edits the specified task's fileds (title and description)
    #[command(alias = "ed")]
    Edit {
        /// The title of the target task
        #[arg(required = true)]
        #[arg(long = "target")]
        target: String,

        /// The new title of the task
        #[arg(short = 't', long = "title")]
        title: Option<String>,

        /// The new description of the task
        #[arg(short = 'd', long = "description")]
        description: Option<String>,
    },

    /// Deletes the specified task
    #[command(alias = "del")]
    Delete {
        /// The title of the task to delete
        #[arg(required = true)]
        target: String,
    },

    /// Deletes all the tasks marked as done
    #[command(alias = "cls")]
    Clear
}
