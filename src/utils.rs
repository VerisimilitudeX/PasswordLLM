pub mod PwnedAPI {
    pub async fn pass_check(password: &str) -> u64 {
        let pwned_api = pwned::api::PwnedBuilder::default().build().unwrap();
        let request = pwned_api.check_password(password).await;

        if request.is_ok() {
            request.unwrap().count
        }
        else {
            0
        }
    }
}