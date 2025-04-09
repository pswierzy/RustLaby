use rand::Rng;

fn main() {
    let password_length: u8 = 10;
    let charsets: [&str; 4] = ["lowercase", "uppercase", "digits", "special"];
    let password: String = generate_password(password_length, &charsets);
    println!("Wygenerowane hasło: {}", password);
}

fn generate_password(leng: u8, charsets: &[&str]) -> String {
    let lowercase: [char; 26] = [
        'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'
    ];
    let uppercase: [char; 26] = [
        'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'
    ];
    let digits: [char; 10] = [
        '0','1','2','3','4','5','6','7','8','9'
    ];
    let special: [char; 14] = [
        '!','@','#','$','%','^','&','*','(',')','-','_','=','+'
    ];

    let mut characters: Vec<char> = Vec::new();

    for &charset in charsets {
        match charset {
            "lowercase" => characters.extend_from_slice(&lowercase),
            "uppercase" => characters.extend_from_slice(&uppercase),
            "digits"    => characters.extend_from_slice(&digits),
            "special"   => characters.extend_from_slice(&special),
            _           => {}
        }
    }

    if characters.is_empty() {
        panic!("Brak poprawnych zestawów znaków.");
    }

    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let mut passwd = String::new();

    for _ in 0..leng {
        let index = rng.random_range(0..characters.len());
        passwd.push(characters[index]);
    }

    passwd
}
