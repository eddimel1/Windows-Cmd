pub mod ShutDown {
    use eframe::egui::{self, Ui};
    use std::process::{Command, Output};
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter;

    use crate::button_height;
    //wmic path Win32_VideoController
    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum ShutDown {
        #[default]
        Shutdown,
        Restart,
        Logout,
        Sleep,
        RestartAndBios,
        Abort,
    }

    impl std::fmt::Display for ShutDown {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let variant_str = match self {
                ShutDown::Shutdown => "Shutdown",
                ShutDown::Restart => "Restart",
                ShutDown::Logout => "Logout",
                ShutDown::Sleep => "Sleep",
                ShutDown::RestartAndBios => "RestartAndBios",
                ShutDown::Abort => "Abort",
            };

            write!(f, "{}", variant_str)
        }
    }
    #[derive(Debug, Default)]
    pub struct ShutDownScripts {
        timeout:String
    }
    impl ShutDownScripts {

        pub fn new() -> Self {
            ShutDownScripts { timeout: String::new() }
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
        pub fn cmd_tasks(options: ShutDown , timeout:&String) -> Option<String> {
            let args_to_pass = match options {
                ShutDown::Shutdown => "/s",
                ShutDown::Restart => "/r",
                ShutDown::Logout => "/l",
                ShutDown::Sleep => "/h",
                ShutDown::RestartAndBios => "/o",
                ShutDown::Abort => "/a",
            };
            let output: Output;
            if options == ShutDown::Abort {
                output = Command::new("shutdown")
                .args([args_to_pass])
                .output()
                .expect("Failed to execute command");
            }
            else {
                output = Command::new("shutdown")
                .args([args_to_pass,"/t",timeout])
                .output()
                .expect("Failed to execute command");
            }
           
            Self::validate_output(output)
        }

        pub fn get_shutdown_options(&mut self,command_output: &mut String, ui: &mut Ui) {
            ui.horizontal(|ui|{
                for op in ShutDown::iter() {
                    let option_str = format!("{:?}", op);
                    let res = ui.add_sized([50., button_height], egui::Button::new(option_str));
    
                    if res.clicked() {
                        *command_output = Self::cmd_tasks(op,&self.timeout).unwrap_or("An Error happened".to_owned());
                    }
                }
                
                ui.add(egui::TextEdit::singleline(&mut self.timeout));
                ui.label("timeout")
            });
            
        }
    }
}
