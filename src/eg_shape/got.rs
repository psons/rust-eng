/*
Module for the Goal, Objective, and Task structs in the
End Goal app's EffortDomain data hierarchy.
 */

// static let defaultOslot = 0
// static let invalidOslot = -1

use std::fmt::{Display, Formatter};
// use std::error::Error;
use serde::{Serialize, Deserialize};

use crate::eg_shape::{get_eg_id};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Goal {
    id: String,
    pub name: String,
    pub max_objectives: u32,
    pub gid: String,
    pub objectives: Vec<Objective>,
}

impl Goal {
    // new method provided to compute gid from name.
    pub fn new(name: String, max_objectives: u32) -> Goal {
        let gid = get_eg_id(); // digest_string_short(&name);
        let id = gid.clone();
        Goal {id, name, max_objectives, gid, objectives: vec![] }
    }

    /// Appends the objective to the objectives list and returns a reference to the objective
    /// which is now owned by the Vector.
    pub fn add_objective(&mut self, objective: Objective) -> &Objective {
        self.objectives.push(objective);
        &self.objectives.last().unwrap()
    }

    /*
    // example data has a oid:
    */
    /// Return mutable ref to a matching objective if found.
    /// Failure to fine the objective is not necessarily an error because it may
    /// be found in a different goal
    pub fn get_objective(&mut self, oid: &str) -> Option<&mut Objective>  {
        for objective_mref in &mut self.objectives {
            if objective_mref.oid == oid {
                return Option::Some(objective_mref)
            }
        }
        None
    }

}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Objective {
    pub name: String,
    pub max_tasks: u32,
    pub oid: String,
    pub tasks: Vec<Task>,
}

impl Objective {
    // new method provided to compute oid from name.
    pub fn new(name: String, max_tasks: u32) -> Objective {
        let oid = get_eg_id(); // digest_string_short(&name);
        Objective {name, max_tasks, oid, tasks: vec![] }
    }

    /*
    Appends the task to the tasks list and returns a reference to the task
    which is now owned by the Vector.
     */
    pub fn add_task(&mut self, task: Task) -> &Task {
        self.tasks.push(task);
        &self.tasks.last().unwrap()
    }

}

#[derive(Debug, Copy)]
#[derive(Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
#[derive(Default)]
pub enum Status {
    abandoned,
    completed,
    scheduled,
    in_progress,
    unfinished,
    #[default]          // default: https://doc.rust-lang.org/std/default/trait.Default.html#enums
    todo,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Status::abandoned => {
                write!(f, "abandoned")
            }
            Status::completed => {
                write!(f, "completed")
            }
            Status::scheduled => {
                write!(f, "scheduled")
            }
            Status::in_progress => {
                write!(f, "in_progress")
            }
            Status::unfinished => {
                write!(f, "unfinished")
            }
            Status::todo => {
                write!(f, "todo")
            }
        }
    }
}



pub fn status_opt_from_str(s: &str) -> Option<Status> {
    match s {
        "abandoned" => Some(Status::abandoned),
        "completed" => Some(Status::completed),
        "scheduled" => Some(Status::scheduled),
        "in_progress" => Some(Status::in_progress),
        "unfinished" => Some(Status::unfinished),
        "todo" => Some(Status::todo),
        _ => None,
    }
}


#[derive(Debug)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub status: Status,
    pub name: String,
    pub detail: String, // future feature: allow detail to be option so that other platforms can omit the attribute.
                        // String has a defaul;t of empty string
                        // https://doc.rust-lang.org/std/string/struct.String.html#impl-Default-for-String
    pub tid: String,
}


impl Task {
    // new method provided to compute oid from name.
    pub fn new(status: Status, name: String, detail: String) -> Task {
        let tid = get_eg_id(); // digest_string_short(&name);
        Task {status, name, detail, tid }
    }
}


