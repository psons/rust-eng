/*
eng project main
    Command line version End Goal App for prioritizing goals and tasks

Clap docs: https://docs.rs/clap/latest/clap/
Clap Demo from: https://docs.rs/clap/latest/clap/_cookbook/git/index.html
Serde Demo fom: https://serde.rs/
 */

mod eg_cmd;
mod eg_fs;
mod eg_shape;

use std::ffi::{OsStr, OsString};
// use std::fmt::Error;
use std::error::Error;

// use std::fmt::Error;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{arg, Arg, ArgMatches, Command, Parser}; // unused: ArgGroup
use eg_cmd::{do_eg_addtask, do_eg_goal_location, do_eg_objective_location};
use eg_shape::got::{status_opt_from_str, Status};
// use serde::de::Error;

pub trait Runner {
    fn run(self) -> Result<(), Box<dyn Error>>;
}

// consider eyre error  and error result instead of box dyn error.

/// Command line version End Goal App for decomposing goals to achievable objectives and tasks
#[derive(Parser)]
pub enum Cmd {
    Init(eg_cmd::Init),
    Load(eg_cmd::Load), // TODO - in_progress
    #[command(subcommand)]
    Add(eg_cmd::Add),
    //Update(eg_cmd::Update),
    //Delete(eg_cmd::Delete),
    Location(eg_cmd::Location),
    List(eg_cmd::List),
}

impl Runner for Cmd {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Cmd::Init(inner) => inner.run(),
            Cmd::Load(inner) => inner.run(),
            Cmd::Add(inner) => inner.run(),
            Cmd::Location(inner) => inner.run(),
            Cmd::List(inner) => inner.run(),
        }
    }
}

// fn cli() -> Command {
//     Command::new("eng")
//         .about("Command line version End Goal App for decomposing goals to achievable objectives \
//                 and tasks")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         // .allow_external_subcommands(true) // see comment in main
//         .subcommand(
//             Command::new("update")
//                 .about("PARSES CMD, BUT NOT IMPLEMENTED: update a Goal, Objective, or Task")
//                 .args(location_args())
//                     // need to be able to address a task for update, which should search all g and t for the task_hash
//                     // make task update be its own sub command?
//                 .arg( Arg::new("n").short('n').long("name") )
//                 .arg( Arg::new("d").short('d').long("detail") )
//                 .arg( Arg::new("m").short('m').long("max") )
//                 .arg_required_else_help(false)
//         )
//         .subcommand(
//             Command::new("delete")
//                 .about("PARSES CMD, BUT NOT IMPLEMENTED: delete a task or an empty goal or objective")
//                 .arg(arg!(<HASH> "Hash Code to find and delete"))
//                 // [g|goal|o|objective|s|sub|subobjective|t|task] [object_path]
//                 .arg_required_else_help(false)
//         )
// }

fn location_args() -> Vec<clap::Arg> {
    vec![
        Arg::new("g").short('g').long("goal"),
        Arg::new("o").short('o').long("objective"),
    ]
}

fn need_better_error_handling(message: String) {
    panic!("{}", message)
    // todo.  change to panic to an error return that doesn't look like a program bug
}

fn main() -> Result<(), Box<dyn Error>> {
    Cmd::parse().run()
}

