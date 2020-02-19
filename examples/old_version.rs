struct KeyPair(String);

fn old_do_some_crypto_computation(
    password1: String,
    password2: String,
) -> Result<String, std::io::Error> {
    Ok(password1 + &password2)
}

fn old_get_keypair(keys: &mut KeyPair) -> Result<(), std::io::Error> {
    let password1 = rpassword::prompt_password_stdout("password1: ")?;
    let password2 = rpassword::prompt_password_stdout("password2: ")?;
    keys.0 = old_do_some_crypto_computation(password1, password2)?;
    Ok(())
}

pub fn main() {
    let mut keys = KeyPair(String::new());
    old_get_keypair(&mut keys).unwrap();
    println!("{}", keys.0);
}
