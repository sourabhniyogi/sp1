// verifyopsuccinct.js -- shows how to verify an OP Succinct proof (groth16) posted to ETH Mainnet using sample
//  https://etherscan.io/tx/0x20cb55cb5c643654cc87bfdcfbb12c73642c6c8199573c25d573b123280038f1
// posted by Phala on Jan-08-2025 11:36:23 AM to ETH Mainnet contract:
//  https://etherscan.io/address/0xb45440830bd8d288bb2b5b01be303ae60fc855d8

// You run it (with an infura API key)
// # node verifyphala.js
// SP1 Proof verified!

const ethers = require("ethers"); // v5
const provider = new ethers.providers.JsonRpcProvider("https://mainnet.infura.io/v3/YOURINFURAKEY");

(async () => {
    try {
	// SP1 Verifier: https://etherscan.io/address/0x397A5f7f3dBd538f23DE225B51f532c34448dA9B
	// SP1 verifier - The deployed SP1Verifier contract to verify proofs.
	// 0. You can find this in the Phala contract here: https://etherscan.io/address/0xb45440830bd8d288bb2b5b01be303ae60fc855d8#readProxyContract#F29
	const contractAddress = "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B"
	// verifyProof ABI: https://etherscan.io/address/0x397A5f7f3dBd538f23DE225B51f532c34448dA9B#code
	const abi = [{
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "programVKey",
        "type": "bytes32"
      },
      {
        "internalType": "bytes",
        "name": "publicValues",
        "type": "bytes"
      },
      {
        "internalType": "bytes",
        "name": "proofBytes",
        "type": "bytes"
      }
    ],
    "name": "verifyProof",
    "outputs": [],
    "stateMutability": "view",
    "type": "function"
	}];

	// 1. aggregationVkey - The verification key of the aggregation SP1 program
	const programVKey = "0x00d4e72bc998d0528b0722a53bedd9c6f0143c9157af194ad4bb2502e37a496f";

	// 2. https://github.com/succinctlabs/op-succinct show how publicValues are computed
	const publicValues = "0x67563d903c1f507b067b54315e41f0c4617ee2e4f9c8f8ac98754dd8da616d768a222376757719a4dde372c7e7b7440590f27adff846bf64bbac31bd8d9eba76cccd6ca792d8ca5bde76888c38a79b1363af8993925b9bdfeaabf4ad0ee8b272000000000000000000000000000000000000000000000000000000000002f7f660875d3128003350b1726b6a370c2ab6fa8b8ff7802134ed77fb2caa1f1d3db833e3678015df481724af3aac49d000923caeec277027610b1490f857769f9459";

	// 3. proofBytes from https://etherscan.io/tx/0x20cb55cb5c643654cc87bfdcfbb12c73642c6c8199573c25d573b123280038f1 call data (Groth16 proofs are 260 bytes)
	const proofBytes = "0x0906909016dbcb901992b880b3eee0aea5ddc16c5226255f83e3e1b884775fb4fd6b527b0ce82f88fec32827db2741fc665ffc2958bf2b57ba908284bccb2ee45aa252c10ff15180514cb079130a4b4e2e2bdbe6c58ba5c56fba6133c70ebf504e6fabc1009043f44f784bef25682007bdb3d26149b338bca8f66e3ecbfdd5ca8dcd9fe1042b279a1a26b6d9e8d2af3da030f6604042fac61c74420b02e95fc3ab4513551e657023b8b90b1788a2c90b18d83390656e16a3dfc24a6aa8a9ed28fd2f96411d1ea3574adb9bbb1ef9c0659f978979f58e7a1a6a22c490f988229af044c7501b0098b1dafc9b3e8e4ce0012f35e3516e93d18513e6c407f2921a9e28efc680";

	const contract = new ethers.Contract(contractAddress, abi, provider);
        const result = await contract.verifyProof(programVKey, publicValues, proofBytes);
	console.log("SP1 Proof verified!");
    } catch (error) {
        console.error("Error", error);
    }
})();
