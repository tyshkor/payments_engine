mod entities;
mod handlers;
mod utils;

use std::{process, env};

use handlers::*;
use utils::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    let arg = &args[1];

    match parse_csv(arg) {
        Err(err) => {

            println!("error running csv parser: {}", err);
            process::exit(1);
        },
        Ok(res_vec) => {

            let res = par_handler(res_vec);

            if let Err(err) = output(res) {
                println!("error outputing result: {}", err);
                process::exit(1);
            }   
        }
    }
}

#[cfg(test)]
mod tests {

    use std::process;
    use std::collections::BTreeMap;

    use crate::handlers::*;

    use crate::entities::{
        account_enriched::AccountEnriched,
        account::Account,
        transaction::Transaction,
        transaction_type::TransactionType::*
    };
    

    #[test]
    fn basic_test() {

        match parse_csv(&"test_data/basic.csv".to_string()) {
            Err(err) => {
        
                println!("error running csv parser: {}", err);
                process::exit(1);
            },
            Ok(res_vec) => {
        
                let res = par_handler(res_vec);
    
                assert_eq!(res, vec![
                    AccountEnriched { 
                        account: Account { id: 1, available: 1.5, held: 0.0, total: 1.5, locked: false }, 
                        transactions: BTreeMap::from(
                            [
                                (1, Transaction { transaction_type: Deposit, account_id: 1, id: 1, amount: Some(1.0) }), 
                                (3, Transaction { transaction_type: Deposit, account_id: 1, id: 3, amount: Some(2.0) }), 
                                (4, Transaction { transaction_type: Withdrawal, account_id: 1, id: 4, amount: Some(1.5) }),
                            ]
                        )
                    }, 
                    AccountEnriched { 
                        account: Account { id: 2, available: -1.0, held: 0.0, total: -1.0, locked: false }, 
                        transactions: BTreeMap::from(
                            [
                                (2, Transaction { transaction_type: Deposit, account_id: 2, id: 2, amount: Some(2.0) }), 
                                (5, Transaction { transaction_type: Withdrawal, account_id: 2, id: 5, amount: Some(3.0) })
                            ]
                        )
                    }
                ]
                );  
            }
        }
    }

    #[test]
    fn dispute_test() {

        match parse_csv(&"test_data/dispute.csv".to_string()) {
            Err(err) => {
        
                println!("error running csv parser: {}", err);
                process::exit(1);
            },
            Ok(res_vec) => {
        
                let res = par_handler(res_vec);
    
                assert_eq!(res, vec![
                    AccountEnriched { 
                        account: Account { id: 1, available: 0.0, held: 1.0, total: 1.0, locked: false }, 
                        transactions: BTreeMap::from(
                            [
                                (1, Transaction { transaction_type: Deposit, account_id: 1, id: 1, amount: Some(1.0) }), 
                                (2, Transaction { transaction_type: Dispute, account_id: 1, id: 1, amount: None }), 
                            ]
                        )
                    }
                ]
                );  
            }
        }
    }

    
    #[test]
    fn dispute_resolve_test() {

        match parse_csv(&"test_data/dispute_resolve.csv".to_string()) {
            Err(err) => {
        
                println!("error running csv parser: {}", err);
                process::exit(1);
            },
            Ok(res_vec) => {
        
                let res = par_handler(res_vec);
    
                assert_eq!(res, vec![
                    AccountEnriched { 
                        account: Account { id: 1, available: 1.0, held: 0.0, total: 1.0, locked: false }, 
                        transactions: BTreeMap::from(
                            [
                                (1, Transaction { transaction_type: Deposit, account_id: 1, id: 1, amount: Some(1.0) }), 
                                (2, Transaction { transaction_type: Dispute, account_id: 1, id: 1, amount: None }), 
                                (3, Transaction { transaction_type: Resolve, account_id: 1, id: 1, amount: None }),
                            ]
                        )
                    }
                ]
                );  
            }
        }
    }

    #[test]
    fn chargeback_test() {

        match parse_csv(&"test_data/chargeback.csv".to_string()) {
            Err(err) => {
        
                println!("error running csv parser: {}", err);
                process::exit(1);
            },
            Ok(res_vec) => {
        
                let res = par_handler(res_vec);
    
                assert_eq!(res, vec![
                    AccountEnriched { 
                        account: Account { id: 1, available: 0.0, held: 0.0, total: 0.0, locked: true }, 
                        transactions: BTreeMap::from(
                            [
                                (1, Transaction { transaction_type: Deposit, account_id: 1, id: 1, amount: Some(1.0) }), 
                                (2, Transaction { transaction_type: Dispute, account_id: 1, id: 1, amount: None }), 
                                (3, Transaction { transaction_type: Chargeback, account_id: 1, id: 1, amount: None }),
                            ]
                        )
                    }
                ]
                );  
            }
        }
    }

    #[test]
    fn ignore_test() {

        match parse_csv(&"test_data/ignore.csv".to_string()) {
            Err(err) => {
        
                println!("error running csv parser: {}", err);
                process::exit(1);
            },
            Ok(res_vec) => {
        
                let res = par_handler(res_vec);
    
                assert_eq!(res, vec![
                    AccountEnriched { 
                        account: Account { id: 1, available: 0.0, held: 0.0, total: 0.0, locked: false }, 
                        transactions: BTreeMap::from(
                            [
                                (1, Transaction { transaction_type: Chargeback, account_id: 1, id: 1, amount: None }),
                            ]
                        )
                    }
                ]
                );  
            }
        }
    }
}



