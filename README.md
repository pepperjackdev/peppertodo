# peppertodo (pt)
A to-do list command line utlity written in Rust.

> [!WARNING]
> The utility is still in development an is really unstable! 

## Table of contents
- [Build with](#build-with)
- [Intallation](#installation)
- [Getting Started](#getting-started)
    - [A task's structure](#a-tasks-structure)
    - [Adding a new task](#adding-a-new-task-add-)
    - [Listing tasks](#listing-tasks-list-ls)
    - [Marking a task's status](#marking-a-tasks-status-mark-)
    - [Editing a task's fields](#editing-a-tasks-title-or-description-edit-ed)
    - [Deleting a task](#deleting-a-task-delete-del)

## Build with
- [clap](https://crates.io/crates/clap): a simple, efficient and full-featured command line argument parser.
- [dirs](https://crates.io/crates/dirs): library that provides platform-specific standard locations of directories for config, cache and other data on Linux, Windows, macOS and Redox.
- [rusqlite](https://crates.io/crates/rusqlite): an ergonomic wrapper for SQLite's DBMS.

## Installation
You can install this utility via `cargo`:
    
    $ cargo install peppertodo

After the installation succeeded, you should be able to run the utility by using its bin name:

    $ pt 

Note that all the tasks are saved into an SQLite database (`appdata.db`) under the `peppertodo` folter, into your `data-dir`. For more informations about the effective path of the `data-dir` on your platform, just visit the `dirs`'s crate docs [here](https://docs.rs/dirs/5.0.1/dirs/fn.data_dir.html).

Here a short summary:

| Platform | Data directory path                      |
|----------|------------------------------------------|
| Linux    | /home/alice/.local/share                 |
| macOs    | /Users/Alice/Library/Application Support |
| Windows  | C:\Users\Alice\AppData\Roaming           |

## Getting started
If you're trying to get started with the utility, this is the right place!

### A task's structure

A task is has three main components:
- A title: used to briefly describe (i.e., a title) and target (meaning that it behaves like an ID) a task.
- A description: used to give more details about the task.
- A status: it could be either `undone`, `underway` or `done`.

### Adding a new task: `add` (`+`)
To add a new task, just run:

    $ pt add -t "Task title" -d "task description"

As you run this command, a new task with `title` "_Task title_" and a `description`
"_task description_" is added to your tasks. 

You can alternatively use its alias, `+`:

    $ pt + -t "Task title" -d "task description"

This does the same thing as the previous command.

Just remeber that, as a task's title is what allows you to target one specific task, it is recomended to choose only short titles for your tasks, leaving all the details for the description.

### Listing tasks: `list` (`ls`)
To list all the task you've added, use `list`:

    $ pt list
    [undone] Task title: task description

If you want to list _only_ tasks with a certain status, just put the status you're interested in after the `list` command:

    $ pt list <status>

For example

    $ pt list undone
    [undone] Task title: task description
    $ pt list done
    $ # nothing has been display as no task with done status exists

Also `list` has its short-hand alias: `ls`.

### Marking a task's status: `mark` (`!`)
To edit a task status (i.e. mark), use `mark`:

    $ pt mark -t "Task title" done

Now, the task with title "Task title" has been marked as `done`:

    $ pt ls
    [done] Task title: task description

The short-hand alias for `mark` is `!`.

    $ pt ! -t "Task title" done

### Editing a task's title or description: `edit` (`ed`)
To edit a task's `title` or `description` ther's `edit`

    $ pt edit --target "Task title" -t "New task title" -d "new task description"
    $ pt edit --target "New task title" -d "new new task description"
    $ pt edit --target "New task title" -t "Task title"

If you want to edit quicker, just use its alias: `ed`.

### Deleting a task: `delete` (`del`)
To delete a task, use:

    $ pt delete -t "Task title"

or its short hand version:

    $ pt del -t "Task title"