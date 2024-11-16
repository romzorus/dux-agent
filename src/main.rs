use duxcore::prelude::*;
use std::process::exit;

mod cliargs;
mod conf;
mod connection;

use crate::cliargs::{parse_cli_args_agent, CliArgsAgent};
use crate::conf::DuxConfigAgent;

#[tokio::main]
async fn main() {

    let cliargs: CliArgsAgent = parse_cli_args_agent().unwrap();

    // Get the configuration
    let conf =
        DuxConfigAgent::from(cliargs.conf).expect("Unable to determine configuration. Abort.");

    // Git method coming soon.
    let tasklist = match cliargs.tasklist.clone() {
        Some(tasklist_path) => TaskList::from_file(tasklist_path.as_str(), TaskListFileType::Unknown).unwrap(),
        None => match conf.source.method {
            Some(value) => match value.to_lowercase().as_str() {
                "local" => TaskList::from_file(conf.source.path.unwrap().as_str(), TaskListFileType::Unknown).unwrap(),
                "http" => {
                    assert_ne!(conf.source.url, None);
                    let task_list_content = connection::http_https::http_https_get_file(conf.source.url.unwrap()).await;
                    TaskList::from_str(task_list_content.as_str(), TaskListFileType::Unknown).unwrap()
                }
                "https" => {
                    assert_ne!(conf.source.url, None);
                    let task_list_content = connection::http_https::http_https_get_file(conf.source.url.unwrap()).await;
                    TaskList::from_str(task_list_content.as_str(), TaskListFileType::Unknown).unwrap()
                    
                }
                _ => {
                    panic!("Source type value not recognized/handled.")
                }
            },
            None => {
                panic!("Missing source type field")
            }
        },
    };

    if tasklist.tasks.is_empty() {
        println!("No task in given list ({})", &cliargs.tasklist.unwrap());
        exit(0);
    }

    let connection_info = match cliargs.user {
        Some(username) => {
            match cliargs.password {
                Some(password) => {
                    HostConnectionInfo::localhost_as_user(username, Some(password))
                }
                None => {
                    HostConnectionInfo::localhost_as_user(username, None)
                }
            }
        }
        None => {
            HostConnectionInfo::localhost_current_user()
        }
    };

    let mut local_job = Job::new();

    local_job
        .set_address("localhost")
        .set_connection(connection_info).unwrap()
        .set_tasklist(tasklist);

    match local_job.apply() {
        Ok(()) => {
            println!("{}", local_job.display_pretty());
        }
        Err(error) => {
            println!("Unable to apply the tasklist : {:?}", error);
        }
    }
}

