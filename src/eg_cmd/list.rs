use clap::Parser;

use crate::{eg_fs::read_from_domain_store, eg_shape::EffortDomain, Runner};

/// List goals, objectives, and tasks
#[derive(Parser)]
pub struct List {
    /// Type to filter list
    #[clap(short, long = "type")]
    #[arg(value_enum)]
    ty: Option<Type>,
}

impl Runner for List {
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let ed: EffortDomain = read_from_domain_store()?;
        println!("EffortDomain: {}", ed.as_yaml());
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    All,
    Sprint,
    Location,
}

impl clap::ValueEnum for Type {
    fn value_variants<'a>() -> &'a [Self] {
        &[Type::All, Type::Sprint, Type::Location]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new("all"))
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::All => write!(f, "all"),
            Type::Sprint => write!(f, "sprint"),
            Type::Location => write!(f, "location"),
        }
    }
}
