# OP Succinct Proof of Finality Proof of Concept

Here we are trying to aggregate 2 Groth16 proofs into 1:
* program: has 2 groth16 proof verifications (sample Phala Testnet data (l2proofofinclusiononl1, l2execution)
* script: generates 1 groth16 proof (based on program)



### 1. l2proofofinclusiononl1 
 
[This contract call](https://sepolia.etherscan.io/tx/0xb979469cfdc348ae39044fe11e501928d93ae90c416b2ec7df7a70f39acac497)
(from Phala OP Succinct Testnet) at Sepolia block 7132639 on Nov-23-2024 12:02:48 AM UTC (concerning a Phala testnet 4982488 / Sepolia 7132618) writes to _l2OutputIndex (63) at [contract 0x3009..54cb](https://sepolia.etherscan.io/address/0x30094da24be28682f2d647d405011d1d0be154cb#readProxyContract) the following tuple:

```
0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a,1732320168,4982488
```  

The fetchstorage.js fetches this data 

```
# npm install ethers@5.0.0
# node fetchstorage.js 
Value at l2Outputs[126]: 0x94df86f9d1f61f93f8e32a46b747e8202024241dc2e8e8f7f402326a2686ed1a
Value at l2Outputs[127]: 0x000000000000000000000000004c06d800000000000000000000000067411ba8
```

This is used to generate one of the 2 groth16 proofs following [zkzoomer/sp1-storage-proof](https://github.com/zkzoomer/sp1-storage-proof).


### 2. L2 Execution (1 hour)

Basically [the proof here](https://sepolia.etherscan.io/tx/0xb979469cfdc348ae39044fe11e501928d93ae90c416b2ec7df7a70f39acac497)

