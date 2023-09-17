use serde::{Serialize, Deserialize};
use alloy_primitives::{Address, U256, StorageKey, TxHash, StorageValue};

use revm::{
    primitives::{AccountInfo, TxEnv, B160},
    InMemoryDB, EVM,
};

use std::{io::Read, net::TcpListener, str::FromStr};

mod server;
use server::get_key_and_cert;

// This payload should be generalized to include all the pre-state for each
// simulation.
#[derive(Serialize, Deserialize)]
struct Payload {
    sender: Address,
    amount: U256,
}


#[derive(Serialize, Deserialize, Debug)]
struct AccessListData {
    address: Address,
    storage_keys: Vec<StorageKey>,
    storage_values: Vec<StorageValue>,
}


// This payload should be generalized to include all the pre-state for each
// simulation.
#[derive(Serialize, Deserialize)]
struct Payload2 {
    state_root: U256,
    sender_address: Address,
    access_list: Vec<AccessListData>,
    tx_hash: TxHash,
}

/*#[post("/payload", data = "<payload>")]
fn post_payload(payload: Json<Payload>) -> Json<Payload> {
    let received_payload: Payload = serde_json::from_slice(payload);
    simulate(received_payload)
}*/

fn main() -> eyre::Result<()>{

    let (mut key, mut cert) = get_key_and_cert();
    // dbg!(&cert);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = vec![];
        let _num_bytes = stream.read_to_end(&mut buf)?;
        let data: Payload2 = serde_json::from_slice(&buf)?;
        simulate_storage_proofs_validation(data)?;

        // TODO: Re-enable this,
        // let _ = serve(stream, &mut key, &mut cert).unwrap();
    }
    Ok(())
    
    /*rocket::ignite()
        .mount("/", routes![post_payload])
        .launch();*/
    
}

fn simulate(payload: Payload) -> eyre::Result<()> {
    let mut db = InMemoryDB::default();
    let receiver = payload.sender;
    let value = payload.amount;

    let balance = U256::from(111);
    // this is a random address
    let address = "0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97".parse()?;
    let info = AccountInfo {
        balance,
        ..Default::default()
    };

    // Populate the DB pre-state,
    // TODO: Make this data witnessed via merkle patricia proofs.
    db.insert_account_info(address, info);
    // For storage insertions:
    // db.insert_account_storage(address, slot, value)

    // Setup the EVM with the configured DB
    // The EVM will ONLY be able to access the witnessed state, and
    // any simulation that tries to use state outside of the provided data
    // will fail.
    let mut evm = EVM::new();
    evm.database(db);

    evm.env.tx = TxEnv {
        caller: address,
        transact_to: revm::primitives::TransactTo::Call(B160::from(receiver.0 .0)),
        value,
        ..Default::default()
    };

    let result = evm.transact_ref()?;

    assert_eq!(
        result.state.get(&address).unwrap().info.balance,
        U256::from(69)
    );

    dbg!(&result);

    Ok(())
}


fn simulate_storage_proofs_validation(payload: Payload2) -> eyre::Result<()> {
    // Storage and Access List Proofs are parsed inside the payload as a vec<address, storageKeys>.

    // Retrieve the Block's State Root is also parsed inside the payload (currently coming from the untrusted side leaving the it to the validator to check if the state root matches or not).

    // Extract the above and rebuild the MPT to Verify Access List.

    //Verify Storage Proofs for Access List Keys.
    // a. Obtain state root.
    // b. Loop through access list keys.
    // c. Verify storage proofs.

    // Step 5: Complete Verification.

    Ok(())
}