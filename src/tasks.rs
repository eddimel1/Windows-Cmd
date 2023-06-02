pub mod Tasks {
    use std::process::{Command, Output};

    use eframe::{
        egui::{self, Ui},
        emath::Align,
    };
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    use crate::button_height;

    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum TaskOptions {
        #[default]
        TaskList,
        TaskKill,
    }
    #[derive(Debug, Default)]
    pub struct Tasks {
        pid: String,
    }

    impl Tasks {
        pub fn new() -> Self {
            Tasks { pid: String::new() }
        }
        pub fn validate_output(output: Output) -> Option<String> {
            if output.status.success() {
                let utf8_string = String::from_utf8_lossy(&output.stdout).to_string();
                Some(utf8_string)
            } else {
                // Print the error message if the command failed
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Command failed with error: {}", error_message);
                None
            }
        }
        pub fn cmd_tasks(options: TaskOptions, pid: Option<String>) -> Option<String> {
            let output: Output;
            if options == TaskOptions::TaskList && pid.is_none() {
                output = Command::new("tasklist")
                    .output()
                    .expect("Failed to execute command");
            } else {
                output = Command::new("taskkill")
                    .args(["/F", "/PID", &pid.unwrap()])
                    .output()
                    .expect("Failed to execute command");
            }
            Self::validate_output(output)
        }

        pub fn get_task_options(&mut self, command_output: &mut String, ui: &mut Ui) {
            
            for op in TaskOptions::iter() {
                ui.vertical(|ui| {
                     if op == TaskOptions::TaskList {
                        
                        let option_str = format!("{:?}", op);
                        let res = ui.add_sized([60., button_height], egui::Button::new(option_str));

                        if res.clicked() {
                            *command_output = Self::cmd_tasks(op, None).unwrap();
                        }
                    }

                    else if op == TaskOptions::TaskKill {
                        ui.horizontal(|ui|{
                           
                            let option_str = format!("{:?}", op);
                            let res = ui.add_sized([50., button_height], egui::Button::new(option_str));
    
                            if res.clicked() {
                                *command_output = Self::cmd_tasks(op, Some(self.pid.clone()))
                                    .unwrap_or("Process was not found".to_owned());
                            }
                            ui.add(egui::TextEdit::singleline(&mut self.pid));
                        });
                       
                    }
                });
            }
        }
    }

    impl std::fmt::Display for TaskOptions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let variant_str = match self {
                TaskOptions::TaskList => "TaskList",
                TaskOptions::TaskKill => "TaskKill",
            };

            write!(f, "{}", variant_str)
        }
    }
}
