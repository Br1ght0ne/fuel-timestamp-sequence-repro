use fuels::{prelude::*, programs::contract::LoadConfiguration};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/debug/timestamp-repro-abi.json"
));

pub async fn init_wallets() -> Vec<WalletUnlocked> {
    let wallets_config = WalletsConfig::new(Some(5), Some(1), Some(1_000_000_000));

    let block_time = 1u64; // 1 Second block time
    let provider_config = NodeConfig {
        block_production: Trigger::Interval {
            block_time: std::time::Duration::from_secs(block_time),
        },
        ..NodeConfig::default()
    };

    launch_custom_provider_and_get_wallets(wallets_config, Some(provider_config), None)
        .await
        .unwrap()
}

#[tokio::test]
async fn repro() {
    let wallets = init_wallets().await;

    let admin = &wallets[0];
    let alice = &wallets[1];

    let id = Contract::load_from(
        "./out/debug/timestamp-repro.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(admin, TxPolicies::default())
    .await
    .unwrap();
    let contract_admin = MyContract::new(id.clone(), admin.clone());
    let contract_alice = MyContract::new(id.clone(), alice.clone());
    // Produce blocks
    admin
        .try_provider()
        .unwrap()
        .produce_blocks(5, None)
        .await
        .unwrap();
    // Contract call 1
    let time1 = contract_admin
        .methods()
        .get_timestamp()
        .call()
        .await
        .unwrap()
        .value;
    dbg!(time1);
    // Produce blocks
    alice
        .try_provider()
        .unwrap()
        .produce_blocks(10, None)
        .await
        .unwrap();
    // Contract call 2 (std::block:timestamp returns lower value than for previous call)
    let time2 = contract_alice
        .methods()
        .get_timestamp()
        .call()
        .await
        .unwrap()
        .value;
    dbg!(time2);
    assert!(time1 < time2, "producing blocks should advance timestamp");
}
