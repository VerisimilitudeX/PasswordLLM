pub mod pwned_api {
    use crate::Error;

    pub async fn pass_check(password: &str) -> Result<u64, Error> {
        let pwned_api = pwned::api::PwnedBuilder::default().build().unwrap();
        Ok(pwned_api.check_password(password).await?.count)
    }
}