pub mod NetWorking {
    use std::process::{Command, Output};
    use eframe::egui::{self, Ui};
    use strum::IntoEnumIterator; // 0.17.1
    use strum_macros::EnumIter;

    use crate::button_height;

    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum IpConfigOptions {
        ResetIp,
        ResetIpv6,
        ResetDns,
        #[default]
        IpConfig,
        IpconfigAll,
        DisplayDns,
    }
    #[derive(Debug, Default, PartialEq, EnumIter)]
    pub enum NetStatOptions {
        #[default]
        TCPConnections,
        EthernetStatistics,
        RoutingTable,
    }

    impl std::fmt::Display for IpConfigOptions {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let variant_str = match self {
                IpConfigOptions::ResetIp => "Reset Ip",
                IpConfigOptions::ResetIpv6 => "Reset v6 Ip",
                IpConfigOptions::ResetDns => "Reset Dns",
                IpConfigOptions::IpConfig => "IpConfig",
                IpConfigOptions::IpconfigAll => "Ip Config Expanded",
                IpConfigOptions::DisplayDns => "Display Dns",
            };
            
            write!(f, "{}", variant_str)
        }
    }
    #[derive(Debug, Default)]
    pub struct Networking {}
    impl Networking {

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
        pub fn ipconfig(args: Option<IpConfigOptions>) -> Option<String> {
            let args_to_pass = match args {
                Some(IpConfigOptions::IpconfigAll) => "/all",
                Some(IpConfigOptions::IpConfig) => "/",
                Some(IpConfigOptions::ResetDns) => "/flushdns",
                Some(IpConfigOptions::DisplayDns) => "/displaydns",
                Some(IpConfigOptions::ResetIp) => "/release",
                Some(IpConfigOptions::ResetIpv6) => "/release6",
                None => "",
            };
            let output: Output;
            if (args == Some(IpConfigOptions::IpConfig)) {
                output = Command::new("ipconfig")
                    .output()
                    .expect("Failed to execute command");
            } 
            else if (args == Some(IpConfigOptions::ResetIp)){
                output = Command::new("ipconfig").args(["/release"  , "renew"])
                .output()
                .expect("Failed to execute command");
            }

            else if (args == Some(IpConfigOptions::ResetIpv6)){
                output = Command::new("ipconfig").args(["/release6"  , "renew6"])
                .output()
                .expect("Failed to execute command");
            }
            else {
                output = Command::new("ipconfig")
                    .args([args_to_pass])
                    .output()
                    .expect("Failed to execute command");
            }
            Self::validate_output(output)
        }

        pub fn get_network_options(command_output: &mut String, ui: &mut Ui) {
       
        
            for op in IpConfigOptions::iter() {
                let option_str = format!("{:?}", op);
                let res = ui.add_sized([50., button_height], egui::Button::new(option_str));
                
                if res.clicked() {
                    *command_output = Self::ipconfig(Some(op))
                        .unwrap_or("Successful".to_owned());
                } 
            }
            
            for op in NetStatOptions::iter() {
                let option_str = format!("{:?}", op);
                let res = ui.add_sized([50., button_height], egui::Button::new(option_str));
                
                if res.clicked() {
                    *command_output = Self::netstat(op)
                        .unwrap();
                }
            }
            
        }
        pub fn netstat(args : NetStatOptions) -> Option<String>{
                let args_to_pass = match args {
                    NetStatOptions::TCPConnections => "/a",
                    NetStatOptions::EthernetStatistics => "/e",
                    NetStatOptions::RoutingTable => "/r",
                };
                let output = Command::new("netstat").args([args_to_pass]).output().expect("Failed to execute command");
                Self::validate_output(output)
        }
    }
}
