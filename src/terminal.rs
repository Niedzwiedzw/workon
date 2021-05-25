use crate::{config::ProjectConfig, error::WorkonResult};
use std::process::Command;

pub trait RunTerminal {
    fn exec_name() -> &'static str;
    fn working_dir_arg() -> &'static str;
    fn run_command_arg() -> &'static str;
}

pub struct Alacritty;

impl RunTerminal for Alacritty {
    fn exec_name() -> &'static str {
        "alacritty"
    }

    fn working_dir_arg() -> &'static str {
        "--working-directory"
    }

    fn run_command_arg() -> &'static str {
        "-e"
    }
}

pub fn startup<T: RunTerminal>(project: &ProjectConfig) -> WorkonResult<()> {
    let mut tasks = vec![];
    for terminal_command in project.terminals.iter() {
        println!(":: starting up :: {}", project.project_name);
        let mut base_command = Command::new(T::exec_name());
        let mut args = vec![
            T::working_dir_arg().to_string(),
            terminal_command.workdir.to_string_lossy().to_string(),
            T::run_command_arg().to_string(),
        ];
        args.append(&mut terminal_command.command.clone());

        tasks.push(std::thread::spawn(move || {
            let command = base_command.args(&args[..]);
            eprintln!(
                ":: executing >> {} {}",
                command.get_program().to_string_lossy(),
                command
                    .get_args()
                    .map(|c| c.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            command.status().unwrap();
        }));
    }

    for program_command in project.programs.iter() {
        println!(":: starting up :: {}", project.project_name);
        let mut base_command = Command::new(T::exec_name());
        let mut args = vec![
            T::working_dir_arg().to_string(),
            program_command.workdir.to_string_lossy().to_string(),
            T::run_command_arg().to_string(),
        ];
        args.append(&mut program_command.command.clone());

        tasks.push(std::thread::spawn(move || {
            let command = base_command.args(&args[..]);
            eprintln!(
                ":: executing >> {} {}",
                command.get_program().to_string_lossy(),
                command
                    .get_args()
                    .map(|c| c.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            command.status().unwrap();
        }));
    }

    tasks.into_iter().for_each(|t| {
        match t.join() {
            Ok(_) => println!(":: DONE :: "),
            Err(e) => eprintln!("an error occured {:?}", e),
        }
    });

    Ok(())
}
