const ethers = require("ethers");

const provider = new ethers.providers.JsonRpcProvider("https://sepolia.infura.io/v3/c209b31b7c3e41d8a9dacce9cf5dbbf4");


var opsuccincts = {
    // phala (not active)
    "0x30094da24be28682f2d647d405011d1d0be154cb": { lasttx: "0xb979469cfdc348ae39044fe11e501928d93ae90c416b2ec7df7a70f39acac497",
						    aggregationVkey: "0x00ea4171dbd0027768055bee7f6d64e17e9cec99b29aad5d18e5d804b967775b",
						    rangeVkeyCommitment: "0x28fd6a001de229d4520a62825ba78c0d57f083fc608851b94a71a29a52080056"
						  },
    // actively posting circa 12/20/24 from 0xded0000e32f8f40414d3ab3a830f735a3553e18e
    "0x6509e7c49fff1baabd362823dd7baea5c1e1b478":  { lasttx: "0xc9df0a789eb2f8486c5a5f0600a84a39f0301ef7b259b62894dd66de52b8b628",
						   aggregationVkey: "0x00d4e72bc998d0528b0722a53bedd9c6f0143c9157af194ad4bb2502e37a496f",
						   rangeVkeyCommitment: "0x33e3678015df481724af3aac49d000923caeec277027610b1490f857769f9459"
						 },
    // actively posting circa 12/20/24 from 0xded0000e32f8f40414d3ab3a830f735a3553e18e
    "0x862d874222e3e8cc19ec023a112c76c9b621bfda": { lasttx: "0xcd634f6a4e687b6911c2263077d123101db9dfc53b845a99dc0a24ffee368a15",
						   aggregationVkey: "0x00d4e72bc998d0528b0722a53bedd9c6f0143c9157af194ad4bb2502e37a496f",
						   rangeVkeyCommitment: "0x33e3678015df481724af3aac49d000923caeec277027610b1490f857769f9459"
						  }
}

async function getStorageValueArrayAtIndex(contractAddress, baseSlot, index, provider) {
    const inp = ethers.utils.hexZeroPad(ethers.BigNumber.from(baseSlot).toHexString(), 32)
    const baseHash = ethers.utils.keccak256( inp );
    const outputRootSlot = ethers.BigNumber.from(baseHash).add(index).toHexString();
    const output = await provider.getStorageAt(contractAddress, outputRootSlot);
    return output
}

async function getL2ExecProof(txHash) {
    const tx = await provider.getTransaction(txHash);
    const iface = new ethers.utils.Interface(["function proposeL2Output(bytes32 _outputRoot,uint256 _l2BlockNumber,uint256 _l1BlockNumber,bytes _proof)"]);
    const d = iface.decodeFunctionData("proposeL2Output", tx.data);
    return {
	outputRoot: d._outputRoot,
	l2blocknumber: d._l2BlockNumber.toString(),
	l1blocknumber: d._l1BlockNumber.toString(),
	proof: d._proof
    }
}

/**
 * Calls verifyProof on the SP1VerifierGateway contract.
 * @param {string} programVKey A 32-byte hex string (e.g. "0x...") representing the program verification key.
 * @param {string} publicValues A hex string representing the public values (e.g. "0x...").
 * @param {string} proofBytes A hex string representing the proof bytes (e.g. "0x...").
 * @returns {Promise<boolean>} The result of the verifyProof call.
 */
async function verifyProofOnContract(programVKey, publicValues, proofBytes) {
    try {
	const contractAddress = "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B"; // sepolia
	const abi = [
	    "function verifyProof(bytes32 programVKey, bytes publicValues, bytes proofBytes) external view returns (bool)"
	];
	const contract = new ethers.Contract(contractAddress, abi, provider);
	
        const result = await contract.verifyProof(programVKey, publicValues, proofBytes);
        return result;
    } catch (error) {
        console.error("Error calling verifyProof:", error);
        throw error;
    }
}

(async () => {
    
    try {
	for (const [contractAddress, opsuccinct] of Object.entries(opsuccincts)) {
	    const l2exec = await getL2ExecProof(opsuccinct.lasttx);
	    
	    const programVKey = opsuccinct.vk;
	    const proofBytes = l2exec.proof;
            const baseSlot = 3;
	    let vals = {}
            for (let index = 0; index < 2; index++) {
                let value = await getStorageValueArrayAtIndex(contractAddress, baseSlot, 126+index, provider);  // 63 => 126+127
		vals[index] = value;
		console.log(`Storage Value at l2Outputs[${index}]:`, value);
            }
	    // TODO: figure this out so that the verification passes from https://github.com/succinctlabs/op-succinct
	    const publicValues = "0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a"; // 0x000000000000000000000000004c06d800000000000000000000000067411ba8";
	    console.log(`programVKey:`, programVKey);
	    console.log(`publicValues:`, publicValues);
	    console.log(`proofBytes:`, proofBytes);
	    const result = await verifyProofOnContract(programVKey, publicValues, proofBytes);
	} 
    } catch (error) {
        console.error("Error reading storage:", error);
    }
})();

