fn truth_base(project: &str) -> bool {
    let is_true = if project == "true" { true } else { false };

    is_true
}

pub mod file_reader {
    use std::{
        fs::{self, File},
        io::stdin,
        process::{Command, Stdio},
    };

    use crate::{
        file_parsin::{CreatedByArgs, FileCreated},
        golang::FinalProjectBase,
        opener::truth_base,
    };

    pub fn read_input(str_message: &str) -> String {
        format!("{}", str_message)
    }

    pub fn opener() -> Result<(), Box<dyn std::error::Error>> {
        use serde_json::Value;
        use std::path::Path;

        let f = Path::new("Args.json");

        let data_file = fs::read_to_string(f)?;
        let data: Value = serde_json::from_str(&data_file)?;
        let d = data["FileName"].as_str().unwrap();

        let project_custom = data["Custom"].as_str().unwrap();

        if project_custom.contains(&truth_base("true").to_string()) {
            let project_name = data["NameProject"].as_str().unwrap();

            Command::new(format!("{}{}", "cargo new", project_name.to_lowercase()))
                .spawn()
                .expect("No File or Go Project Created");
        }

        let project = data["Project"].as_str().unwrap();
        if project == "GoLang" {
            let mut file_go = FileCreated { file: "main.go" };
            File::create(file_go.check().file)?;

            let project_name = data["NameProject"].as_str().unwrap();

            let project_info: FinalProjectBase<String> = FinalProjectBase {
                name: project_name.to_string(),
            };

            Command::new("cmd")
                .args(["/C", project_info.name.as_str()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .expect("Cannot run Go Project");
        }

        if project == "RustProject" {
            let project_name = data["NameProject"].as_str().unwrap();

            Command::new(format!("{}{}", "cargo new", project_name.to_lowercase()))
                .spawn()
                .expect("No File or Go Project Created");
        }

        let project_read_json = data["ReadJsonFile"].as_str().unwrap();

        let mod_file = data["ReadJsonFile"].as_str().unwrap();

        if mod_file == &truth_base("true").to_string() {
            let name_prod = data["NameProject"].as_str().unwrap();

            let project_inner_name: FinalProjectBase<String> = FinalProjectBase {
                name: name_prod.to_string(),
            };

            Command::new("cmd")
                .arg("/C")
                .arg("go mod init")
                .arg(project_inner_name.name)
                .spawn()
                .expect("No File or Go Project Created");
        }

        if project_read_json == "true" {
            let project_name = data["NameProject"].as_str().unwrap();

            Command::new(format!("{}{}", "cargo new", project_name.to_lowercase()))
                .spawn()
                .expect("No File or Go Project Created");
        } else {
            let mut read_json_parser = String::new();
            println!(
                "{}",
                read_input("Enter Option for Project Creation").to_uppercase()
            );
            stdin().read_line(&mut read_json_parser).unwrap();

            if read_json_parser == "GoLang" {
                let mut file_go = FileCreated { file: "main.go" };
                File::create(file_go.check().file)?;

                let project_name = data["NameProject"].as_str().unwrap();

                let project_info: FinalProjectBase<String> = FinalProjectBase {
                    name: project_name.to_string(),
                };

                Command::new("cmd")
                    .args(["/C", project_info.name.as_str()])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .expect("Cannot run Go Project");
            } else {
                if read_json_parser == "Cargo" || read_json_parser == "rust" {
                    let project_name = data["NameProject"].as_str().unwrap();

                    let project_info: FinalProjectBase<String> = FinalProjectBase {
                        name: project_name.to_string(),
                    };

                    Command::new("cmd")
                        .args(["/C", project_info.name.as_str()])
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect(&"Cannot run or Create Rust Project...".to_uppercase());
                }
            }
        }

        if d.len() > 1 {
            let mut file_s = FileCreated { file: d };
            File::create(file_s.check().file)?;
            println!("File Name: {}", d);
        }

        Ok(())
    }
}
