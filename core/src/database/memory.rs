use async_trait::async_trait;

use crate::prelude::*;

use std::collections::HashMap;

pub struct InMemoryDatabaseProvider(HashMap<(u32, u32), Vec<TransactionEvent>>);

impl InMemoryDatabaseProvider {

    pub fn new() -> InMemoryDatabaseProvider {
        InMemoryDatabaseProvider(HashMap::new())
    }
}

impl Provider for InMemoryDatabaseProvider {

    fn nirah_provider_identifier(&self) -> &'static str {
        "InMemoryDatabaseProvider"
    }

    fn nirah_provider_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}


#[async_trait]
impl DatabaseProvider for InMemoryDatabaseProvider {
    async fn contact_transactions(&mut self, account: u32, contact: u32) -> Option<&Vec<TransactionEvent>> {
        if let Some(contact) = self.0.get(&(account, contact)) {
            Some(contact)
        } else {
            None
        }
    }

    async fn account_transactions(&mut self, account: u32) -> NirahResult<Vec<(u32, Option<TransactionEvent>)>> {
        let mut output = vec![];
        for((acc, contact), transactions) in self.0.iter() {
            if &account == acc {
                for (contact, _) in &output {
                    if contact == contact {
                        continue;
                    }
                }
               output.push((contact.clone(), transactions.iter().last().map(|item|item.clone())));
            }
        }
        Ok(output)
    }

    async fn log(&mut self, account: u32, contact: u32, log: NewTransactionEvent) -> NirahResult<u32> {
        let new_id = self.0.values().len() as u32;
        if self.0.contains_key(&(account, contact)) {
            let contact_data = self.0.get_mut(&(account, contact)).unwrap();
            contact_data.push(log.save(new_id));
            Ok(new_id)
        } else {
            self.0.insert((account, contact), vec![log.save(0)]);
            Ok(0)
        }
    }

    async fn get_log(&self, account: u32, contact: u32, log: u32) -> NirahResult<Option<&TransactionEvent>> {
        if let Some(data) = self.0.get(&(account, contact)) {
            let mut location = None;
            for (id, item) in data.iter().enumerate() {
                if item.id == log {
                    location = Some(id);
                }
            }
            if let Some(log) = location.map(|item| data.get(item)) {
                Ok(log)
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn remove_log(&mut self, account: u32, contact: u32, log: u32) -> NirahResult<()> {
        if let Some(data) = self.0.get_mut(&(account, contact)) {
            let mut location = None;
            for (id, item) in data.iter().enumerate() {
                if item.id == log {
                    location = Some(id);
                }
            }
            if let Some(location) = location {
                data.remove(location);
            }
        }
        Ok(())
    }
}
