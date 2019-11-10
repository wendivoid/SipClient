use async_trait::async_trait;
use crate::core::Provider;
use crate::core::NirahResult;

mod memory;
pub use self::memory::InMemoryContactsProvider;

mod models;
pub use self::models::Contact;
pub use self::models::NewContact;

#[async_trait]
pub trait ContactsProvider: Provider {
    async fn all_contacts(&self) -> Vec<Contact>;

    async fn create_contact(&mut self, _: NewContact) -> NirahResult<u32>;

    async fn edit_contact(&mut self, _: Contact) -> NirahResult<()>;

    async fn get_contact(&mut self, _: u32) -> NirahResult<Option<Contact>>;

    async fn get_contact_from_uri(&mut self, _: nirah_uri::Uri) -> NirahResult<Option<Contact>>;

    async fn remove_contact(&mut self, _: u32) -> NirahResult<()>;
}
