use std::collections::HashSet;

use crate::domain::BannedTokenStore;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_token_store: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {

    async fn store_token(&mut self, token: String) -> Result<bool, bool>{
        self.banned_token_store.insert(token);
        Ok(true)
    }

    async fn token_exists(&self, token: &str) -> Result<bool, bool> {
        // match self.banned_token_store.contains(token) {
        //     true => return Ok(true),
        //     false => return Err(false) 
        // };
        Ok(self.banned_token_store.contains(token))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_add_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token".to_owned();

        let result = store.store_token(token.clone()).await;

        assert!(result.is_ok());
        assert!(store.banned_token_store.contains(&token));
    }

    #[tokio::test]
    async fn test_contains_token() {
        let mut store = HashsetBannedTokenStore::default();
        let token = "test_token".to_owned();
        store.banned_token_store.insert(token.clone());

        let result = store.token_exists(&token).await;

        assert!(result.unwrap());
    }
}