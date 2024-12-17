//! A program that verifies 3 Groth16 proofs and generates another groth16 proof.

#![no_main]
sp1_zkvm::entrypoint!(main);
use sp1_verifier::Groth16Verifier;
use alloy_primitives::{address, b256, B256, Address};


struct L2ProofOfL1InclusionData {
    state_root_l1: B256,
    l2PostRoot: B256,
    slot: u64,
    contract_address: Address,
    l2BlockNumber: u64,
    timestamp: u64,
    // OPTIONAL: l1blocknumber, contractaddress, timestamp
}

struct L2ExecData {
    l1Head: B256,
    l2PreRoot: B256,
    l2PostRoot: B256,
    l2BlockNumber: u64,
    rollupConfigHash: B256,
    multiBlockVKey: B256,
    // l2outputIndex: u64,
    // contract_address: Address,
}

struct HeliosData {
    state_root: B256,
    // TODO: executionStateRoot, newHeader, nextSyncCommitteeHash, newHead, prevHeader, syncCommitteeHash, vk_helios
}

fn get_l2exec() -> (Vec<u8>, Vec<u8>, String, L2ExecData) {
    // l2exec proof data https://sepolia.etherscan.io/tx/0xb979469cfdc348ae39044fe11e501928d93ae90c416b2ec7df7a70f39acac497
    let proof: Vec<u8> = vec![
    0x09, 0x06, 0x90, 0x90, 0x08, 0x39, 0x32, 0x70, 0x1d, 0x28, 0xaf, 0x36, 0x91, 0x22, 0xf1, 0xb3,
    0xd0, 0xd5, 0xce, 0x3a, 0x36, 0x06, 0xa9, 0x84, 0x5f, 0xc7, 0x3c, 0x1d, 0x6c, 0x60, 0x75, 0x12,
    0x1b, 0xd2, 0x00, 0xed, 0x1f, 0xb1, 0xa0, 0x42, 0xe4, 0x4d, 0x82, 0x63, 0xb8, 0x51, 0x8f, 0xee,
    0xad, 0x82, 0x47, 0x18, 0x5d, 0xeb, 0xe8, 0xa9, 0xf5, 0x45, 0xa0, 0x8b, 0x57, 0xef, 0xcd, 0x6f,
    0xdc, 0x30, 0xc1, 0xe1, 0x15, 0x81, 0xb2, 0x3d, 0xbd, 0x36, 0x48, 0xc4, 0xeb, 0xdc, 0x64, 0xef,
    0x48, 0x61, 0x34, 0xaa, 0xa8, 0x44, 0xde, 0x55, 0x7c, 0x12, 0x16, 0x84, 0xc4, 0xfc, 0xd6, 0x9e,
    0xbb, 0x80, 0x47, 0x34, 0x03, 0x99, 0x1a, 0x96, 0x7f, 0x59, 0x44, 0xd4, 0x23, 0x4c, 0x67, 0x3d,
    0xc3, 0xdd, 0x8c, 0x4a, 0x89, 0x5a, 0x1c, 0xb5, 0xa5, 0x60, 0x05, 0x17, 0xc4, 0xbd, 0x13, 0xa2,
    0x17, 0xfe, 0x91, 0x1a, 0x2f, 0xed, 0x0f, 0x6a, 0x02, 0xd1, 0x29, 0x55, 0xbe, 0x3c, 0xc7, 0x24,
    0xfe, 0x92, 0xcc, 0x2b, 0xc1, 0x9b, 0x3a, 0xb0, 0xa8, 0xa1, 0xff, 0x3b, 0x37, 0x1d, 0xd1, 0x92,
    0x4c, 0x77, 0x14, 0x00, 0x18, 0x0d, 0x11, 0x1a, 0x2b, 0x1a, 0xa0, 0x03, 0xa3, 0xe2, 0x60, 0x12,
    0x85, 0x93, 0x19, 0x19, 0xca, 0x35, 0xf7, 0xe3, 0x01, 0xa4, 0x21, 0xb1, 0x14, 0x6d, 0xb4, 0x06,
    0x42, 0x2c, 0xe2, 0xd7, 0x00, 0x98, 0x27, 0x68, 0x27, 0xcf, 0x64, 0x2f, 0xea, 0xbf, 0xf3, 0xf9,
    0x63, 0xd3, 0xfc, 0x82, 0xa2, 0x2a, 0xe9, 0x64, 0x3a, 0x68, 0xdb, 0x41, 0x6f, 0x27, 0xef, 0xb9,
    0xa9, 0xc2, 0xc6, 0xff, 0x15, 0x5d, 0x3d, 0x33, 0x0a, 0xe4, 0x62, 0x34, 0x14, 0x55, 0xb8, 0x36,
    0x7b, 0x0c, 0x18, 0x33, 0x4e, 0x4a, 0x1c, 0x99, 0x73, 0xc9, 0x89, 0x93, 0x03, 0xb0, 0x2e, 0x7b,
    0xdc, 0x73, 0x7c, 0xc2
    ];

    // Value at l2Outputs[126]: 0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a
    // Value at l2Outputs[127]: 0x000000000000000000000000004c06d800000000000000000000000067411ba8
    let public_values: Vec<u8> = vec![
     0x94, 0xdf, 0x86, 0xf9,    0xd1, 0xf6, 0x1f, 0x93,    0xf8, 0xe3, 0x2a, 0x46,    0xb7, 0x47, 0xe8, 0x20,
     0x20, 0x24, 0x24, 0x1d,    0xc2, 0xe8, 0xe8, 0xf7,    0xf4, 0x02, 0x32, 0x6a,    0x26, 0x86, 0xed, 0x1a,
     0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,    0x00, 0x4c, 0x06, 0xd8,
     0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,    0x00, 0x00, 0x00, 0x00,    0x67, 0x41, 0x1b, 0xa8,
    ];
    // Guess from: https://sepolia.etherscan.io/address/0x30094da24be28682f2d647d405011d1d0be154cb#readProxyContract#F6
    let vk: String = "0x00ea4171dbd0027768055bee7f6d64e17e9cec99b29aad5d18e5d804b967775b".to_string();

    let l2exec = L2ExecData{
        l1Head:  b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"), 
        l2PreRoot:  b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"), 
        l2PostRoot:  b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"),
        l2BlockNumber: 4982488, // uint64
	rollupConfigHash: b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"),
	multiBlockVKey:  b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"),
    };
    /*
    //l2outputIndex: 63,        
    // contract_address: address!("30094dA24be28682F2d647D405011d1D0Be154cB"),
    // https://github.com/succinctlabs/op-succinct/blob/main/programs/aggregation/src/main.rs#L83-L102
    
    // Commit to the aggregated [`AggregationOutputs`].
    sp1_zkvm::io::commit_slice(&agg_outputs.abi_encode());
    */
    (proof, public_values, vk, l2exec)
}

