struct KeyPair(String);

trait UserPasswordInput {
    fn get_password(&self, prompt: &str) -> std::io::Result<String>;
}

impl UserPasswordInput for std::io::Stdin {
    fn get_password(&self, prompt: &str) -> std::io::Result<String> {
        Ok(rpassword::prompt_password_stdout(prompt)?)
    }
}

fn new_do_some_crypto_computation(
    password1: String,
    password2: String,
) -> Result<String, std::io::Error> {
    Ok(password1 + &password2)
}

fn new_get_keypair(keys: &mut KeyPair) -> Result<(), std::io::Error> {
    new_inner_get_keypair(std::io::stdin(), keys)
}

fn new_inner_get_keypair<T: UserPasswordInput>(
    stdin: T,
    keys: &mut KeyPair,
) -> Result<(), std::io::Error> {
    let password1 = stdin.get_password("password1: ")?;
    let password2 = stdin.get_password("password2: ")?;
    keys.0 = new_do_some_crypto_computation(password1, password2)?;
    Ok(())
}

#[cfg(test)]
impl UserPasswordInput for () {
    fn get_password(&self, prompt: &str) -> std::io::Result<String> {
        if prompt.starts_with("password1:") {
            Ok("abcd".to_owned())
        } else if prompt.starts_with("password2:") {
            Ok("1234".to_owned())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("should not be called with {}", prompt),
            ))
        }
    }
}

#[test]
fn test_inner_get_pair() {
    let mut keys = KeyPair(String::new());
    new_inner_get_keypair((), &mut keys).unwrap();
    assert_eq!(keys.0, String::from("abcd1234"));
}

pub fn main() {
    let mut keys = KeyPair(String::new());
    new_get_keypair(&mut keys).unwrap();
    println!("{}", keys.0);
}
