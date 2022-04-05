use crate::entities::{
    account::Account,
    account_enriched::AccountEnriched,
    transaction::Transaction,
};

use std::error::Error;
use std::collections::BTreeMap;
use std::io::Read;

use rayon::prelude::*;

const VEC_SIZE: usize = u16::MAX as usize;

pub fn parse_csv(reader: &String) -> Result<Vec<Option<AccountEnriched>>, Box<dyn Error>> {
    
    let mut res_vec: Vec<Option<AccountEnriched>> = Vec::with_capacity(VEC_SIZE);
    res_vec.resize_with(VEC_SIZE, || None);

    let mut rdr_result = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(reader);

    if let Ok(mut rdr) = rdr_result {
        for result in rdr.deserialize().skip(1) { // skip the headers

            let record: Transaction = result?;
            let client_id = record.account_id as usize;
    
            if let Some(loc_map) = &mut res_vec[client_id] {
                record.insert_to_map(&mut loc_map.transactions);
            } else {
                let mut new_map = BTreeMap::new();
                record.insert_to_map(&mut new_map);
                
                res_vec[client_id] = Some(AccountEnriched {
                    account: Account::new(client_id as u16),
                    transactions: new_map,
                });
            }
        }
    
        Ok(res_vec)
    } else {
       Err(Box::new(rdr_result.err().unwrap()))
    }

    
}

pub fn par_handler(mut res_vec: Vec<Option<AccountEnriched>>) -> Vec<AccountEnriched>{
    res_vec.par_iter_mut().flat_map(|opt| opt).for_each(|item| {
            item.handle();    
    });

    res_vec.into_iter().flat_map(|opt| opt).collect()
}