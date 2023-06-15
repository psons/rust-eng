use clap::{Parser, Subcommand};

use crate::{
    eg_fs::{read_from_domain_store, write_objective_store, write_to_domain_store},
    eg_shape::{got, EffortDomain},
    Runner,
};

use super::{context_goal_for_objectives, context_objective_for_tasks};

/// add an entity to the system
#[derive(Subcommand)]
pub enum Add {
    Goal(Goal),
    Objective(Objective),
    Task(Task),
}

impl Runner for Add {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Add::Goal(inner) => inner.run(),
            Add::Objective(inner) => inner.run(),
            Add::Task(inner) => inner.run(),
        }
    }
}

/// add a goal to the domain
#[derive(Parser)]
pub struct Goal {
    /// name of goal
    #[clap(short, long)]
    name: String,

    /// max objectives
    #[clap(short, long)]
    max: u32,
}

impl Runner for Goal {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ed: EffortDomain = read_from_domain_store()?;
        let new_goal = got::Goal::new(self.name, self.max);
        ed.add_goal(new_goal);
        write_to_domain_store(&ed)?;
        // todo: report the objective name too, but at present this fn doesn't have it
        // println!("Domain store updated with new goal: {:?}", new_goal); // todo make Goal Clone
        Ok(())
    }
}

/// add an objective to the current goal or as specified with --goal
#[derive(Parser)]
pub struct Objective {
    /// name of the objective
    #[clap(short, long)]
    name: String,

    /// max number of tasks
    #[clap(short, long)]
    max: u32,

    /// goal to associate with
    #[clap(short, long)]
    goal: Option<String>,
}

impl Runner for Objective {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ed: EffortDomain = read_from_domain_store()?;
        let context_g_for_o = context_goal_for_objectives(&mut ed, self.goal.as_ref())?; // goal_option: Option<&String>)
        let new_objective = got::Objective::new(self.name, self.max);
        let new_oid = String::from(new_objective.oid.as_str());
        context_g_for_o.add_objective(new_objective);
        write_to_domain_store(&ed)?;
        write_objective_store(new_oid.as_str())?;
        Ok(())
    }
}

/// add a goal to the domain
#[derive(Parser)]
pub struct Task {
    /// name of the task
    #[clap(short, long)]
    name: String,

    /// current status of the task
    #[clap(short, long)]
    #[arg(value_enum)]
    status: got::Status,

    /// detail of the task
    #[clap(short, long)]
    detail: String,

    /// objective to associate with
    #[clap(short, long)]
    objective: Option<String>,
}

impl Runner for Task {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ed: EffortDomain = read_from_domain_store()?;
        let context_o_for_t = context_objective_for_tasks(&mut ed, self.objective.as_ref())?; // objective_option: Option<&String>)
        let new_task = got::Task::new(self.status, self.name, self.detail);
        context_o_for_t.add_task(new_task);
        write_to_domain_store(&ed)?;
        Ok(())
    }
}
