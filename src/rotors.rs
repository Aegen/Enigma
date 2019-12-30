// A bunch of arrays representing rotor mappings, probably going to move eventually
// static BASE_ALPHABET: [i32; 26] = [
//     0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
// ];

pub static ENIGMA1_1: [i32; 26] = [
    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9,
];

pub static ENIGMA1_2: [i32; 26] = [
    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4,
];

pub static ENIGMA1_3: [i32; 26] = [
    1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14,
];

/// Data representing a single rotor
pub struct Rotor {
    wire_map: [i32; 26],
    reverse_wire_map: [i32; 26],
    ticks: i32,  // Counts the ticks since the last carry over
    cursor: i32, // Tracks where the cursor is
}

impl Rotor {
    /// Contructor for making a Rotor
    ///
    /// # Arguments
    /// * `map` - An array 26 unique integers representing a mapping of letters,
    ///     used as the signal passes through initially
    /// * `cursor` - An integer representing what the rotor's initial position
    ///     should be
    pub fn new(map: [i32; 26], cursor: i32) -> Rotor {
        // Construct reverse map from input map, defaults to -1 to make errors obvious
        let mut inverse_map: [i32; 26] = [
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1,
        ];
        for x in 0..26 {
            inverse_map[map[x as usize] as usize] = x;
        }

        Rotor {
            wire_map: map,
            reverse_wire_map: inverse_map,
            ticks: 0,
            cursor: cursor,
        }
    }

    /// Increments the tick and cursor counters
    /// Returns true if rotor has completed a full rotation
    pub fn tick(&mut self) -> bool {
        self.ticks = (self.ticks + 1) % 26;
        self.cursor = (self.cursor + 1) % 26;

        if self.ticks == 0 {
            return true;
        }

        return false;
    }

    /// Used pre reflector to convert the input pin the output pin
    ///
    /// #Arguments
    /// * `pin` - The index of the pin the be converted
    pub fn map_wire(&self, pin: i32) -> i32 {
        let index = ((pin + self.cursor) % 26) as usize;
        self.wire_map[index]
    }

    /// Used post reflector to convert the input pin the output pin
    ///
    /// #Arguments
    /// * `pin` - The index of the pin the be converted
    pub fn map_reverse_wire(&self, pin: i32) -> i32 {
        let index = ((pin) % 26) as usize;
        ((self.reverse_wire_map[index] - self.cursor) + 26) % 26
    }
}