fn get_l2proofofl1inclusion() -> (Vec<u8>, Vec<u8>, String, L2ProofOfL1InclusionData) {
    let proof: Vec<u8> = vec![
        0x09, 0x06, 0x90, 0x90, 0x15, 0xdc, 0xa4, 0x56, 0xba, 0x22, 0x40, 0x26, 0x15, 0x8c, 0xb0, 0xbd,
        0xec, 0x4e, 0xf3, 0x8c, 0xcf, 0x77, 0xe8, 0xf9, 0x0f, 0x38, 0x39, 0xd9, 0xd1, 0x07, 0x05, 0x54,
        0xf0, 0x00, 0x8c, 0x0f, 0x17, 0x6e, 0x86, 0x10, 0xd5, 0xda, 0xc9, 0xc3, 0xbf, 0x1d, 0x1d, 0xd2,
        0x21, 0x35, 0x45, 0x9a, 0xd2, 0xa9, 0xea, 0x33, 0xd7, 0x26, 0x9d, 0xa6, 0x8d, 0xb1, 0xc0, 0x78,
        0x2a, 0x06, 0xe6, 0x41, 0x1b, 0x7b, 0x7e, 0x7c, 0x97, 0x52, 0xac, 0xbe, 0x7c, 0x02, 0x65, 0x10,
        0xdd, 0xa3, 0xa1, 0xf0, 0x2e, 0x68, 0xfb, 0xaa, 0xc5, 0xdd, 0x4e, 0xa4, 0xe6, 0xd0, 0x45, 0xd8,
        0x94, 0x23, 0xc4, 0xeb, 0x09, 0x3b, 0x54, 0xe6, 0x63, 0xdd, 0x0c, 0xbe, 0x9d, 0xdd, 0x85, 0x78,
        0x86, 0x19, 0x7e, 0x50, 0xe5, 0xcd, 0x9e, 0xe6, 0xe8, 0x6a, 0x3d, 0x13, 0xf1, 0x72, 0x78, 0x36,
        0x20, 0x8a, 0x36, 0x74, 0x2d, 0xff, 0xe4, 0x71, 0xe6, 0xf2, 0x10, 0x57, 0x5d, 0xf6, 0xb3, 0x2c,
        0xff, 0xd4, 0x71, 0x34, 0x9f, 0x21, 0xa4, 0x0e, 0x66, 0xff, 0x82, 0xf5, 0x8e, 0x81, 0xd5, 0x22,
        0x5a, 0xb4, 0x9d, 0x60, 0x25, 0x31, 0x4a, 0x9c, 0xb9, 0x32, 0x74, 0xa1, 0x25, 0x47, 0xb3, 0x69,
        0x7a, 0xc7, 0x9c, 0x98, 0xc0, 0x62, 0xff, 0x26, 0x4b, 0xdc, 0x37, 0xdb, 0x68, 0x96, 0x32, 0xb4,
        0xe4, 0x57, 0xab, 0x4d, 0x1a, 0x43, 0xf3, 0x62, 0x34, 0xc5, 0x45, 0x67, 0x66, 0x2f, 0x25, 0x93,
        0xb6, 0x5c, 0xd3, 0xf6, 0x03, 0x5c, 0xa8, 0xcc, 0xf1, 0xbc, 0xe7, 0x3a, 0x4b, 0x60, 0xf2, 0xfc,
        0x41, 0xe4, 0x88, 0x1a, 0x2c, 0xde, 0x7b, 0x8a, 0x95, 0x95, 0x05, 0xd4, 0x34, 0xc6, 0x59, 0x21,
        0x63, 0x8d, 0x36, 0xc5, 0x56, 0x40, 0xc8, 0x8d, 0x22, 0xf4, 0x71, 0xc8, 0xc0, 0x6c, 0xd4, 0xf3,
        0x6d, 0x30, 0xd7, 0x55
    ];
    /*
    script write: https://github.com/zkzoomer/sp1-storage-proof/blob/main/script/src/bin/main.rs#L79-L83
     stdin.write::<B256>(&BLOCK_STATE_ROOT);
     stdin.write::<Address>(&CONTRACT_ADDRESS);
     stdin.write::<B256>(&slot);
     stdin.write::<U256>(&U256::from_be_slice(&MAPPING_VALUE.to_vec()));
     stdin.write::<StorageSlotProof>(&StorageSlotProof::from_account_proof_response(account_proof_response));

    program read: https://github.com/zkzoomer/sp1-storage-proof/blob/main/program/src/main.rs#L18-L22
     let state_root: B256 = sp1_zkvm::io::read::<B256>();
     let contract_address: Address = sp1_zkvm::io::read::<Address>();
     let slot: B256 = sp1_zkvm::io::read::<B256>();
     let value: U256 = sp1_zkvm::io::read::<U256>();
     let slot_proof: StorageSlotProof = sp1_zkvm::io::read::<StorageSlotProof>();

    program commit:
      NONE -- TODO: add some commits  state_root, contract_address, slot, value
        sp1_zkvm::io::commit(&state_root);
        sp1_zkvm::io::commit(&contract_address);
        sp1_zkvm::io::commit(&slot);
        sp1_zkvm::io::commit(&value);

       0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a,1732320168,4982488
       value is really 2 slots though (126+127) 
        Value at l2Outputs[126]: 0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a
        Value at l2Outputs[127]: 0x000000000000000000000000004c06d800000000000000000000000067411ba8
        sp1_zkvm::io::commit(&state_root_l1);
        sp1_zkvm::io::commit(&contract_address);
        sp1_zkvm::io::commit(&slot);           // eg 126 (127)
        sp1_zkvm::io::commit(&l2PostRoot);  // eg 0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a (126) 32 bytes
        sp1_zkvm::io::commit(&l2blocknumber);  // eg 4982488    (127a)  16 bytes in storage but really just u32 works
        sp1_zkvm::io::commit(&timestamp);      // eg 1732320168 (127b)  16 bytes in storage but really just u32 works
	// ... AND the verifier key
    */
    let vk: String = "0x0032b69684a9b1dece7f8b4a002ce30a61cb1110eabf9a5c6c458cb0f6a69258".to_string();
    let public_values: Vec<u8> = vec![
    ];
    let l2proofofl1inclusion = L2ProofOfL1InclusionData{
      state_root_l1: b256!("0000000000000000000000000000000000000000000000000000000000000003"),
      l2PostRoot: b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"),
      contract_address: address!("30094dA24be28682F2d647D405011d1D0Be154cB"),
      l2BlockNumber: 4982488,
      timestamp: 0,
      slot: 126,
    };
    (proof, public_values, vk, l2proofofl1inclusion)
}

