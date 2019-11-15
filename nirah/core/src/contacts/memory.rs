use async_trait::async_trait;
use nirah_uri::Uri;

use crate::prelude::*;

#[derive(Debug)]
pub struct InMemoryContactsProvider(Vec<Contact>);

impl InMemoryContactsProvider {

    pub fn new() -> InMemoryContactsProvider {
        InMemoryContactsProvider(Vec::new())
    }
}

impl Provider for InMemoryContactsProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "InMemoryContactsProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}


#[async_trait]
impl ContactsProvider for InMemoryContactsProvider {

   async fn create_contact(&mut self, new_contact: NewContact) -> NirahResult<u32> {
       let id = self.0.len() as u32;
       self.0.push(Contact {
           id,
           display_name: new_contact.display_name,
           uri: new_contact.uri
       });
       Ok(id)
   }

   async fn edit_contact(&mut self, new_contact: Contact) -> NirahResult<()> {
       for contact in self.0.iter_mut() {
           if contact.id == new_contact.id {
               *contact = new_contact;
               return Ok(())
           }
       }
       Err(NirahError::InvalidContactId(new_contact.id))
   }

   async fn get_contact(&mut self, contact_id: u32) -> NirahResult<Option<Contact>> {
       for contact in self.0.iter() {
           if contact.id == contact_id {
               return Ok(Some(contact.clone()));
           }
       }
       Ok(None)
   }

   async fn get_contact_from_uri(&mut self, contact_uri: Uri) -> NirahResult<Option<Contact>> {
       let mut index = None;
       for (dex, contact) in self.0.iter().enumerate() {
           if contact_uri == contact.uri {
               index = Some(dex);
           }
       }
       Ok(index.map(|item| self.0.get(item).unwrap()).map(|item|item.clone()))
   }

   async fn remove_contact(&mut self, contact: u32) -> NirahResult<()> {
       self.0.remove(contact as usize);
       Ok(())
   }

   async fn all_contacts(&self) -> Vec<Contact> {
       self.0.clone()
   }
}
