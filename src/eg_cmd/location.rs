use clap::Parser;

use crate::eg_cmd::{do_eg_goal_location, do_eg_objective_location};
use crate::Runner;

/// Sets the goal context for adding objective or the objective context for adding tasks.
#[derive(Parser)]
pub struct Location {
    /// Goal to use for objective additions
    #[clap(short, long)]
    goal: Option<String>,

    /// Objective to use for task additions
    #[clap(short, long)]
    objective: Option<String>,
}

impl Runner for Location {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("sub command: location");
        match self.goal {
            None => println!("No --goal specified"),
            Some(a_gid) => {
                println!("location --goal: ");
                let _ = do_eg_goal_location(a_gid.as_str())?;
            }
        };
        match self.objective {
            None => println!("No --objective specified"),
            Some(an_oid) => {
                println!("location --objective: ");
                let _ = do_eg_objective_location(an_oid.as_str())?;
            }
        };
        Ok(())
    }
}
