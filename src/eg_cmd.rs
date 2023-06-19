use crate::eg_fs::{
    goal_dir_file, objective_dir_file, read_dir_file_string, read_from_domain_store,
    write_goal_store, write_objective_store, write_to_domain_store,
};
use crate::eg_shape;
use crate::eg_shape::EffortDomain;
use std::error::Error;

use crate::eg_shape::got::{Goal, Objective, Status, Task};
use std::ffi::OsStr;
use std::fs;

mod add;
mod init;
mod list;
mod load;
mod location;

pub use add::Add;
pub use init::Init;
pub use list::List;
pub use load::Load;
pub use location::Location;

/// for location -g: validate the goal is found
///  - get_goal(gid: )
///  - if no error, persist the gid
///  - error can result from
///     - location not found
///     - file and directory problems persisting
pub fn do_eg_goal_location(gid: &str) -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let goal_mref = ed.find_goal(gid)?; // other usages of get_goal need mutable, but not here
    write_goal_store(goal_mref.gid.as_str())?;
    println!("\tGoal: {}", goal_mref.name);
    println!(
        "\twith ID: {} saved as current goal context for adding Objectives",
        goal_mref.gid
    );
    Ok(())
}

/// for location -o: validate the objective is found
///  - get_objective(oid: )
///  - if no error, persist the oid
///  - error can result from
///     - location not found
///     - file and directory problems persisting
pub fn do_eg_objective_location(oid: &str) -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let objective_mref = ed.find_objective(oid)?; // other usages of get_... need mutable. not here
    write_objective_store(objective_mref.oid.as_str())?;
    // todo: report the objective name too, but at present this fn doesn't have it
    println!("\tObjective: {}", objective_mref.name);
    println!(
        "\twith ID: {} saved as current objective context for adding Tasks",
        objective_mref.oid
    );
    Ok(())
}

/// Tries 3 possible ways to get a Goal to add Objectives to
///  1 - gid_option might have a valid goal ID that is in the ed.
///  2 - there might be a goal ID saved in the goal store that is valid in the ed.
///  3 - if the ed is correctly initialized, there is a default goal.
pub fn context_goal_for_objectives<'a>(
    ed: &'a mut EffortDomain,
    gid_option: Option<&'a String>,
) -> Result<&'a mut Goal, Box<dyn Error>> {
    let goal_mref: &mut Goal;
    if let Some(gid) = gid_option {
        // a gid was specified, so find it or Result is Error
        let goal_mref = ed.find_goal(gid)?; // <-- might error if the user gave a bad gid
        return Ok(goal_mref);
    } else {
        // there was no gid specified so we must find a goal to put objectives under
        let ns_str = "nothing_stored";
        let ns = String::from(ns_str);
        let stored_gid = read_dir_file_string(goal_dir_file()).unwrap_or(ns); // prevents panic if the read is an Err
        if stored_gid.as_str() != ns_str {
            let goal_mref = ed.find_goal(stored_gid.as_str())?;
            // got a valid stored_gid and have a goal_mref to return
            return Ok(goal_mref);
        }
        // if we get here, there was no valid gid available
        goal_mref = &mut ed.goals[0]; // todo - not sure this won't just panic if there is no goals[0]
                                      //  will the ed deserialize if there are no goals?  That should be an error.
        return Ok(goal_mref);
    }
    // Err(EgShapeError("The Domain is not correctly initialized to create Objectives\n \
    // try starting with eng init 'some domain name'"))  // todo make a different error type.
}

// do_eg_addtask(name, status, detail, objective_option)
pub fn do_eg_addtask(
    name: &str,
    status: Status,
    detail: &str,
    objective_option: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let context_o_for_t = context_objective_for_tasks(&mut ed, objective_option)?; // objective_option: Option<&String>)
    let new_task = Task::new(status, String::from(name), String::from(detail));
    context_o_for_t.add_task(new_task);
    write_to_domain_store(&ed)?;
    Ok(())
}

// context_objective_for_tasks(&mut ed, objective_option)?;
/// Tries 3 possible ways to get an Objective to add Tasks to
///  1 - oid_option might have a valid objective ID that is in the ed.
///  2 - there might be an objective ID saved in the objective store that is valid in the ed.
///  3 - if the ed is correctly initialized, there is a default objective.
pub fn context_objective_for_tasks<'a>(
    ed: &'a mut EffortDomain,
    oid_option: Option<&'a String>,
) -> Result<&'a mut Objective, Box<dyn Error>> {
    let objective_mref: &mut Objective;
    if let Some(oid) = oid_option {
        // an oid was specified, so find it or Result is Error
        let objective_mref = ed.find_objective(oid)?; // <-- might error if the user gave a bad oid
        return Ok(objective_mref);
    } else {
        // there was no oid specified so we must find an objective to put tasks under
        let ns_str = "nothing_stored";
        let ns = String::from(ns_str);
        let stored_oid = read_dir_file_string(objective_dir_file()).unwrap_or(ns); // prevents panic if the read is an Err
        if stored_oid.as_str() != ns_str {
            let objective_mref = ed.find_objective(stored_oid.as_str())?;
            // got a valid stored_gid and have a goal_mref to return
            return Ok(objective_mref);
        }
        // if we get here, there was no valid gid available
        objective_mref = &mut ed.goals[0].objectives[0];
        // todo - not sure this won't just panic if there is no goals[0]
        //  will the ed deserialize if there are no goals or goal[0] has no objectives?
        //  That should be an error.
        return Ok(objective_mref);
    }
}
