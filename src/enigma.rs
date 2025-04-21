use std::collections::HashMap;

type Rotor = HashMap<char, char>;

pub struct EnigmaMachine {
    rotors: Vec<Rotor>,
    plugboard: HashMap<char, char>,
}

impl EnigmaMachine {
    pub fn new() -> Self {
        EnigmaMachine {
            rotors: Vec::new(),
            plugboard: HashMap::new(),
        }
    }

    pub fn encrypt_msg(&self, input: String) -> String {
        input.chars().map(|ch| self.encrypt_char(ch)).collect()
    }

    fn encrypt_char(&self, input_ch: char) -> char {
        let ch = self.rotors.iter().fold(input_ch, |ch, rotor| {
            rotor.get(&ch).unwrap_or(&ch).to_owned()
        });

        let reflected = self
            .rotors
            .iter()
            .rev()
            .fold(ch, |ch, rotor| rotor.get(&ch).unwrap_or(&ch).to_owned());

        reflected
    }
}
