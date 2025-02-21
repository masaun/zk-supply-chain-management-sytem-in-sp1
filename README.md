# ZK Supply Chain Management System


# 【Architecture】

## Actors in Supply Chain
- 1/ Supplier (= Producer)
- 2/ Manufacturing company
- 3/ Wholesaler (= Distributor) (NOTE：If this is ordered via online marketplace, this process will be cut off) 
- 4/ Retailer (NOTE：If this is ordered via online marketplace, this process will be cut off) 
- 5/ Logistics company
- 6/ Consumer (= Buyer)
   <img width="497" alt="Image" src="https://github.com/user-attachments/assets/e0332e9c-9a28-4533-9f4d-0368db6d61a6" />

(Source：https://www.debutinfotech.com/blog/blockchain-in-supply-chain-challenges-benefits-use-cases-considerations )

<br>

## Architecture

- Prepare 3 ZK circuit for proving 3 ZKPs:
  - 1/ Proving a ZKP for a consumer/buyer (/w the `Order Detail` struct data)
    - icl. Non ZKP in the `Order Detail` struct data
  - 2/ Proving a ZKP for a supplier/producer (/w the `Product Detail` struct data)
    - icl. a `Order ZKP` in the `Product Detail` struct data
  - 3/ Proving a ZKP for a logistics company (/w the `Logistics Detail` struct data)
    - icl. a `Order ZKP` + `Product Detail ZKP` in the `Logistics Detail` struct data
  - 4/ When a product would be delivered at the consumer's house🏡, the consumer will be able to confirm whether or not the product is the product that the consumer him/herself actually ordered - by calling a view function on blockchain /w the `proof` that was submitted onto blockchain when the step 1/ and they hold. 


<br>

- When a ZKP is proved in each stage, the `submitProof()` should be called with a ZKP (`proof`) and a `signature` (signed with an actor's wallet `PK`)  via SC on the Blockchain in each stage.
  - Each ZKP would be stored into the **"private" `mapping storage` of product order**🟣, which will become the `ZKP-tree` 🌳

<br>

## Directory composition

```bash
zk-for-supply-chain-in-sp1
├─── zk-circuit-for-consumer
│    └─── program
│    └─── script
└─── zk-circuit-for-supplier
│    └─── program
│    └─── script
├─── zk-circuit-for-logistics-company
│    └─── program
│    └─── script
```




<br>

## Flow
- **Smart Contract ①**:
  - 1/ Buyer would pay the fees for logistics -> The payment will be deposited into the SC. 

<br>

- **ZK circuit** for proving a ZKP:
  - 2/ In each check point, each supplier generate a ZKP + send (submit) it to the blockchain.
    


<br>

- **Smart Contract ②**:
  - 3/ **All** ZKPs, which was submitted from each supplier in each check point, would be stored in the SC.🟣
     => Using the mapping storages + the array storage

  - 4/ Once the logistics process would be done (finalized), each supplier can receive the fees-earned via calling the `claim()` /w their `proof`. 🟣
     => These fees will be transferred from the SC①.