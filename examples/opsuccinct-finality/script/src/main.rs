use sp1_sdk::{utils, HashableKey, ProverClient, SP1Stdin, include_elf};

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("opsuccinct-finality-program");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '123' to it.

    let stdin = SP1Stdin::new();
    // let n = 123u32;
    // stdin.write(&n);

    // Set up the pk and vk.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    println!("vk: {:?}", vk.bytes32());

    // Generate the Groth16 proof.
    let proof = client.prove(&pk, stdin).groth16().run().unwrap();
    println!("generated proof");

    // Get the public values as bytes.
    let public_values = proof.public_values.as_slice();
    println!("public values: 0x{}", hex::encode(public_values));

    // Get the proof as bytes.
    let solidity_proof = proof.bytes();
    println!("proof: 0x{}", hex::encode(solidity_proof));

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Save the proof.
    proof.save("opsuccinct-2groth16.bin").expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}