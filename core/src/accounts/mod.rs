use async_trait::async_trait;

use crate::core::Provider;
use crate::core::NirahResult;

mod account_type;
pub use self::account_type::AccountType;

mod account;
pub use self::account::NewAccount;
pub use self::account::Account;

mod memory;
pub use self::memory::InMemoryAccountsProvider;

#[async_trait]
pub trait AccountsProvider: Provider {

   async fn create_account(&mut self, new: NewAccount) -> NirahResult<u32>;

   async fn edit_account(&mut self, acc: Account) -> NirahResult<()>;

   async fn get_account(&mut self, acc: u32) -> NirahResult<Option<Account>>;

   async fn remove_account(&mut self, acc: u32) -> NirahResult<()>;

   async fn all_accounts(&self) -> Vec<Account>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
