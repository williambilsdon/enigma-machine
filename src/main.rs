use app::App;

use enigma::EnigmaMachine;
use std::io::Error;

mod app;
mod enigma;

fn main() -> Result<(), Error> {
    let _enigma_machine = EnigmaMachine::new();
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
