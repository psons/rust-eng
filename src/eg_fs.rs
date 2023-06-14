/*
File system related thing for eng aka end goal cmd line.
 */
use std::error::Error;
use std::fs;
use crate::eg_shape::EffortDomain;

pub struct DirFile {
    dir: String,
    file: String,
}

/// Returns String form of the file path represented by the dir and file.
impl DirFile {
    pub fn as_path(&self) -> String {
        format!("{}/{}", self.dir, self.file )
    }
}

/// Uses eng_dir() to build a domain filepath
pub fn domain_dir_file() -> DirFile {
    let eng_dir_s: String = eng_dir();
    DirFile {dir: eng_dir_s, file: String::from("/domain.json")}
}

/// Uses eng_dir() to build a current goal filepath
pub fn goal_dir_file() -> DirFile {
    let eng_dir_s: String = eng_dir();
    DirFile {dir: eng_dir_s, file: String::from("/current_goal.txt")}
}

/// Uses eng_dir() to build a current objective filepath
pub fn objective_dir_file() -> DirFile {
    DirFile {dir: eng_dir(), file: String::from("/current_objective.txt")}
}


/// builds conventional domain dir string from the users HOME dir
fn eng_dir() -> String {
    let home = std::env::var("HOME").unwrap();
    let eng_dir = format!("{home}/.eng/");
    eng_dir
}

/// Codes the the domain file location into this module
pub fn write_domain_store(string_data: &str) -> Result<(), Box<dyn Error>> {
    let df = domain_dir_file();
    write_dir_file_string(df.dir.as_str(), df.file.as_str(), string_data)?;
    Ok(())
}

/// Codes the the goal file location into this module
pub fn write_goal_store(string_data: &str) -> Result<(), Box<dyn Error>> {
    let df =  DirFile {dir: eng_dir(), file: String::from("/current_goal.txt")};
    write_dir_file_string(df.dir.as_str(), df.file.as_str(), string_data)?;
    Ok(())
}


/// Codes the the objective file location into this module
pub fn write_objective_store(string_data: &str) -> Result<(), Box<dyn Error>> {
    // let df =  DirFile {dir: eng_dir(), file: String::from("/current_objective.txt")};
    let df =  objective_dir_file();
    write_dir_file_string(df.dir.as_str(), df.file.as_str(), string_data)?;
    Ok(())
}

/// uses DirFile dir_file to make a path to read a String result
pub fn read_dir_file_string(dir_file: DirFile) -> Result<String, Box<dyn Error>> {
    let file_path: String =  dir_file.as_path();
    let file_content_string = fs::read_to_string(file_path)?;
    Ok(file_content_string)
}

/*
generic make the directory and write the file.
 */
fn write_dir_file_string(dir: &str, file: &str, data: &str) -> Result<(), Box<dyn Error>> {
    let dir_file_path = format!("{dir}/{file}");
    std::fs::create_dir_all(dir)?;
    fs::write(dir_file_path, data)?;
    Ok(())
}

// ###### ported from eng lib
/*
Writes the EffortDomain type as serialized to the domain store.
 */
pub fn write_to_domain_store(e_domain: &EffortDomain) ->  Result<(), Box<dyn Error>> {
    // let e_domain_json: String =
    //     crate::eg_shape::domain_as_json(e_domain);
    let e_domain_json: String = e_domain.as_json();
    write_domain_store(e_domain_json.as_str())?;
    Ok(())
}

/*
read the domain store and return EffortDomain type
 */
pub fn read_from_domain_store() -> Result<EffortDomain, Box<dyn Error>>  {
    let data =  read_dir_file_string(domain_dir_file())?; // read_domain().unwrap();
    let deserialized_ed: EffortDomain = serde_json::from_str(&data).unwrap();
    Ok(deserialized_ed)
}


