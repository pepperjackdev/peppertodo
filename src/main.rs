use clap::{command, Arg, ArgAction};
use todo::{manager::TaskManager, task::Task};

fn main() {
    let mut manager = TaskManager::new();

    let matches = command!()
        .arg(
            Arg::new("title")
                .long("add")
                .short('a')
                .value_name("title")
                .help("The title of the new task")
                .required(true)
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .value_name("description")
                .help("The description of the new task")
                .required(false)   
        )
        .get_matches();

    if let Some(title) = matches.get_one::<String>("title") {
        let description = matches.get_one::<String>("description");
        manager.add_new_task(Task::new(&title, &description));
    }
}