fn get_helios() -> (Vec<u8>, Vec<u8>, String, HeliosData) {
    let proof: Vec<u8> = vec![
        9, 6, 144, 144, 44, 97, 246, 251, 229, 151, 210, 230, 246, 193, 219, 115, 153, 19,
        222, 218, 20, 110, 182, 175, 33, 180, 216, 17, 134, 54, 38, 33, 54, 218, 160, 173,
        32, 224, 56, 178, 175, 163, 181, 19, 165, 247, 209, 168, 252, 221, 98, 33, 20, 213,
        235, 169, 113, 161, 32, 165, 225, 69, 242, 69, 149, 95, 93, 180, 8, 229, 95, 110,
        193, 16, 196, 217, 84, 145, 239, 16, 132, 104, 129, 93, 23, 108, 224, 170, 161, 130,
        124, 245, 66, 113, 252, 231, 153, 25, 100, 200, 4, 70, 120, 10, 83, 149, 115, 175,
        64, 238, 196, 65, 70, 28, 34, 215, 135, 155, 229, 185, 172, 112, 1, 174, 89, 192,
        136, 70, 160, 124, 145, 15, 46, 150, 114, 182, 57, 80, 183, 101, 138, 7, 49, 132,
        120, 2, 33, 83, 225, 168, 245, 173, 124, 143, 162, 255, 237, 52, 164, 189, 60, 23,
        9, 84, 45, 227, 210, 252, 153, 144, 143, 199, 218, 172, 236, 230, 180, 78, 164, 253,
        35, 181, 58, 230, 100, 44, 28, 3, 227, 2, 233, 217, 49, 98, 214, 75, 30, 97, 213,
        17, 3, 164, 197, 179, 39, 183, 43, 15, 227, 22, 161, 235, 71, 73, 52, 213, 176, 157,
        180, 78, 212, 126, 60, 133, 150, 96, 246, 169, 6, 45, 0, 82, 77, 52, 187, 102, 85,
        168, 129, 153, 158, 50, 2, 229, 252, 142, 237, 169, 69, 194, 169, 250, 25, 214, 227,
        225, 64, 161, 120, 47
    ];
    /*
    script write:
     stdin.write(&n);           // https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/script/src/main.rs#L16
    program input:
     let n = sp1_zkvm::io::read::<u32>();     
    program commit:
     sp1_zkvm::io::commit(&n);  // https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/program/src/main.rs#L19
     sp1_zkvm::io::commit(&a);  // https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/program/src/main.rs#L35
     sp1_zkvm::io::commit(&b);  // https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/program/src/main.rs#L36
    script read: https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/script/src/main.rs#L37
     let _ = proof.public_values.read::<u32>(); // [20 0 0 0]     20    
     let a = proof.public_values.read::<u32>(); // [109 26 0 0]   6765  
     let b = proof.public_values.read::<u32>(); // [211 11 0 0]   3027  
    */
    let public_values: Vec<u8> = vec![20, 0, 0, 0, 109, 26, 0, 0, 211, 11, 0, 0];
    let vk: String = "0x006848cdd3c603c396931ccf13cb98fc2d917b4d1d2d77f46b582af9b23075b8".to_string();
    let helios = HeliosData {
    	state_root:  b256!("94DF86F9D1F61F93F8E32A46B747E8202024241DC2E8E8F7F402326A2686ED1A"),
    };
    (proof, public_values, vk, helios)
}

