#![no_main]
sp1_zkvm::entrypoint!(main);
use sp1_verifier::Groth16Verifier;

fn prepare_test_data_l2exec1hr() -> (Vec<u8>, Vec<u8>, String) {
    // l2exec1hr proof data (formerly fib proof)
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
    let public_values: Vec<u8> = vec![20, 0, 0, 0, 109, 26, 0, 0, 211, 11, 0, 0];
    let vk: String = "0x006848cdd3c603c396931ccf13cb98fc2d917b4d1d2d77f46b582af9b23075b8".to_string();
    (proof, public_values, vk)
}

fn prepare_test_data_l2proofofl1inclusion() -> (Vec<u8>, Vec<u8>, String) {
    // l2proofofl1inclusion proof data (formerly storage proof)
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
    let public_values: Vec<u8> = vec![];
    let vk: String = "0x0032b69684a9b1dece7f8b4a002ce30a61cb1110eabf9a5c6c458cb0f6a69258".to_string();
    (proof, public_values, vk)
}

pub fn main() {
    let groth16_vk = *sp1_verifier::GROTH16_VK_BYTES;

    // This will be 2N+1 Groth proofs, but we'll start with N=1 
    let test_cases: &[(&str, fn() -> (Vec<u8>, Vec<u8>, String))] = &[
        ("l2exec1hr", prepare_test_data_l2exec1hr), // OP Succinct 1hr (=> 12.8min) execution on Mainnet (==> ALL N OP Succinct proofs of validity, including a linkage to the previous)
        ("l2proofofl1inclusion", prepare_test_data_l2proofofl1inclusion), // storage proof of inclusion against a finalized block (==> ALL N OP Succinct storage proofs)
	// NEXT: helios proof of finality for above l1 block (just 1)
    ];

    // Loop over the test Groth16 proofs
    for (label, prepare_fn) in test_cases {
        let (proof, sp1_public_values, sp1_vkey_hash) = prepare_fn();
        
        println!("cycle-tracker-start: verify {}", label);
        let result = Groth16Verifier::verify(&proof, &sp1_public_values, &sp1_vkey_hash, groth16_vk);
        println!("cycle-tracker-end: verify {}", label);
	if label == "l2exec1hr" {
	   n = 1;
	   
        } else {
     	   n = 2;
        }
	
        match result {
            Ok(()) => {
                println!("{} proof is valid", label);
		// TODO: add OP Succinct tuple from l2exec1hr
		sp1_zkvm::io::commit(&n);
            }
            Err(e) => {
                println!("Error verifying {} proof: {:?}", label, e);
            }
        }
    }
    // NEED HELP: how do we write one groth16 proof for the 2 verified Groth16 proofs?
}



