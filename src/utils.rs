pub mod PwnedAPI {

    use reqwest;
    use sha1::{Sha1, Digest};
    use anyhow::{Context, Result};

    #[tokio::main(flavor = "current_thread")]
    pub async fn pass_check (pass: String) -> Result<()> {
        
        let mut hasher = Sha1::new();
        hasher.update(pass);
        let digest = hex::encode_upper(hasher.finalize());
        let (prefix, suffix) = (&digest[..5], &digest[5..]);

        // API requires us to submit just the first 5 characters of the hash

        let url = format!("https://api.pwnedpasswords.com/range/{prefix}");
        let response = reqwest::get(&url)
            .await
            .with_context(|| format!("failed to GET {url}"))?;

        let body = response
            .text()
            .await
            .context("failed to parse request as text")?;

        // Reponse is a series of lines like
        //
        //  suffix:N
        //
        // Where N is the number of times that password has appeared.

        let matching_prefix = format!("{suffix}:");

        for line in body.lines() {
            if let Some(count) = line.strip_prefix(&matching_prefix) {
                let times = if count == "1" { "time" } else { "times" };
                println!("Your password appears in the database {count} {times}.");
                return Ok(());
            }
        }

        println!("No matches found.");

        Ok(())
    }
}
