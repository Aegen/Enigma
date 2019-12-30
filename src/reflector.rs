pub static REFLECTOR_MAP: [i32; 26] = [
    24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19,
];

/// Data representing a single rotor
pub struct Reflector {
    wire_map: [i32; 26],
}

impl Reflector {
    /// Contructor for making a Reflector
    pub fn new() -> Reflector {
        Reflector {
            wire_map: REFLECTOR_MAP,
        }
    }

    /// Used to convert the pin
    ///
    /// #Arguments
    /// * `pin` - The index of the pin the be converted
    pub fn reflect(&self, pin: i32) -> i32 {
        let index = pin as usize;
        self.wire_map[index]
    }
}