// fn main_old() -> Result<(), Box<dyn Error>> {
//     let matches = cli().get_matches();
//     match matches.subcommand() {
//         Some(("init", sub_matches)) => {
//             let domain_name = sub_matches
//                 .get_one::<String>("DOMAIN")
//                 .expect("An effort domain name is required");
//             do_eg_init(domain_name)?;
//         }
//
//         Some(("load", sub_matches)) => {
//             let path: &OsStr = sub_matches
//                 .get_one::<PathBuf>("PATH")
//                 .map(|s| s.as_os_str())
//                 // no method named `as_str` ... method with a similar name .map(|s| s.as_os_str())
//                 .expect("Default list type is 'all'");
//             // println!("load: will destroy existing data in a file place known in eg_fs.rs and load from {path:?} ");
//             println!(
//                 "load: will destroy existing data in the domain store and load from {path:?} "
//             );
//             do_eg_load(path)?;
//         }
//
//         Some(("addgoal", sub_matches)) => {
//             println!("\tAdd goal:");
//             let name = sub_matches.get_one::<String>("n").expect("required");
//             println!("\t\tname from -n: {name}");
//
//             let max_objectives = process_m_arg(sub_matches);
//             do_eg_addgoal(name, max_objectives)?;
//         }
//
//         // a goal test gid: 647d2a0600007e0e0030c38e
//         Some(("addobjective", sub_matches)) => {
//             println!("\tAdd objective:");
//             let name = sub_matches.get_one::<String>("n").expect("required");
//             println!("\t\tname from -n: {name}");
//
//             let max_tasks = process_m_arg(sub_matches);
//
//             let goal_option: Option<&String>;
//             if let Some(goal_id_temp) = sub_matches.get_one::<String>("g") {
//                 println!("\t\tlocation -g: {goal_id_temp}");
//                 goal_option = Some(goal_id_temp);
//                 // error returned later if -g gid is not found in the domain
//             } else {
//                 goal_option = None;
//                 println!("\t\tno goal location specified.  A pre-specified location will be used.")
//             }
//             do_eg_addobjective(name, max_tasks, goal_option)?;
//         }
//
//         // an objective test oid: 647d2a060053d2d20030c38f
//         Some(("addtask", sub_matches)) => {
//             println!("\tAdd task:");
//             let name = sub_matches.get_one::<String>("n").expect("required");
//             println!("\t\tname from -n: {name}");
//
//             let default_status = Status::todo;
//             let mut task_status = default_status;
//             if let Some(status_s) = sub_matches.get_one::<String>("s") {
//                 task_status = match status_opt_from_str(status_s.as_str()) {
//                     Some(status) => {
//                         println!("\t\tstatus from -s: {}", status);
//                         status
//                     }
//                     None => {
//                         println!(
//                             "\t\tstatus from -s was not recognized. \
//                          using {:?}",
//                             default_status.to_string()
//                         );
//                         default_status
//                     }
//                 };
//             } else {
//                 println!(
//                     "\t\tstatus (-s) was not provided. Using {:?}",
//                     default_status
//                 );
//             }
//
//             let mut detail_s: &String = &String::default();
//             // let mut detail_s: &String = (&Default::default)();
//             if let Some(detail) = sub_matches.get_one::<String>("d") {
//                 detail_s = detail;
//                 println!("\t\tdetail from -d: {detail}");
//             } else {
//                 println!("\t\tDetail (-d) was not provided for the task")
//             }
//
//             let objective_option: Option<&String>;
//             if let Some(objective_id_temp) = sub_matches.get_one::<String>("o") {
//                 println!("\t\tlocation -o: {objective_id_temp}");
//                 objective_option = Some(objective_id_temp);
//                 // error returned later if -o oid is not found in the domain
//             } else {
//                 println!(
//                     "\t\tno objective location specified.  \
//                     A pre-specified location will be used."
//                 );
//                 objective_option = None
//             }
//             do_eg_addtask(name, task_status, detail_s, objective_option)?
//         }
//
//         Some(("delete", sub_matches)) => {
//             let hash = sub_matches.get_one::<String>("HASH").expect("required");
//             println!("\t\t user requested delete of object with hash {hash}")
//         }
//
//         Some(("update", sub_matches)) => {
//             if let Some(name) = sub_matches.get_one::<String>("n") {
//                 println!("\t\tname to be updated from -n: {name}");
//             }
//             if let Some(detail) = sub_matches.get_one::<String>("d") {
//                 println!("\t\tdetail name from -d: {detail}");
//             }
//             if let Some(max_s) = sub_matches.get_one::<String>("m") {
//                 match u32::from_str(&*max_s) {
//                     // match the value of a Result
//                     Ok(max) => {
//                         println!("\t\tmax from -m: {max}");
//                     }
//                     Err(err) => {
//                         need_better_error_handling(format!(
//                             "error getting number from -m {max_s}: {err}"
//                         ));
//                     }
//                 }
//             }
//             let g_hash = sub_matches.get_one::<String>("g").expect("required");
//             let o_hash: &String;
//             if let Some(o_hash_tmp) = sub_matches.get_one::<String>("o") {
//                 o_hash = o_hash_tmp;
//                 println!("\ttodo: implement update for (gHash: {g_hash}, oHash: {o_hash})");
//             } else {
//                 println!("\ttodo: implement update for (gHash: {g_hash})")
//             }
//             println!("\ttodo: implement update(slotState). use hashes to update new g, o and t",);
//             println!("instead of slots.");
//         }
//
//         Some(("location", sub_matches)) => {
//             println!("sub command: location");
//             let goal_id: &String;
//             if let Some(goal_id_temp) = sub_matches.get_one::<String>("g") {
//                 println!("location -g: ");
//                 goal_id = goal_id_temp;
//                 let _ = do_eg_goal_location(goal_id)?;
//             };
//
//             let objective_id: &String;
//             if let Some(objective_id_temp) = sub_matches.get_one::<String>("o") {
//                 println!("location -o: ");
//                 objective_id = objective_id_temp;
//                 let _ = do_eg_objective_location(objective_id)?;
//             };
//         }
//
//         Some(("list", sub_matches)) => {
//             let list_type = sub_matches
//                 .get_one::<String>("type")
//                 .map(|s| s.as_str())
//                 .expect("Defaut list type is 'all'");
//             println!("list (type={list_type})");
//             let _ = do_eg_list()?;
//         }
//
//         Some((ext, sub_matches)) => {
//             // in the Command Defenition .allow_external_subcommands(true) makes this match legal.
//             let args = sub_matches
//                 .get_many::<OsString>("")
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<_>>();
//             println!("No direct support for {ext:?} with args: {args:?}");
//         }
//
//         _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
//     }
//
//     Ok(())
// }
