use capture_screen::{capture_screen};
use eframe::{egui, App, run_native};

mod capture_screen;

    

fn main() {
    // Crea una finestra nativa
    let options = eframe::NativeOptions::default();
    run_native(
        "rustream",  // Titolo della finestra
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default())))
    );
}

// Definisci la tua struttura MyApp
#[derive(Default)]
struct MyApp {
    button1_clicked: bool,
    button2_clicked: bool,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Creiamo un pannello centrale
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.horizontal_centered(|ui| {
                

                ui.vertical_centered( |ui| {
                    
                    ui.add_space(40.0);
                    ui.heading("rustream");
                    ui.add_space(40.0);
                    
                    // Primo bottone
                    if ui.button("Cast New Streaming").clicked() {
                        self.button1_clicked = true;
                        self.button2_clicked = false;
                    }
                    
                    ui.add_space(20.0);
    
                    // Secondo bottone
                    if ui.button("View Streaming").clicked() {
                        self.button2_clicked = true;
                        self.button1_clicked = false;
                    }
        
                    // Mostra il messaggio relativo al bottone cliccato
                    if self.button1_clicked {
                        ui.label("Hai cliccato Cast New Streaming");
                    } else if self.button2_clicked {
                        ui.label("Hai cliccato View Existing Streaming");
                    }
                });
            });
  
        });
    }
}

