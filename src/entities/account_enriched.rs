use std::collections::{BTreeMap, HashSet};

use serde::{Serialize, Deserialize};

use crate::entities::{
    account::Account,
    transaction::Transaction,
    transaction_type::TransactionType
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AccountEnriched {
    pub account: Account,
    pub transactions: BTreeMap<u32, Transaction>
}

impl AccountEnriched {

    pub fn handle(&mut self) {

        let transaction_map = &self.transactions;

        let mut disputed_transactions_id_set: HashSet<u32> = HashSet::new();

        for (_, Transaction { transaction_type, account_id, id, amount }) in transaction_map.iter() {

            match transaction_type {
                TransactionType::Deposit => {
                    if let Some(amount_value) = amount {
                        self.account.total += *amount_value;
                        self.account.available += *amount_value;
                    }
                },
                TransactionType::Withdrawal => {
                    if let Some(amount_value) = amount {
                        self.account.total -= *amount_value;
                        self.account.available -= *amount_value;
                    }
                },
                TransactionType::Dispute => {

                    if let Some(disputed_transaction) = transaction_map.get(id) {
                        disputed_transactions_id_set.insert(*id);

                        if let Some(disputed_amount) = disputed_transaction.amount {
                            self.account.held += disputed_amount;
                            self.account.available -= disputed_amount;
                        } else {
                            // never reach this branch as we put only valid transations' ids to the disputed_transactions_id_set HashSet
                        }
                        
                    } else {
                        // ignore the error, as stated in the task
                    }
                },
                TransactionType::Resolve => {

                    if let Some(disputed_transaction) = transaction_map.get(id) {
                        disputed_transactions_id_set.remove(id);

                        if let Some(disputed_amount) = disputed_transaction.amount {
                            self.account.held -= disputed_amount;
                            self.account.available += disputed_amount;
                        } else {
                            // never reach this branch as we put only valid transations' ids to the disputed_transactions_id_set HashSet
                        }
                        
                    } else {
                        // ignore the error, as stated in the task
                    }
                },
                TransactionType::Chargeback => {
                    if let Some(chargeback_transaction) = transaction_map.get(id) {
                        if disputed_transactions_id_set.contains(&chargeback_transaction.id)  {
                            if let Some(chargeback_amount) = chargeback_transaction.amount {
                                self.account.total -= chargeback_amount;
                                self.account.held -= chargeback_amount;
                                self.account.locked = true;
                            }
                        } else {
                            // never reach this branch as we put only valid transations' ids to the disputed_transactions_id_set HashSet
                        }
                    } else {
                        // ignore the error, as stated in the task
                    }
                },
            }
        }
    
    }
}
