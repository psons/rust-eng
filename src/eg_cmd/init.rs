use std::error::Error;

use crate::{
    eg_fs::write_to_domain_store,
    eg_shape::{self, EffortDomain},
    Runner,
};
use clap::Parser;

/// Sets up a data store  with an effort domain for adding tasks
#[derive(Parser)]
pub struct Init {
    ///Name for this effort priority context
    domain: String,
}

impl Runner for Init {
    fn run(self) -> Result<(), Box<dyn Error>> {
        let name = self.domain.clone();
        let ed: EffortDomain = eg_shape::init_domain_data(self.domain);
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
}
