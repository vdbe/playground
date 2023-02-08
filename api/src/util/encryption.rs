use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use crate::error::ResultRepr;

fn argon2_config<'a>() -> Argon2<'a> {
    Argon2::default()
}

pub(crate) async fn hash_password(password: String) -> ResultRepr<String> {
    // TODO: Add pepper
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = argon2_config();

        let result = argon2.hash_password(password.as_bytes(), &salt);

        let error = match result {
            Ok(result) => send.send(Ok(result.to_string())).is_err(),
            Err(err) => send.send(Err(err)).is_err(),
        };
        if error {
            tracing::error!("the receiver dropped");
        }
    });

    Ok(recv.await??)
}

pub(crate) async fn verify_password(
    password: String,
    hash: String,
) -> ResultRepr<bool> {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let result = match PasswordHash::new(&hash) {
            Ok(parsed_hash) => {
                let result = Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();

                Ok(result)
            }
            Err(err) => Err(err),
        };

        if send.send(result).is_err() {
            tracing::error!("the receiver dropped");
        };
    });

    Ok(recv.await??)
}
