use csv::Reader;

use crate::{err::Result, handle_tx::ClientAccounts, models::Record};

mod err;
mod handle_tx;
mod models;

fn main() -> crate::err::Result<()> {
    println!("Hello, incoming csv!");
    let (tx, rx) = std::sync::mpsc::channel::<Record>();
    let r = std::env::args().nth(1);
    let thread_deserialize = std::thread::spawn(move || -> Result<()> {
        if let Some(path) = r {
            let mut rdr = Reader::from_path(path)?;
            let iter = rdr.deserialize::<Record>();
            for (mut counter, record) in iter.enumerate() {
                counter += 1;
                if let Ok(record) = record {
                    tx.send(record)?;
                } else {
                    dbg!("Incorrect value type found at row: ", counter);
                }
            }
        }
        Ok(())
    });

    let mut account_manager = ClientAccounts::default();

    let thread_transactions = std::thread::spawn(move || -> Result<()> {
        while let Ok(record) = rx.recv() {
            account_manager.trans(record);
        }
        let mut wtr = csv::Writer::from_writer(std::io::stdout());

        wtr.write_record(["client", "available", "held", "total", "locked"])?;

        for (client_id, ele) in account_manager.client_account {
            wtr.write_record([
                client_id.to_string(),
                (ele.total - ele.held).to_string(),
                (ele.total + ele.available).to_string(),
                (ele.available + ele.held).to_string(),
                ele.locked.to_string(),
            ])?;
        }
        Ok(())
    });

    thread_deserialize.join().unwrap();
    thread_transactions.join().unwrap();

    Ok(())
}
