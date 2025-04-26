use machine::EnigmaMachine;
use std::collections::HashMap;

pub mod machine;

pub struct EnigmaBuilder(EnigmaMachine);

impl EnigmaBuilder {
    pub fn new() -> Self {
        EnigmaBuilder(EnigmaMachine {
            rotors: Vec::new(),
            plugboard: HashMap::new(),
        })
    }

    pub fn with_enigma_i(mut self) -> Self {
        // ['e', 'k', 'm', 'f', 'l', 'g', 'd', 'q', 'v', 'z', 'n', 't', 'o', 'w', 'y', 'h', 'x', 'u', 's', 'p', 'a', 'i', 'b', 'r', 'c', 'j']  // I 1930 	Enigma I
        let rotor = [
            4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17,
            2, 9,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_ii(mut self) -> Self {
        // ['a', 'j', 'd', 'k', 's', 'i', 'r', 'u', 'x', 'b', 'l', 'h', 'w', 't', 'm', 'c', 'q', 'g', 'z', 'n', 'p', 'y', 'f', 'v', 'o', 'e' ] // II 1930 	Enigma I
        let rotor = [
            0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21,
            14, 4,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_iii(mut self) -> Self {
        //['b', 'd', 'f', 'h', 'j', 'l', 'c', 'p', 'r', 't', 'x', 'v', 'z', 'n', 'y', 'e', 'i', 'w', 'g', 'a', 'k', 'm', 'u', 's', 'q', 'o']  // III 1930 	Enigma I
        let rotor = [
            1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18,
            16, 14,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_iv(mut self) -> Self {
        // ['e', 's', 'o', 'v', 'p', 'z', 'j', 'a', 'y', 'q', 'u', 'i', 'r', 'h', 'x', 'l', 'n', 'f', 't', 'g', 'k', 'd', 'c', 'm', 'w', 'b']  // IV December 1938 	M3 Army
        let rotor = [
            4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12,
            22, 1,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_v(mut self) -> Self {
        //['v', 'z', 'b', 'r', 'g', 'i', 't', 'y', 'u', 'p', 's', 'd', 'n', 'h', 'l', 'x', 'a', 'w', 'm', 'j', 'q', 'o', 'f', 'e', 'c', 'k']  // V December 1938 	M3 Army
        let rotor = [
            21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4,
            2, 10,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_vi(mut self) -> Self {
        //['j', 'p', 'g', 'v', 'o', 'u', 'm', 'f', 'y', 'q', 'b', 'e', 'n', 'h', 'z', 'r', 'd', 'k', 'a', 's', 'x', 'l', 'i', 'c', 't', 'w'] //  VI 1939 	M3 & M4 Naval (FEB 1942)
        let rotor = [
            9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2,
            19, 22,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_vii(mut self) -> Self {
        //['n', 'z', 'j', 'h', 'g', 'r', 'c', 'x', 'm', 'y', 's', 'w', 'b', 'o', 'u', 'f', 'a', 'i', 'v', 'l', 'p', 'e', 'k', 'q', 'd', 't'] // VII 1939 	M3 & M4 Naval (FEB 1942)
        let rotor = [
            13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16,
            3, 19,
        ];
        self.0.rotors.push(rotor);
        self
    }
    pub fn with_enigma_viii(mut self) -> Self {
        //['f', 'k', 'q', 'h', 't', 'l', 'x', 'o', 'c', 'b', 'j', 's', 'p', 'd', 'z', 'r', 'a', 'm', 'e', 'w', 'n', 'i', 'u', 'y', 'g', 'v'] // VIII 1939 	M3 & M4 Naval (FEB 1942)
        let rotor = [
            5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24,
            6, 21,
        ];
        self.0.rotors.push(rotor);
        self
    }

    pub fn build(self) -> EnigmaMachine {
        self.0
    }
}
