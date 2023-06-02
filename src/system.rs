pub mod SystemInfo {
    use std::process::{Command, Output};
    use eframe::egui::{self, Ui};
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter;

    use crate::button_height;
    //wmic path Win32_VideoController
    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum SystemInfo {
        Cpu,
        VideoCard,
        DiskDrive,
        #[default]
        Os,
        NetWorkAdapters
    }
  
    

    impl std::fmt::Display for SystemInfo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let variant_str = match self {
                SystemInfo::Cpu => "Cpu",
                SystemInfo::VideoCard => "VideoCard",
                SystemInfo::DiskDrive => "DiskDrive",
                SystemInfo::Os => "OS",
                SystemInfo::NetWorkAdapters => "NetWorkAdapters",
               
            };
            
            write!(f, "{}", variant_str)
        }
    }
    #[derive(Debug, Default)]
    pub struct Networking {}
    impl SystemInfo {

        pub fn validate_output (output: Output) -> Option<String>{
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
        pub fn cmd_tasks(options : SystemInfo) -> Option<String>{
            let output:Output;
            if options == SystemInfo::Os {
                output = Command::new("systeminfo")
                .output()
                .expect("Failed to execute command");
            }
            else if options == SystemInfo::Cpu{
                output = Command::new("wmic").args(["cpu","list","full"]).output()
                    .expect("Failed to execute command");
            }
            else if options == SystemInfo::VideoCard{
                output = Command::new("wmic").args(["path","Win32_VideoController","get","/format:list"]).output()
                    .expect("Failed to execute command");
            }
            else if options == SystemInfo::DiskDrive{
                output = Command::new("wmic").args(["path","Win32_DiskDrive","get","/format:list"]).output()
                    .expect("Failed to execute command");
            }
            else {
                output = Command::new("wmic").args(["path","Win32_NetworkAdapter","get","/format:list"]).output()
                    .expect("Failed to execute command");
            }
            Self::validate_output(output)
        }

        pub fn get_system_options(command_output: &mut String, ui: &mut Ui) {
       
            for op in SystemInfo::iter() {
                let option_str = format!("{:?}", op);
                let res = ui.add_sized([50., button_height], egui::Button::new(option_str));
                
                if res.clicked() {
                    *command_output = Self::cmd_tasks(op)
                        .unwrap_or("An Error happened".to_owned());
                } 
            }
            
            
            
        }
      
    }
}