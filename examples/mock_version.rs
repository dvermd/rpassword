#[cfg(test)]
use mockall::*;

struct KeyPair(String);

#[cfg_attr(test, automock)]
trait UserPasswordInput {
    fn get_password(&self, prompt: &str) -> std::io::Result<String>;
}

impl UserPasswordInput for std::io::Stdin {
    fn get_password(&self, prompt: &str) -> std::io::Result<String> {
        Ok(rpassword::prompt_password_stdout(prompt)?)
    }
}

fn mock_do_some_crypto_computation(
    password1: String,
    password2: String,
) -> Result<String, std::io::Error> {
    Ok(password1 + &password2)
}

fn mock_get_keypair(keys: &mut KeyPair) -> Result<(), std::io::Error> {
    mock_inner_get_keypair(std::io::stdin(), keys)
}

fn mock_inner_get_keypair<T: UserPasswordInput>(
    stdin: T,
    keys: &mut KeyPair,
) -> Result<(), std::io::Error> {
    let password1 = stdin.get_password("password1: ")?;
    let password2 = stdin.get_password("password2: ")?;
    keys.0 = mock_do_some_crypto_computation(password1, password2)?;
    Ok(())
}

#[test]
fn test_inner_get_pair() {
    let mut keys = KeyPair(String::new());
    let mut stdin_mock = MockUserPasswordInput::new();
    stdin_mock.expect_get_password().returning(|prompt| {
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
    });

    mock_inner_get_keypair(stdin_mock, &mut keys).unwrap();
    assert_eq!(keys.0, String::from("abcd1234"));
}

pub fn main() {
    let mut keys = KeyPair(String::new());
    mock_get_keypair(&mut keys).unwrap();
    println!("{}", keys.0);
}
