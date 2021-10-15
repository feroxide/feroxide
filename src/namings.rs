use types::*;

// Reference: https://www.shodor.org/unchem/basic/nomen/index.html
// Reference: BiNaS 6th edition, table 66C

/// The basic function which converts a number to greek
/// This function should only be called by the public `number_to_greek` function
fn basic_number_to_greek(n: u8, tenplus: bool) -> String {
    match n {
        0 => panic!("0 can't be converted to greek"),
        1 if !tenplus => String::from("mono"),
        1 if tenplus => String::from("hen"),
        2 if !tenplus => String::from("di"),
        2 if tenplus => String::from("do"),
        3 => String::from("tri"),
        4 => String::from("tetra"),
        5 => String::from("penta"),
        6 => String::from("hexa"),
        7 => String::from("hepta"),
        8 => String::from("octa"),
        9 => String::from("nona"),
        10 => String::from("deca"),
        11 => String::from("undeca"),
        12 => String::from("dodeca"),
        20 if !tenplus => String::from("icosa"),
        20 if tenplus => String::from("cosa"),
        30 => String::from("triaconta"),
        _ => panic!("{} uncalculatable", n),
    }
}

/// Get the last character of a word
fn last_char(word: &str) -> char {
    word.chars().nth(word.len() - 1).unwrap()
}

/// Convert a number to greek notation
/// TODO check for a potentially cleaner way to convert a number to greek
pub fn number_to_greek(n: u8) -> String {
    match n {
        n if n <= 12 => basic_number_to_greek(n, false),
        13..=19 => basic_number_to_greek(n - 10, true) + &basic_number_to_greek(10, false),
        20 => basic_number_to_greek(n, false),
        21..=29 => {
            let prefix = basic_number_to_greek(n - 20, true);
            let suffix = if is_vowel!(last_char(&prefix)) {
                "cosa"
            } else {
                "icosa"
            };

            prefix + suffix
        }
        30 => basic_number_to_greek(n, false),
        31..=39 => basic_number_to_greek(n - 30, true) + &basic_number_to_greek(30, true),
        40..=99 => {
            let factor_ten: u8 = n / 10;

            match n {
                n if n == factor_ten * 10 => basic_number_to_greek(factor_ten, true) + "conta",
                _ => {
                    basic_number_to_greek(n - factor_ten * 10, true)
                        + &basic_number_to_greek(factor_ten, true)
                        + "conta"
                }
            }
        }
        _ => panic!("{} uncalculatable", n),
    }
}

/// Convert a number to roman notaion
pub fn number_to_roman(n: i8) -> String {
    match n {
        //TODO a more elegant solution probobly exists here
        n if n < 0 => String::from("-") + &number_to_roman(-n),
        // The romans didn't even have a 0, but for this purpose:
        0 => String::from("0"),
        1 => String::from("I"),
        2 => String::from("II"),
        3 => String::from("III"),
        4 => String::from("IV"),
        5 => String::from("V"),
        6 => String::from("VI"),
        7 => String::from("VII"),
        8 => String::from("VIII"),
        9 => String::from("IX"),
        10 => String::from("X"),
        11 => String::from("XI"),
        12 => String::from("XII"),
        13 => String::from("XIII"),
        14 => String::from("XIV"),
        15 => String::from("XV"),
        16 => String::from("XVI"),
        _ => panic!("{} uncalculatable", n.to_string()),
    }
}

/// Convert a number to subscript notation
pub fn subscript(n: u8) -> String {
    match n {
        0 => String::from("₀"),
        1 => String::from("₁"),
        2 => String::from("₂"),
        3 => String::from("₃"),
        4 => String::from("₄"),
        5 => String::from("₅"),
        6 => String::from("₆"),
        7 => String::from("₇"),
        8 => String::from("₈"),
        9 => String::from("₉"),
        n if n >= 10 => subscript(n / 10) + &subscript(n % 10),
        _ => panic!("{} can't be converted to subscript.", n.to_string()),
    }
}

// TODO can this function be incorporated into the one above?
/* pub fn subscript(n: u8) -> String {
    format!("_{{{}}}", n)
} */

/// Convert a number to superscript notation
/// See also `ion_superscript`
pub fn superscript(n: i8) -> String {
    match n {
        0 => String::from("⁰"),
        1 => String::from("¹"),
        2 => String::from("²"),
        3 => String::from("³"),
        4 => String::from("⁴"),
        5 => String::from("⁵"),
        6 => String::from("⁶"),
        7 => String::from("⁷"),
        8 => String::from("⁸"),
        9 => String::from("⁹"),
        n if n >= 10 => superscript(n / 10) + &superscript(n % 10),
        _ => panic!("{} can't be converted to superscript.", n.to_string()),
    }
}

/* pub fn superscript(n: i8) -> String {
    format!("{}", n)
} */

/// Convert a number to ionic superscript notation
/// The difference with normal superscript notation is that the 1 is omitted,
/// also, ionic superscript supports negative numbers (of which the sign
/// is put at the end, instead of at the beginning)
/// For positive numbers, a plus-sign is appended too
pub fn ion_superscript(ac: &AtomCharge) -> String {
    let n = ac.0;

    match n {
        -1 => String::from("⁻"),
        1 => String::from("⁺"),
        n if n < 0 => superscript(-n) + &ion_superscript(&AtomCharge::from(-1)),
        n if n > 0 => superscript(n) + &ion_superscript(&AtomCharge::from(1)),
        _ => superscript(n),
    }
}

// TODO can this fucntion be incorporated into the one above?
/* pub fn ion_superscript(ac: &AtomCharge) -> String {
    let n = ac.0;

    match n {
        n if n < 0 => format!("^{{{}-}}", superscript(-n) as u8),
        n if n > 0 => format!("^{{{}+}}", superscript(n as u8)),
        _ => String::from("^{0}"),
    }
} */
