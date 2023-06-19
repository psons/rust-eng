use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;

use crate::{
    eg_fs::{read_from_domain_store, write_objective_store, write_to_domain_store},
    eg_shape::{got, EffortDomain},
    Runner,
};
// use crate::{
//     eg_fs::write_to_domain_store,
//     eg_shape::{self, EffortDomain},
//     Runner,
// };

// todo - validate the PATH arg with pathBuf
// https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
// todo this should be a file option on the init command because it is destructive.
#[derive(Parser)]
pub struct Load {
    /// File path with json data to re-initialize the domain.
    path: String,
}

/// loads the user provided file path and deserializes it to make sure its Ok(),
/// assuming it deserializes without an error.
/// then re-serializes it and writes it to the domain store
impl Runner for Load {
    // replaces eg_cmd.rs do_eg_load
    fn run(self) -> Result<(), Box<dyn Error>> {
        println!(
            "load: will destroy existing data in the domain store and load from {:?} ",
            self.path
        );
        let file_content_string = fs::read_to_string(self.path)?;
        let ed: EffortDomain = serde_json::from_str(&file_content_string).unwrap();
        write_to_domain_store(&ed)?;

        Ok(())
    }
}
//         Some(("load", sub_matches)) => {
//             let path: &OsStr = sub_matches
//                 .get_one::<PathBuf>("PATH")
//                 .map(|s| s.as_os_str())
//                 // no method named `as_str` ... method with a similar name .map(|s| s.as_os_str())
//                 .expect("Default list type is 'all'");
//             // println!("load: will destroy existing data in a file place known in eg_fs.rs and load from {path:?} ");
//             do_eg_load(path)?;
//         }
