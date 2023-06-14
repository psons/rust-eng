use std::error::Error;
use crate::eg_fs::{goal_dir_file, objective_dir_file, read_dir_file_string, read_from_domain_store, write_goal_store, write_objective_store, write_to_domain_store};
use crate::eg_shape;
use crate::eg_shape::EffortDomain;

use std::ffi::{OsStr};
use std::fs;
use crate::eg_shape::got::{Goal, Objective, Status, Task};

pub fn do_eg_init(name: &str) -> Result<(), Box<dyn Error>> {
    let ed: EffortDomain =  eg_shape::init_domain_data(name.to_string());
    // let e_domain_json:String =
    //     self::eg_shape::domain_as_json(
    //         &self::eg_shape::init_domain_data(name.to_string())
    //     );
    // let ed: EffortDomain = serde_json::from_str(&e_domain_json).unwrap();
    write_to_domain_store(&ed)?;
    println!("init: Initialized data file for:'{name}' ");

    // let e_domain_yaml:String =
    //     self::eg_shape::domain_as_yaml(
    //     );
    println!("domain:\n{}", ed.as_yaml());


    Ok(())
}

/*
loads the user provided file path and deserializes it to make sure its Ok(),
assuming it deserializes without an error.
then re-serializes it to the domain store
 */
pub fn do_eg_load(path: &OsStr) -> Result<(), Box<dyn Error>> {
    let file_content_string = fs::read_to_string(path)?;
    let ed: EffortDomain = serde_json::from_str(&file_content_string).unwrap();
    write_to_domain_store(&ed)?;
    Ok(())
}


pub fn do_eg_list() -> Result<(), Box<dyn Error>> {
    let ed: EffortDomain = read_from_domain_store()?;
    println!("EffortDomain: {}", ed.as_yaml());
    Ok(())
}

/// for location -g: validate the goal is found
///  - get_goal(gid: )
///  - if no error, persist the gid
///  - error can result from
///     - location not found
///     - file and directory problems persisting
pub fn do_eg_goal_location(gid: &str) -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let goal_mref =  ed.find_goal(gid)?; // other usages of get_goal need mutable, but not here
    write_goal_store(goal_mref.gid.as_str())?;
    println!("Goal: {}", goal_mref.name);
    println!("with ID: {} saved as current goal context for adding Objectives", goal_mref.gid);
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
    let objective_mref =  ed.find_objective(oid)?; // other usages of get_... need mutable. not here
    write_objective_store(objective_mref.oid.as_str())?;
    // todo: report the objective name too, but at present this fn doesn't have it
    println!("Objective: {}", objective_mref.name);
    println!("with ID: {} saved as current objective context for adding Tasks", objective_mref.oid);
    Ok(())
}

pub fn do_eg_addgoal(name: &str,  max_objectives: u32) -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let new_goal = Goal::new(String::from(name), max_objectives);
    ed.add_goal(new_goal);
    write_to_domain_store(&ed)?;
    // todo: report the objective name too, but at present this fn doesn't have it
    // println!("Domain store updated with new goal: {:?}", new_goal); // todo make Goal Clone
    Ok(())
}

pub fn do_eg_addobjective(name: &str, max_tasks: u32, goal_option: Option<&String>)
                          -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let context_g_for_o =
        context_goal_for_objectives(&mut ed, goal_option)?; // goal_option: Option<&String>)
    let new_objective = Objective::new(String::from(name), max_tasks);
    let new_oid = String::from(new_objective.oid.as_str());
    context_g_for_o.add_objective(new_objective);
    write_to_domain_store(&ed)?;
    write_objective_store(new_oid.as_str())?;
    Ok(())
}

/// Tries 3 possible ways to get a Goal to add Objectives to
///  1 - gid_option might have a valid goal ID that is in the ed.
///  2 - there might be a goal ID saved in the goal store that is valid in the ed.
///  3 - if the ed is correctly initialized, there is a default goal.
pub fn context_goal_for_objectives<'a>(ed: &'a mut EffortDomain, gid_option: Option<&'a String>)  -> Result<&'a mut Goal, Box<dyn Error>> {
    let goal_mref: &mut Goal;
    if let Some(gid) = gid_option {
        // a gid was specified, so find it or Result is Error
        let goal_mref =  ed.find_goal(gid)?;  // <-- might error if the user gave a bad gid
        return Ok(goal_mref);
    } else {
        // there was no gid specified so we must find a goal to put objectives under
        let ns_str = "nothing_stored";
        let ns = String::from(ns_str);
        let stored_gid = read_dir_file_string(goal_dir_file()).unwrap_or( ns); // prevents panic if the read is an Err
        if stored_gid.as_str() != ns_str {
            let goal_mref = ed.find_goal(stored_gid.as_str())?;
            // got a valid stored_gid and have a goal_mref to return
            return Ok(goal_mref)
        }
        // if we get here, there was no valid gid available
        goal_mref = &mut ed.goals[0];  // todo - not sure this won't just panic if there is no goals[0]
        //  will the ed deserialize if there are no goals?  That should be an error.
        return Ok(goal_mref)
    }
    // Err(EgShapeError("The Domain is not correctly initialized to create Objectives\n \
    // try starting with eng init 'some domain name'"))  // todo make a different error type.
}

// do_eg_addtask(name, status, detail, objective_option)
pub fn do_eg_addtask(name: &str, status: Status, detail: &str, objective_option: Option<&String>)
                     -> Result<(), Box<dyn Error>> {
    let mut ed: EffortDomain = read_from_domain_store()?;
    let context_o_for_t =
        context_objective_for_tasks(&mut ed, objective_option)?; // objective_option: Option<&String>)
    let new_task = Task::new(status,String::from(name), String::from(detail));
    context_o_for_t.add_task(new_task);
    write_to_domain_store(&ed)?;
    Ok(())
}

// context_objective_for_tasks(&mut ed, objective_option)?;
/// Tries 3 possible ways to get an Objective to add Tasks to
///  1 - oid_option might have a valid objective ID that is in the ed.
///  2 - there might be an objective ID saved in the objective store that is valid in the ed.
///  3 - if the ed is correctly initialized, there is a default objective.
pub fn context_objective_for_tasks<'a>(ed: &'a mut EffortDomain, oid_option: Option<&'a String>)
                                       -> Result<&'a mut Objective, Box<dyn Error>> {
    let objective_mref: &mut Objective;
    if let Some(oid) = oid_option {
        // an oid was specified, so find it or Result is Error
        let objective_mref =
            ed.find_objective(oid)?;  // <-- might error if the user gave a bad oid
        return Ok(objective_mref);
    } else {
        // there was no oid specified so we must find an objective to put tasks under
        let ns_str = "nothing_stored";
        let ns = String::from(ns_str);
        let stored_oid = read_dir_file_string(objective_dir_file()).unwrap_or( ns); // prevents panic if the read is an Err
        if stored_oid.as_str() != ns_str {
            let objective_mref = ed.find_objective(stored_oid.as_str())?;
            // got a valid stored_gid and have a goal_mref to return
            return Ok(objective_mref)
        }
        // if we get here, there was no valid gid available
        objective_mref = &mut ed.goals[0].objectives[0];
        // todo - not sure this won't just panic if there is no goals[0]
        //  will the ed deserialize if there are no goals or goal[0] has no objectives?
        //  That should be an error.
        return Ok(objective_mref)
    }
}

