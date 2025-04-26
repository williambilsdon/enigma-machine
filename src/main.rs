use app::App;

use enigma::EnigmaBuilder;
use std::io::Error;

mod app;
mod enigma;

fn main() -> Result<(), Error> {
    let enigma_machine = EnigmaBuilder::new()
        .with_enigma_i()
        .with_enigma_ii()
        .with_enigma_iii()
        .build();
    let terminal = ratatui::init();
    let app_result = App::new(enigma_machine).run(terminal);
    ratatui::restore();
    app_result
}
