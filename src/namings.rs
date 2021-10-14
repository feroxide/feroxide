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
pub fn number_to_greek(n: u8) -> String {
    if n <= 12 || n == 20 || n == 30 {
        basic_number_to_greek(n, false)
    } else if n < 20 {
        basic_number_to_greek(n - 10, true) + &basic_number_to_greek(10, false)
    } else if n < 30 {
        let prefix = basic_number_to_greek(n - 20, true);
        let suffix = if is_vowel!(last_char(&prefix)) {
            "cosa"
        } else {
            "icosa"
        };

        prefix + suffix
    } else if n < 40 {
        basic_number_to_greek(n - 30, true) + &basic_number_to_greek(30, true)
    } else if n < 100 {
        let factor_ten: u8 = n / 10;

        if n == factor_ten * 10 {
            basic_number_to_greek(factor_ten, true) + "conta"
        } else {
            basic_number_to_greek(n - factor_ten * 10, true)
                + &basic_number_to_greek(factor_ten, true)
                + "conta"
        }
    } else {
        panic!("{} uncalculatable", n.to_string());
    }
}

/// Convert a number to roman notaion
pub fn number_to_roman(n: i8) -> String {
    if n < 0 {
        "-".to_owned() + &number_to_roman(-n)
    } else if n == 0 {
        // The romans didn't even have a 0, but for this purpose:
        "0".to_owned()
    } else if n == 1 {
        "I".to_owned()
    } else if n == 2 {
        "II".to_owned()
    } else if n == 3 {
        "III".to_owned()
    } else if n == 4 {
        "IV".to_owned()
    } else if n == 5 {
        "V".to_owned()
    } else if n == 6 {
        "VI".to_owned()
    } else if n == 7 {
        "VII".to_owned()
    } else if n == 8 {
        "VIII".to_owned()
    } else if n == 9 {
        "IX".to_owned()
    } else if n == 10 {
        "X".to_owned()
    } else if n == 11 {
        "XI".to_owned()
    } else if n == 12 {
        "XII".to_owned()
    } else if n == 13 {
        "XIII".to_owned()
    } else if n == 14 {
        "XIV".to_owned()
    } else if n == 15 {
        "XV".to_owned()
    } else if n == 16 {
        "XVI".to_owned()
    } else {
        panic!("{} uncalculatable", n.to_string());
    }
}

/// Convert a number to subscript notation
#[cfg(not(feature = "no_utf"))]
pub fn subscript(n: u8) -> String {
    if n >= 10 {
        subscript(n / 10) + &subscript(n % 10)
    } else if n == 0 {
        "₀".to_owned()
    } else if n == 1 {
        "₁".to_owned()
    } else if n == 2 {
        "₂".to_owned()
    } else if n == 3 {
        "₃".to_owned()
    } else if n == 4 {
        "₄".to_owned()
    } else if n == 5 {
        "₅".to_owned()
    } else if n == 6 {
        "₆".to_owned()
    } else if n == 7 {
        "₇".to_owned()
    } else if n == 8 {
        "₈".to_owned()
    } else if n == 9 {
        "₉".to_owned()
    } else {
        panic!("{} can't be converted to subscript.", n.to_string());
    }
}

#[cfg(feature = "no_utf")]
pub fn subscript(n: u8) -> String {
    format!("_{{{}}}", n)
}

/// Convert a number to superscript notation
/// See also `ion_superscript`
#[cfg(not(feature = "no_utf"))]
pub fn superscript(n: u8) -> String {
    if n == 0 {
        "⁰".to_owned()
    } else if n == 1 {
        "¹".to_owned()
    } else if n == 2 {
        "²".to_owned()
    } else if n == 3 {
        "³".to_owned()
    } else if n == 4 {
        "⁴".to_owned()
    } else if n == 5 {
        "⁵".to_owned()
    } else if n == 6 {
        "⁶".to_owned()
    } else if n == 7 {
        "⁷".to_owned()
    } else if n == 8 {
        "⁸".to_owned()
    } else if n == 9 {
        "⁹".to_owned()
    } else if n >= 10 {
        superscript(n / 10) + &superscript(n % 10)
    } else {
        panic!("{} can't be converted to superscript.", n.to_string());
    }
}

#[cfg(feature = "no_utf")]
pub fn superscript(n: u8) -> String {
    format!("{}", n)
}

/// Convert a number to ionic superscript notation
/// The difference with normal superscript notation is that the 1 is omitted,
/// also, ionic superscript supports negative numbers (of which the sign
/// is put at the end, instead of at the beginning)
/// For positive numbers, a plus-sign is appended too
#[cfg(not(feature = "no_utf"))]
pub fn ion_superscript(ac: &AtomCharge) -> String {
    let n = ac.0;

    match n {
        -1 => String::from("⁻"),
        1 => String::from("⁺"),
        n if n < 0 => superscript((-n) as u8) + &ion_superscript(&AtomCharge::from(-1)),
        n if n > 0 => superscript(n as u8) + &ion_superscript(&AtomCharge::from(1)),
        _ => superscript(n as u8),
    }
}

#[cfg(feature = "no_utf")]
pub fn ion_superscript(ac: &AtomCharge) -> String {
    let n = ac.0;

    match n {
        n if n < 0 => format!("^{{{}-}}", superscript(-n) as u8),
        n if n > 0 => format!("^{{{}+}}", superscript(n as u8)),
        _ => String::from("^{0}"),
    }
}
