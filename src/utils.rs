use std::{error::Error, io};

use crate::entities::account_enriched::AccountEnriched;

pub fn output(account_vec: Vec<AccountEnriched>) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for item in account_vec {
        wtr.serialize(item.account)?;
    }

    wtr.flush()?;

    Ok(())
}