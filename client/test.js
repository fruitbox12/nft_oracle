//client side test code

const BufferLayout = require("buffer-layout");
const sol = require("@solana/web3.js"); 
const spl = require("@solana/spl-token");
const fs = require('fs');
const borsh=require("borsh");
const { size } = require("lodash");
const cluster = sol.clusterApiUrl("devnet", true);
console.log(cluster);

const programAddr = "GK4UuXCYhFYjUbf9514GrWuCDtxd75xkqpg9bZ1K9EL7"
let prm=new sol.PublicKey(programAddr);





let alice = sol.Keypair.fromSecretKey(Buffer.from(
    [
        60,148,236,18,180,159,40,102,143,19,204,248,55,249,10,73,134,20,163,148,244,73,46,249,112,167,163,1,229,230,14,69,162,130,19,80,226,16,27,238,16,222,192,67,219,179,45,83,57,57,142,9,160,60,229,83,20,181,175,120,61,129,155,85]
        ));

     
let ownerA = sol.Keypair.generate();
let ownerB = sol.Keypair.generate();


let pda_data =new sol.Keypair();

const collectorLayout = BufferLayout.struct([
    BufferLayout.u8("instruction"),
    BufferLayout.blob(8, "number"),
  
]);

const priceLayout = BufferLayout.struct([
    BufferLayout.u8("instruction"),
    BufferLayout.blob(8, "amount"),
  
]);

async function whitelist(connection) {

    var data = Buffer.alloc(collectorLayout.span);
    collectorLayout.encode({
            // 0 initiates deposit
            instruction: 0,
            amount: new spl.u64(2).toBuffer(),
        },
        data,
    );
    
    
    console.log("ADMIN: %s", alice.publicKey.toBase58()); //admin              
    console.log("DATA:   %s", pda_data.publicKey.toBase58()); // Data storage PDA
    console.log("ownerA: %s", ownerA.publicKey.toBase58()); //creatorA             
    console.log("ownerB:   %s", ownerB.publicKey.toBase58()); //creatorB
    const instruction = new sol.TransactionInstruction({
        keys: [{
            //admin
            pubkey: alice.publicKey,
            isSigner: true,
            isWritable: true,
        }, 
        {
            // This is the system program public key.
            pubkey: sol.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },

            {// pda_data
            pubkey:  pda_data.publicKey,
            isSigner: true,
            isWritable: true,
        },
        {   
            pubkey:  ownerA.publicKey,
            isSigner: false,
            isWritable: true,
        },
        {// pda_data
            pubkey:  ownerB.publicKey,
            isSigner: false,
            isWritable: true,
        },


    ],
        programId: prm,
        data: data,
    });
    // Transaction signed by 
    tx = new sol.Transaction().add(instruction);
    return await sol.sendAndConfirmTransaction(connection, tx, [alice,pda_data],
        );
}

async function price(connection) {

    var data = Buffer.alloc(priceLayout.span);
    priceLayout.encode({
            
            instruction: 1,
            amount: new spl.u64(1000020).toBuffer(),
        },
        data,
    );
    
    
    const instruction = new sol.TransactionInstruction({
        keys: [{
            //admin
            pubkey: alice.publicKey,
            isSigner: true,
            isWritable: true,
        }, 

            {// pda_data
            pubkey: 'B9L4hCZfJJBPeqvCDVoFMenjZHVP4A2NnRhBkJqrwJYt',
            isSigner: false,
            isWritable: true,
        },
        {// creator1
            pubkey:"4uqujE5pdYyrYtrVajzdwNz3K21GSDz9NjNhgSj3n2Lq",
            isSigner: false,
            isWritable: false,
        },
        {// creator2
            pubkey:"A2uTtqEmNMUNg9eeSUNSggdUuoHj3ADzknT8LPj8Viaw",
            isSigner: false,
            isWritable: false,
        },


    ],
        programId: prm,
        data: data,
    });
                
  
    
    // Transaction signed by 
    tx = new sol.Transaction().add(instruction);

    return await sol.sendAndConfirmTransaction(connection, tx, [alice],);
}

async function main(args) {


    const conn = new sol.Connection(cluster);
    
    
    switch (args[2]) {
         case "sol":
            console.log("TXID:", await whitelist(conn));
            break;
        case "p":
            console.log("TXID:", await price(conn));
            break;
    default:
        break;
    }
}

main(process.argv).then(() => process.exit(0)).catch(e => console.error(e));