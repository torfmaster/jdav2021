use shared::UserAuth;

#[derive(Debug)]
enum AuthError {
    InvalidAuthHeader,
    InvalidBase64,
}

fn extract_basicauth(header: String) -> Result<UserAuth, AuthError> {
    let stripped = header.split("Basic ").collect::<Vec<&str>>();
    let stripped = match stripped.as_slice() {
        [_, ref base64_content] => Ok((base64_content).to_owned()),
        _ => Err(AuthError::InvalidAuthHeader),
    }?;
    let decoded =
        String::from_utf8(base64::decode(stripped).map_err(|_| AuthError::InvalidBase64)?)
            .map_err(|_| AuthError::InvalidBase64)?;

    let split = decoded.split(":").collect::<Vec<&str>>();
    match split.as_slice() {
        [ref name, ref pass, ..] => Ok(UserAuth {
            name: (name.to_owned()).to_owned(),
            pass: (pass.to_owned()).to_owned(),
        }),
        _ => Err(AuthError::InvalidAuthHeader),
    }
}

#[cfg(test)]
pub mod test {
    use shared::UserAuth;

    use super::extract_basicauth;

    #[test]
    pub fn works() {
        let pass_entry = base64::encode(b"wurst:kaese");
        let user_auth = extract_basicauth(format!("Basic {}", pass_entry));
        assert_eq!(
            user_auth.unwrap(),
            UserAuth {
                name: "wurst".to_owned(),
                pass: "kaese".to_owned(),
            },
        )
    }
}