pub fn main() {
    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;

    // (1) proof of validity of L2 over some time period (ideally 6.4minutes, but it doesn't matter)
    let (l2exec_proof, l2exec_public_values, l2exec_vkey, l2exec) = get_l2exec();
    
    // (2) storage proof of inclusion of l2exec against a **single** L1 finalized block
    let (l2proofofl1inclusion_proof, l2proofofl1inclusion_public_values, l2proofofl1inclusion_vkey, l2proofofl1inclusion) = get_l2proofofl1inclusion();

    // (3) light client proof re finalization of l1 block
    let (helios_proof, helios_public_values, helios_vkey, helios) = get_helios();

    println!("verifying l2exec proof...");
    let result1 = Groth16Verifier::verify(&l2exec_proof, &l2exec_public_values, &l2exec_vkey, groth16_vk);

    match result1 {
       Ok(()) => {
          println!("verifying l2proofofl1inclusion proof...");
          let result2 = Groth16Verifier::verify(&l2proofofl1inclusion_proof, &l2proofofl1inclusion_public_values, &l2proofofl1inclusion_vkey, groth16_vk);
	   match result2 {
		  Ok(()) => {
		    // TODO: add asserts
		    assert!(l2exec.l2PostRoot == l2proofofl1inclusion.l2PostRoot);
		    //assert!(l2exec.l2outputIndex*2 == l2proofofl1inclusion.slot); 
		    assert!(l2exec.l2BlockNumber == l2proofofl1inclusion.l2BlockNumber);
		    //assert!(l2exec.contract_address == l2proofofl1inclusion.contract_address);
		    println!("verifying helios proof ...");
		    let result3 = Groth16Verifier::verify(&helios_proof, &helios_public_values, &helios_vkey, groth16_vk);
		    match result3 {
		       Ok(()) => {
     		       	//  5. helios.executionStateRoot = l2proofofl1inclusion.state_root_1
		// add commits for l1 (the last stateroot at a tentative checkpoint) -- just one for all N Op Succincts
		// sp1_zkvm::io::commit(&state_root_l1); 
		// sp1_zkvm::io::commit(&l1blocknumber);

		// add commits for l2 -- one for all N Op Succincts
		// sp1_zkvm::io::commit(&vk); // what uniquely maps to chain_id?
		// sp1_zkvm::io::commit(&l2PostRoot);
		// sp1_zkvm::io::commit(&l2BlockNumber);

		// add commits for l1 against sp1_helios
		// sp1_zkvm::io::commit(&newHeader);
		// sp1_zkvm::io::commit(&nextSyncCommitteeHash);
		// sp1_zkvm::io::commit(&newHead);
		// sp1_zkvm::io::commit(&prevHeader);
		// sp1_zkvm::io::commit(&syncCommitteeHash);
		       }
		       Err(e3) => {
		          println!("Error verifying proof: {:?}", e3);
		       }
		    }
		}
		Err(e2) => {
		   println!("Error verifying proof: {:?}", e2);
		}
        }
    }
    Err(e) => {
       println!("Error verifying proof: {:?}", e);
    }
   }
}


