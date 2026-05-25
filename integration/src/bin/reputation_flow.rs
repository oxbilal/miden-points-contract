use integration::helpers::{
    build_project_in_dir, counter_storage_slot, create_account_from_package,
    create_basic_wallet_account, setup_client, AccountCreationConfig, ClientSetup,
    COUNTER_STORAGE_KEY,
};

use anyhow::{Context, Result};
use miden_client::{account::component::InitStorageData, transaction::TransactionRequestBuilder};
use miden_standards::testing::note::NoteBuilder;
use std::{path::Path, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let ClientSetup { mut client, keystore } = setup_client().await?;

    let reputation_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/reputation-account"), true)
            .context("Failed to build reputation account contract")?,
    );

    let note_package = Arc::new(
        build_project_in_dir(Path::new("../contracts/reputation-note"), true)
            .context("Failed to build reputation note")?,
    );

    let storage_slot = counter_storage_slot()?;
    let mut init_storage_data = InitStorageData::default();

    init_storage_data
        .insert_map_entry(storage_slot, COUNTER_STORAGE_KEY, 0_u64)
        .context("Failed to seed reputation storage")?;

    let reputation_cfg = AccountCreationConfig {
        init_storage_data,
        ..Default::default()
    };

    let reputation_account =
        create_account_from_package(&mut client, reputation_package.clone(), reputation_cfg)
            .await
            .context("Failed to create reputation account")?;

    println!("Reputation account ID: {:?}", reputation_account.id().to_hex());

    let sender_account =
        create_basic_wallet_account(&mut client, keystore.clone(), AccountCreationConfig::default())
            .await
            .context("Failed to create sender wallet account")?;

    println!("Sender account ID: {:?}", sender_account.id().to_hex());

    let reputation_note = NoteBuilder::new(sender_account.id(), client.rng())
        .package((*note_package).clone())
        .tag(0)
        .build()
        .context("Failed to create reputation note")?;

    println!("Reputation note hash: {:?}", reputation_note.id().to_hex());

    let publish_request = TransactionRequestBuilder::new()
        .own_output_notes(vec![reputation_note.clone()])
        .build()
        .context("Failed to build note publish transaction")?;

    let publish_tx_id = client
        .submit_new_transaction(sender_account.id(), publish_request)
        .await
        .context("Failed to publish reputation note")?;

    println!("Note publish transaction ID: {:?}", publish_tx_id.to_hex());

    let consume_request = TransactionRequestBuilder::new()
        .input_notes([(reputation_note.clone(), None)])
        .build()
        .context("Failed to build consume note transaction")?;

    let consume_tx_id = client
        .submit_new_transaction(reputation_account.id(), consume_request)
        .await
        .context("Failed to consume reputation note")?;

    println!("Consume transaction ID: {:?}", consume_tx_id.to_hex());
    println!("Reputation increased by 50 ✅");

    Ok(())
}
