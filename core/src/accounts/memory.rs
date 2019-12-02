use async_trait::async_trait;

use crate::prelude::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct InMemoryAccountsProvider(Vec<Account>);

impl InMemoryAccountsProvider {

    pub fn new() -> InMemoryAccountsProvider {
        InMemoryAccountsProvider(vec![
            Account {
                id: 0,
                ty: AccountType::Sip,
                username: "20".into(),
                password: "program".into(),
                host: "192.168.1.133".into(),
                activate: false,
                vars: HashMap::new()
            }
        ])
    }
}

impl Provider for InMemoryAccountsProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "InMemoryAccountsProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}


#[async_trait]
impl AccountsProvider for InMemoryAccountsProvider {

   async fn create_account(&mut self, new: NewAccount) -> NirahResult<u32> {
       let id = self.0.len() as u32;
       self.0.push(Account {
           id,
           ty: new.ty,
           username: new.username,
           password: new.password,
           host: new.host,
           activate: new.activate,
           vars: new.vars
       });
       Ok(id)
   }

   async fn edit_account(&mut self, new_acc: Account) -> NirahResult<()> {
       for acc in self.0.iter_mut() {
           if acc.id == new_acc.id {
               *acc = new_acc;
               return Ok(())
           }
       }
       Err(NirahError::InvalidAccountId(new_acc.id))
   }

   async fn get_account(&mut self, acc: u32) -> NirahResult<Option<Account>> {
       for account in self.0.iter() {
           if account.id == acc {
               return Ok(Some(account.clone()));
           }
       }
       Err(NirahError::InvalidAccountId(acc))
   }

   async fn remove_account(&mut self, acc: u32) -> NirahResult<()> {
       self.0.remove(acc as usize);
       Ok(())
   }

   async fn all_accounts(&self) -> Vec<Account> {
       self.0.clone()
   }
}
