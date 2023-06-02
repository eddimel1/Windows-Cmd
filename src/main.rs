use eframe::egui::{self, SidePanel};
use shutdown::ShutDown::ShutDownScripts;
use std::{cell::RefCell, ffi::CString, rc::Rc};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tasks::Tasks::Tasks;
mod networking;
mod shutdown;
mod system;
mod tasks;
mod utility;

extern "C" {
    pub fn system(command: *const i8);
}
const button_height: f32 = 35.;

#[derive(Debug, Default, PartialEq, EnumIter)]
pub enum ScriptCategories {
    #[default]
    Networking,
    System,
    Tasks,
    Utility,
    Shutdown,
}

impl std::fmt::Display for ScriptCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_str = match self {
            ScriptCategories::Networking => "Networking",
            ScriptCategories::System => "System",
            ScriptCategories::Tasks => "Tasks",
            ScriptCategories::Utility => "Utility",
            ScriptCategories::Shutdown => "Shutdown",
        };
        write!(f, "{}", variant_str)
    }
}

struct App {
    command_output: String,
    selected: ScriptCategories,
    tasks: Rc<RefCell<Tasks>>,
    shutdown: Rc<RefCell<ShutDownScripts>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            SidePanel::left("sidebar").show(ctx, |ui| {
                ui.set_min_width(200.);
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        let button_width = ui.available_width();
                        ui.set_width(button_width);
                        for op in ScriptCategories::iter() {
                            let button = ui.add_sized(
                                [button_width, button_height],
                                egui::Button::new(op.to_string()),
                            );
                            if button.clicked() {
                                self.command_output = String::new();
                                self.selected = op;
                            }
                        }
                    },
                );
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(20.0);
                        if self.selected == ScriptCategories::Networking {
                            networking::NetWorking::Networking::get_network_options(
                                &mut self.command_output,
                                ui,
                            );
                        } else if self.selected == ScriptCategories::System {
                            system::SystemInfo::SystemInfo::get_system_options(
                                &mut self.command_output,
                                ui,
                            )
                        } else if self.selected == ScriptCategories::Utility {
                            utility::Utility::Utility::get_utility_options(
                                &mut self.command_output,
                                ui,
                            )
                        } else if self.selected == ScriptCategories::Shutdown {
                            self.shutdown
                                .borrow_mut()
                                .get_shutdown_options(&mut self.command_output, ui);
                        } else if self.selected == ScriptCategories::Tasks {
                            self.tasks
                                .borrow_mut()
                                .get_task_options(&mut self.command_output, ui);
                        }
                    });
                    ui.add_space(20.0);
                });
                ui.add_space(70.);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        for line in self.command_output.lines() {
                            ui.label(line);
                        }
                    });
                })
            })
        });
    }
}

fn main() {
    unsafe {
        system(CString::new("chcp 65001").unwrap().as_ptr());
        system(CString::new("chcp 437").unwrap().as_ptr());
    }
    let mut win_option = eframe::NativeOptions::default();
    let tasks = Rc::new(RefCell::new(Tasks::new()));
    let shutdown = Rc::new(RefCell::new(ShutDownScripts::new()));
    let app = App {
        command_output: String::new(),
        selected: ScriptCategories::Networking,
        tasks: Rc::clone(&tasks),
        shutdown: Rc::clone(&shutdown), // Pass the tasks instance as a reference to the App struct
    };
    win_option.initial_window_size = Some(egui::Vec2::new(1100., 960.));
    eframe::run_native("myapp", win_option, Box::new(|_cc| Box::<App>::new(app)));
}
