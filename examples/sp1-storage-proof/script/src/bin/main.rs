use alloy::eips::BlockId;
use alloy::network::Ethereum;
use alloy::primitives::{address, b256, Address, B256}; // U256
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::ClientBuilder;
use alloy::rpc::types::EIP1186AccountProofResponse;
use alloy::transports::layers::RetryBackoffLayer;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin, HashableKey};
use storage_proof_lib::{StorageSlotProof}; // get_slot_index

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("storage-proof-program");

// Contract address whose storage is being read
pub const CONTRACT_ADDRESS: Address = address!("A1a3A3Ab81168ECfc0F7F39489754B877B6fFe85");  
pub const CONTRACT_MAPPING_SLOT: B256 = b256!("000000000000000000000000000000000000000000000000000000000000000e");
pub const MAPPING_VALUE: B256 = b256!("0000000000000000000000000000000000000000000000000000000000000001");
pub const BLOCK_NUMBER: u64 = 190434;  // https://explorer-jam-ccw030wxbz.t.conduit.xyz/block/190434
pub const BLOCK_STATE_ROOT: B256 = b256!("ecdc9dde35836e1f0334fe763dfef9c07931f98fa67cb6213be543f0ee747003");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,
    #[clap(long)]
    prove: bool,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the client
    dotenv::dotenv().ok();
    // export ETHEREUM_RPC_URL="https://sepolia.infura.io/v3/c209b31b7c3e41d8a9dacce9cf5dbbf4"
    // std::env::var("ETHEREUM_RPC_URL")
    let rpc_url: String = "https://rpc-jam-ccw030wxbz.t.conduit.xyz/Pwe4skpfPaM8HSTPwHDhXzoJoKqdpjfRQ".to_string();
    //        .expect("ETHEREUM_RPC_URL not set");
    let client = ClientBuilder::default()
        .layer(RetryBackoffLayer::new(100, 50, 300))
        .http(rpc_url.parse()
        .expect("Failed to parse RPC URL"));
    // Setup provider
    #[cfg(target_arch = "wasm32")]
    let client = ClientBuilder::default().http(rpc_url.parse()
        .expect("Failed to parse RPC URL"));
    let provider = ProviderBuilder::new().network::<Ethereum>().on_client(client);

    // Fetch the storage proof
    let slot: B256 = b256!("000000000000000000000000000000000000000000000000000000000000000e"); // get_slot_index(&CONTRACT_MAPPING_SLOT, 126);
    let account_proof_response: EIP1186AccountProofResponse = provider
        .get_proof(CONTRACT_ADDRESS, vec![slot])
        .block_id(BlockId::from(BLOCK_NUMBER))
        .await
        .unwrap();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write::<B256>(&BLOCK_STATE_ROOT);
    stdin.write::<Address>(&CONTRACT_ADDRESS);
    stdin.write::<B256>(&slot);
    stdin.write::<B256>(&MAPPING_VALUE);
    stdin.write::<StorageSlotProof>(&StorageSlotProof::from_account_proof_response(account_proof_response));

    if args.execute {
        // Execute the program
        let (_, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);
    println!("vk: {:?}", vk.bytes32());

        // Generate the proof
        println!("Generating proof...");
        let start_time = std::time::Instant::now();
        let proof = client
            .prove(&pk, &stdin)
	    .groth16()
            .run()
            .expect("failed to generate proof");
        let proving_time = start_time.elapsed();
        println!("Successfully generated proof! Proving time: {:?}", proving_time);

    // Get the public values as bytes.
    let public_values = proof.public_values.as_slice();
    println!("public values: 0x{}", hex::encode(public_values));

    // Get the proof as bytes.
    let solidity_proof = proof.bytes();
    println!("proof: 0x{}", hex::encode(solidity_proof));
        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    // Save the proof.
    proof.save("storageproof.bin").expect("saving proof failed");
    }
}

