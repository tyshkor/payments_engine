use std::error::Error;
use std::collections::BTreeMap;

use rayon::prelude::*;

use crate::entities::{
    account::Account,
    account_enriched::AccountEnriched,
    transaction::Transaction,
};

const VEC_SIZE: usize = u16::MAX as usize;

pub fn parse_csv(reader: &String) -> Result<Vec<Option<AccountEnriched>>, Box<dyn Error>> {
    
    let mut account_enriched_vec: Vec<Option<AccountEnriched>> = Vec::with_capacity(VEC_SIZE);
    account_enriched_vec.resize_with(VEC_SIZE, || None);

    let rdr_result = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(reader);

    if let Ok(mut rdr) = rdr_result {
        for result in rdr.deserialize().skip(1) { // skip the headers

            let record: Transaction = result?;
            let client_id = record.account_id as usize;
    
            if let Some(loc_map) = &mut account_enriched_vec[client_id] {
                record.insert_to_map(&mut loc_map.transactions);
            } else {
                let mut new_map = BTreeMap::new();
                record.insert_to_map(&mut new_map);
                
                account_enriched_vec[client_id] = Some(AccountEnriched {
                    account: Account::new(client_id as u16),
                    transactions: new_map,
                });
            }
        }
    
        Ok(account_enriched_vec)
    } else {
       Err(Box::new(rdr_result.err().unwrap())) // we know that we have an error here, so unwrap won't panic
    }

    
}

pub fn par_handler(mut account_enriched_vec: Vec<Option<AccountEnriched>>) -> Vec<AccountEnriched>{
    account_enriched_vec.par_iter_mut().flat_map(|opt| opt).for_each(|account_enriched| {
            account_enriched.handle();    
    });

    account_enriched_vec.into_iter().flat_map(|opt| opt).collect()
}