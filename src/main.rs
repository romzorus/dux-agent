use duxcore::prelude::*;
use std::collections::HashMap;
use std::process::exit;

mod cliargs;
mod conf;

use crate::cliargs::{parse_cli_args_agent, CliArgsAgent};
use crate::conf::DuxConfigAgent;

fn main() {
    welcome_message_agent();

    let cliargs: CliArgsAgent = parse_cli_args_agent().unwrap();

    // Get the configuration
    let conf =
        DuxConfigAgent::from(cliargs.conf).expect("Unable to determine configuration. Abort.");

    // Only local method is handled for now. Http and Git coming soon.
    let local_tasklist_path = match cliargs.tasklist.clone() {
        Some(value) => value,
        None => match conf.source.method {
            Some(value) => match value.as_str() {
                "local" => conf.source.path.unwrap(),
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
        tasklist_get_from_file(local_tasklist_path.as_str()),
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

    let mut assignment = Assignment::from(
        correlationid.get_new_value().unwrap(), // This unwrap() is safe because initialization is checked before.
        RunningMode::Apply,
        "localhost".to_string(),
        HostHandlingInfo::from(
            ConnectionMode::LocalHost,
            "localhost".to_string(),
            ConnectionDetails::LocalHost(LocalHostConnectionDetails::current_user()),
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
