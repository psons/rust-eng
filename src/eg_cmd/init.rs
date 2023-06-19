use std::error::Error;

use crate::{
    eg_fs::write_to_domain_store,
    eg_shape::{self, EffortDomain},
    Runner,
};
use clap::Parser;

/// Sets up an effort domain data store for adding tasks
#[derive(Parser)]
pub struct Init {
    ///Name for this effort priority context
    domain: String,
}

impl Runner for Init {
    // replaces eg_cmd.rs do_eg_init
    fn run(self) -> Result<(), Box<dyn Error>> {
        let name = self.domain.clone();
        let ed: EffortDomain = eg_shape::init_domain_data(self.domain);
        write_to_domain_store(&ed)?;
        println!("init: Initialized data file for:'{name}' ");
        println!("domain:\n{}", ed.as_yaml());

        Ok(())
    }
}
