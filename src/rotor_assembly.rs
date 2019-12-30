use crate::reflector;
use crate::rotors;

/// Contains the group of 3 rotors along with the reflector
pub struct RotorAssembly {
    rotor_group: [rotors::Rotor; 3],
    end_reflector: reflector::Reflector,
}

impl RotorAssembly {
    /// The constructor, accepts 3 RotorProp objects containing the settings
    /// for the 3 Rotors
    ///
    /// # Arguments
    /// * `prop_1` - Contains the properties for the first Rotor
    /// * `prop_2` - Contains the properties for the second Rotor
    /// * `prop_3` - Contains the properties for the third Rotor
    pub fn new(
        prop_1: rotors::RotorProp,
        prop_2: rotors::RotorProp,
        prop_3: rotors::RotorProp,
    ) -> RotorAssembly {
        let rotor_set: [rotors::Rotor; 3] = [
            rotors::Rotor::new(prop_3.map, prop_3.cursor),
            rotors::Rotor::new(prop_2.map, prop_2.cursor),
            rotors::Rotor::new(prop_1.map, prop_1.cursor),
        ];

        let refl: reflector::Reflector = reflector::Reflector::new();

        RotorAssembly {
            rotor_group: rotor_set,
            end_reflector: refl,
        }
    }

    /// Calls the tick methods for each Rotor when appropriate
    fn tick(&mut self) {
        let mut carry = true;

        for x in &mut self.rotor_group {
            if carry {
                carry = x.tick();
            }
        }
    }

    /// Sends a single character through the rotors and returns the numeric
    /// representation of the result
    ///
    /// # Arguments
    /// `letter` - The 0..25 representation of the letter to be converted
    pub fn run(&mut self, letter: i32) -> i32 {
        self.tick();

        // Pass through the rotors
        let phase_1 = self.rotor_group[2]
            .map_wire(self.rotor_group[1].map_wire(self.rotor_group[0].map_wire(letter)));

        // Pass through the reflector
        let phase_2 = self.end_reflector.reflect(phase_1);

        // Pass back through the rotors
        let phase_3 = self.rotor_group[0].map_reverse_wire(
            self.rotor_group[1].map_reverse_wire(self.rotor_group[2].map_reverse_wire(phase_2)),
        );

        phase_3
    }
}
