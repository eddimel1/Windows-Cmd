pub mod Utility {
    use eframe::egui::{self, Ui};
    use std::{
        io::{self, BufRead},
        process::{Command, Stdio},
    };
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter;

    use crate::button_height;
    //wmic path Win32_VideoController
    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum Utility {
        #[default]
        CheckDisk,
        FormatDisk,
    }

    impl std::fmt::Display for Utility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let variant_str = match self {
                Utility::CheckDisk => "Check Disk",
                Utility::FormatDisk => "Format Disk",
            };

            write!(f, "{}", variant_str)
        }
    }
    #[derive(Debug, Default)]
    pub struct UtilityScripts {}
    impl Utility {
        fn get_administrator_name() -> Option<String> {
            let output = Command::new("whoami")
                .args(&["/groups"])
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.split('\n').collect();

                // Find the line containing "S-1-5-32-544" (Administrator group)
                for line in lines {
                    if line.contains("S-1-5-32-544") {
                        let fields: Vec<&str> = line.split(' ').collect();
                        if let Some(name) = fields.get(1) {
                            return Some(name.trim().to_string());
                        }
                    }
                }
            }

            None
        }
        pub fn cmd_tasks(options: Utility) -> String {
            let mut output: String = String::new();
            let admin_name = format!("/user:{:?}", Self::get_administrator_name());

            let mut cmd = Command::new("runas")
                .args(&["/noprofile", &admin_name.to_owned(), "chkdsk"])
                .args(&["/f", "/r", "/x"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped()) // Redirect stderr to separate pipe
                .spawn()
                .expect("Failed to execute command");

            let stdout = cmd.stdout.take().unwrap();
            let stdout_reader = io::BufReader::new(stdout);
            for line in stdout_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }

            let stderr = cmd.stderr.take().unwrap();
            let stderr_reader = io::BufReader::new(stderr);
            for line in stderr_reader.lines() {
                if let Ok(line) = line {
                    println!("{}", line);
                    output.push_str(&line);
                    output.push('\n');
                }
            }

            let status = cmd.wait().expect("Failed to wait for command");
            let exit_code = status.code();
            if let Some(code) = exit_code {
                if code != 0 {
                    output.push_str(&format!("Command failed with exit code: {:?}", code));
                }
            } else {
                output.push_str("Command failed to execute");
            }

            println!("{}", &output);
            output
        }

        pub fn get_utility_options(command_output: &mut String, ui: &mut Ui) {
            for op in Utility::iter() {
                let option_str = format!("{:?}", op);
                let res = ui.add_sized([50., button_height], egui::Button::new(option_str));

                if res.clicked() {
                    *command_output = Self::cmd_tasks(op)
                }
            }
        }
    }
}
