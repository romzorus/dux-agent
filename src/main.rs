use duxcore::connection::connectionmode::localhost::WhichUser;
use duxcore::prelude::*;
use std::collections::HashMap;
use std::process::exit;

mod cliargs;
mod conf;
mod connection;

use crate::cliargs::{parse_cli_args_agent, CliArgsAgent};
use crate::conf::DuxConfigAgent;

#[tokio::main]
async fn main() {
    welcome_message_agent();

    let cliargs: CliArgsAgent = parse_cli_args_agent().unwrap();

    // Get the configuration
    let conf =
        DuxConfigAgent::from(cliargs.conf).expect("Unable to determine configuration. Abort.");

    // Only local method is handled for now. Http and Git coming soon.
    let tasklist_content = match cliargs.tasklist.clone() {
        Some(value) => value,
        None => match conf.source.method {
            Some(value) => match value.to_lowercase().as_str() {
                "local" => tasklist_get_from_file(conf.source.path.unwrap().as_str()),
                "http" => {
                    assert_ne!(conf.source.url, None);
                    connection::http_https::http_https_get_file(conf.source.url.unwrap()).await
                }
                "https" => {
                    assert_ne!(conf.source.url, None);
                    connection::http_https::http_https_get_file(conf.source.url.unwrap()).await
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

    let tasklist = tasklist_parser(
        tasklist_content,
        &Host::from_string("localhost".to_string()),
    );

    if tasklist.tasks.is_empty() {
        println!("No task in given list ({})", &cliargs.tasklist.unwrap());
        exit(0);
    }

    let mut correlationid = CorrelationIdGenerator::new();
    match correlationid.init() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: failure to initialize CorrelationId");
            println!("{:?}", e);
            exit(1);
        }
    }

    let connection_details = match cliargs.user {
        Some(username) => {
            match cliargs.password {
                Some(password) => {
                    LocalHostConnectionDetails::user(
                        WhichUser::UsernamePassword(
                            Credentials {
                                username,
                                password
                            }
                        )
                    )
                }
                None => {
                    LocalHostConnectionDetails::user(
                        WhichUser::PasswordLessUser(username)
                    )
                }
            }
        }
        None => {
            LocalHostConnectionDetails::current_user()
        }
    };

    let mut assignment = Assignment::from(
        correlationid.get_new_value().unwrap(), // This unwrap() is safe because initialization is checked before.
        RunningMode::Apply,
        "localhost".to_string(),
        HostHandlingInfo::from(
            ConnectionMode::LocalHost,
            "localhost".to_string(),
            ConnectionDetails::LocalHost(connection_details),
        ),
        HashMap::new(),
        tasklist.clone(),
        ChangeList::new(),
        ResultList::new(),
        AssignmentFinalStatus::Unset,
    );

    let mut hosthandler = HostHandler::from(&assignment.hosthandlinginfo).unwrap();

    let _ = hosthandler.init();

    let _ = assignment.dry_run(&mut hosthandler);
    if let AssignmentFinalStatus::Unset = assignment.finalstatus {
        assignment.apply(&mut hosthandler);
    }

    display_output(assignment);
}

pub fn welcome_message_agent() {
    println!(
        r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ 
    🅰🅶🅴🅽🆃                                
"
    );
}
