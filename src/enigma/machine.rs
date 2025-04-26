use std::collections::HashMap;

type Rotor = [usize; 26];

const ALPHABETIC_INDEX: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub struct EnigmaMachine {
    pub rotors: Vec<Rotor>,
    pub plugboard: HashMap<char, char>,
}

impl EnigmaMachine {
    pub fn encrypt_msg(&self, input: String) -> String {
        input.chars().map(|ch| self.encrypt_char(ch)).collect()
    }

    pub fn encrypt_char(&self, input_ch: char) -> char {
        self.rotor_passthroughs(input_ch.to_ascii_lowercase())
    }

    fn rotor_passthroughs(&self, input_ch: char) -> char {
        let input_idx = ALPHABETIC_INDEX.binary_search(&input_ch).unwrap();
        let running_idx = self.rotors.iter().fold(input_idx, |running_idx, rotor| {
            rotor.get(running_idx).unwrap_or(&running_idx).to_owned()
        });

        let reflected_idx = self
            .rotors
            .iter()
            .rev()
            .fold(running_idx, |running_idx: usize, rotor| {
                rotor.get(running_idx).unwrap_or(&running_idx).to_owned()
            });

        ALPHABETIC_INDEX.get(reflected_idx).unwrap().to_owned()
    }
}
