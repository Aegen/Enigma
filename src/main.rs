mod reflector;
mod rotors;

use std::collections::HashMap;

fn main() {
    // Maps numbers to letters
    let alphabet = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    // Maps letters to numbers
    // Kodad was x
    let mut alpha_in = HashMap::new();
    alpha_in.entry("A").or_insert(0);
    alpha_in.entry("B").or_insert(1);
    alpha_in.entry("C").or_insert(2);
    alpha_in.entry("D").or_insert(3);
    alpha_in.entry("E").or_insert(4);
    alpha_in.entry("F").or_insert(5);
    alpha_in.entry("G").or_insert(6);
    alpha_in.entry("H").or_insert(7);
    alpha_in.entry("I").or_insert(8);
    alpha_in.entry("J").or_insert(9);
    alpha_in.entry("K").or_insert(10);
    alpha_in.entry("L").or_insert(11);
    alpha_in.entry("M").or_insert(12);
    alpha_in.entry("N").or_insert(13);
    alpha_in.entry("O").or_insert(14);
    alpha_in.entry("P").or_insert(15);
    alpha_in.entry("Q").or_insert(16);
    alpha_in.entry("R").or_insert(17);
    alpha_in.entry("S").or_insert(18);
    alpha_in.entry("T").or_insert(19);
    alpha_in.entry("U").or_insert(20);
    alpha_in.entry("V").or_insert(21);
    alpha_in.entry("W").or_insert(22);
    alpha_in.entry("X").or_insert(23);
    alpha_in.entry("Y").or_insert(24);
    alpha_in.entry("Z").or_insert(25);

    for x in &alphabet {
        let inp = alpha_in[x];

        let result = run_input(inp);
        let inverse_result = run_input(result);

        println!("================");
        println!("Input:         {}", x);
        println!("First output:  {}", alphabet[result as usize]);
        println!("Second output: {}", alphabet[inverse_result as usize]);
    }
}

fn run_input(letter: i32) -> i32 {
    let mut rotor1 = rotors::Rotor::new(rotors::ENIGMA1_1, rotors::ENIGMA1_1_INVERSE, 0);
    let mut rotor2 = rotors::Rotor::new(rotors::ENIGMA1_2, rotors::ENIGMA1_2_INVERSE, 0);
    let mut rotor3 = rotors::Rotor::new(rotors::ENIGMA1_3, rotors::ENIGMA1_3_INVERSE, 0);

    let the_reflector = reflector::Reflector::new();

    // Pass through the rotors
    let phase_1 = rotor3.map_wire(rotor2.map_wire(rotor1.map_wire(letter)));

    // Pass through the reflector
    let phase_2 = the_reflector.reflect(phase_1);

    // Pass back through the rotors
    let phase_3 =
        rotor1.map_reverse_wire(rotor2.map_reverse_wire(rotor3.map_reverse_wire(phase_2)));

    // These ticks exist just to make the compiler shutup about unused code
    rotor1.tick();
    rotor2.tick();
    rotor3.tick();

    phase_3
}
