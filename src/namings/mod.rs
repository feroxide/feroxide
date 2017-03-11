fn basic_number_to_greek(n: u8, tenplus: bool) -> String {
    if n == 0 {
        "0?".to_owned()
    }

    else if n == 1 && !tenplus  { "mono".to_owned() }
    else if n == 1 && tenplus   { "hen".to_owned()  }

    else if n == 2 && !tenplus  { "di".to_owned() }
    else if n == 2 && tenplus   { "do".to_owned() }

    else if n == 20 && !tenplus { "icosa".to_owned() }
    else if n == 20 && tenplus  { "cosa".to_owned()  }

    else if n == 3  { "tri".to_owned() }
    else if n == 4  { "tetra".to_owned() }
    else if n == 5  { "penta".to_owned() }
    else if n == 6  { "hexa".to_owned() }
    else if n == 7  { "hepta".to_owned() }
    else if n == 8  { "octa".to_owned() }
    else if n == 9  { "nona".to_owned() }
    else if n == 10 { "deca".to_owned() }
    else if n == 11 { "undeca".to_owned() }
    else if n == 12 { "dodeca".to_owned() }
    else if n == 30 { "triaconta".to_owned() }


    else {
        n.to_string().to_owned() + " unknown @basic"
    }
}

fn is_vowel(letter: char) -> bool {
    letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
}

fn last_char(word: &str) -> char {
    word.chars().nth(word.len()-1).unwrap()
}

pub fn number_to_greek(n: u8) -> String {
    if n <= 12 || n == 20 || n == 30 {
        basic_number_to_greek(n, false)
    }

    else if n >= 13 && n < 20 {
        basic_number_to_greek(n - 10, true) + "deca"
    }

    else if n > 20 && n < 30 {
        let prefix = basic_number_to_greek(n - 20, true);
        let suffix = if is_vowel(last_char(&prefix)) { "cosa" } else { "icosa" };

        prefix + suffix
    }

    else if n > 30 && n < 40 {
        basic_number_to_greek(n - 30, true) + "triaconta"
    }

    else if n >= 40 && n < 100 {
        let factor_ten: u8 = n / 10;

        if n == factor_ten*10 {
            return basic_number_to_greek(factor_ten, true) + "conta";
        } else {
            return basic_number_to_greek(n - factor_ten*10, true) + &basic_number_to_greek(factor_ten, true) + "conta";
        }
    }

    else {
        n.to_string().to_owned() + " unknown @pub"
    }
}


pub fn number_to_roman(n: i8) -> String {
    if n < 0 {
        "-".to_owned() + &number_to_roman(-n)
    }

    else if n == 0 { "0".to_owned() }
    else if n == 1 { "I".to_owned() }
    else if n == 2 { "II".to_owned() }
    else if n == 3 { "III".to_owned() }
    else if n == 4 { "IV".to_owned() }
    else if n == 5 { "V".to_owned() }
    else if n == 6 { "VI".to_owned() }
    else if n == 7 { "VII".to_owned() }
    else if n == 8 { "VIII".to_owned() }
    else if n == 9 { "IX".to_owned() }
    else if n == 10 { "X".to_owned() }
    else if n == 11 { "XI".to_owned() }
    else if n == 12 { "XII".to_owned() }
    else if n == 13 { "XIII".to_owned() }
    else if n == 14 { "XIV".to_owned() }
    else if n == 15 { "XV".to_owned() }
    else if n == 16 { "XVI".to_owned() }

    else {
        "?".to_owned()
    }
}


pub fn subscript(n: u8) -> String {
    if n >= 10 {
        return subscript(n / 10) + &subscript(n % 10);
    }

    else if n == 0  { "₀".to_owned() }
    else if n == 1  { "₁".to_owned() }
    else if n == 2  { "₂".to_owned() }
    else if n == 3  { "₃".to_owned() }
    else if n == 4  { "₄".to_owned() }
    else if n == 5  { "₅".to_owned() }
    else if n == 6  { "₆".to_owned() }
    else if n == 7  { "₇".to_owned() }
    else if n == 8  { "₈".to_owned() }
    else if n == 9  { "₉".to_owned() }

    else {
        "?".to_owned()
    }
}
