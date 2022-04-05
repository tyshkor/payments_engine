use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};

use crate::entities::transaction_type::TransactionType;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub account_id: u16,
    pub id: u32,
    pub amount: Option<f32>,  // for Dispute, Resolve and Chargeback types of transactions there is no amount
}

impl Transaction {
    pub fn insert_to_map(self, map: &mut BTreeMap<u32, Transaction>) {
        match self.transaction_type {
            TransactionType::Deposit | TransactionType::Withdrawal => {
                map.insert(self.id, self);
            },
            _ => {
                map.insert((map.len() + 1) as u32, self); // unique, local to the account, id
            }
        }
    }
}
