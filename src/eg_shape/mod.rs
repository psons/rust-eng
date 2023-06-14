pub mod got;

use std::error::Error;
use std::fmt;
use std::fmt::{Display,Formatter};
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::eg_shape::got::{Goal};
use crate::eg_shape::got::Objective;
// use serde_json::{Result};
/*
https://github.com/serde-rs/json
A string of JSON data can be parsed into a serde_json::Value by the
serde_json::from_str function. There is also from_slice for
parsing from a byte slice &[u8] and from_reader for parsing
from any io::Read like a File or a TCP stream.
 */

#[derive(Clone,Debug)]
pub struct EgShapeError {
    pub error_description : String
}
/*
Implement Display Trait for EgShapeError
*/
impl Display for EgShapeError {
    fn fmt (&self, f : &mut Formatter<'_>) -> fmt::Result {
        write! (f, "EgShapeError:{}", self.error_description)
    }
}
/*
Implement the Error Trait
*/
impl Error for EgShapeError {
    fn source (&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/*
return an EffortDomain with a caller provided name and default values
 */
pub fn init_domain_data(name: String) -> EffortDomain {
    let default_todo_max_tasks = 6;  // tasks in the sprint_todo list
    let default_max_objectives = 3; // objectives a from which Goal may contribute tasks
                                         //     to the sprint
    let default_max_tasks = 1;      // tasks an objective (aka story) may contribute
    let default_goal_name = "default goal";
    let default_objective_name = "default objective";
    let mut e_domain =
        EffortDomain::new(name, default_todo_max_tasks) ;
    let mut default_goal =
        Goal::new(default_goal_name.to_string(), default_max_objectives);
    let default_objective =
        Objective::new(default_objective_name.to_string(), default_max_tasks);
    default_goal.add_objective(default_objective);
    e_domain.add_goal(default_goal);
    e_domain
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct EffortDomain {
    pub name: String,
    pub todo_max_tasks: u32,
    pub goals: Vec<Goal>,
}



impl EffortDomain {
    pub fn new(name: String, todo_max_tasks: u32) -> EffortDomain {
        EffortDomain { name, todo_max_tasks, goals: vec![] }
    }
    /*
    Appends the goal to the goals list and returns a reference to the goal
    which is now owned by the Vector.
     */
    pub fn add_goal(&mut self, goal: Goal) -> &Goal {
        self.goals.push(goal);
        &self.goals.last().unwrap()
    }

    /// The find_ family of methods in EffortDomain return a Result.
    /// Failure of a find_method in the Effort Domain is an error.
    /// Some find methods use the get_family of methods, which return Option
    /// Failure in the get_ methods is not considered an error.
    pub fn find_goal(&mut self, gid: &str) -> Result<&mut Goal, EgShapeError>  {
        for goal_ref in &mut self.goals {
            if goal_ref.gid == gid {
                return Ok(goal_ref)
            }
        }
        let err_message: String = format!("Goal gid {gid} not found in the domain");
        let not_found_err: Result<&mut Goal,EgShapeError> = Err(EgShapeError {
            error_description : err_message });
        not_found_err
    }

    /*
    // example data has a oid:
    */
    pub fn find_objective(&mut self, oid: &str) -> Result<&mut Objective, EgShapeError>  {
        for goal_ref in &mut self.goals {
            if let Some(objective_mref) =  goal_ref.get_objective(oid) {
                return Ok(objective_mref)
            }
        }
        let err_message: String = format!("Objective oid {oid} not found in the domain");
        let not_found_err: Result<&mut Objective,EgShapeError> = Err(EgShapeError {
            error_description : err_message });
        not_found_err
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn as_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }


}

pub fn get_eg_id() -> String {
    return ObjectId::new().unwrap().to_hex();
}
