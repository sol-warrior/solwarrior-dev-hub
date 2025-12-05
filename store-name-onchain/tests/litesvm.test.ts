
import { Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from "@solana/web3.js";
import { LiteSVM } from "litesvm";
import IDL from "../target/idl/store_name_onchain.json" with {
    type: "json"
};
import anchor from "@coral-xyz/anchor";
import { assert } from "chai";

describe("LiteSVM: Store name onchain", () => {
    const svm = new LiteSVM();
    const payer = Keypair.generate();
    const programId = new PublicKey(IDL.address);

    svm.airdrop(payer.publicKey, BigInt(1000000000));
    const coder = new anchor.BorshCoder(IDL as anchor.Idl);

    const programPath = new URL("../target/deploy/store_name_onchain.so", import.meta.url).pathname
    svm.addProgramFromFile(programId, programPath);


    const [nameStore] = PublicKey.findProgramAddressSync(
        [Buffer.from("name"), payer.publicKey.toBuffer()], programId
    );

    it("Create name store", () => {
        const data = coder.instruction.encode("initialize", { name: "solwarrior" });

        const ix = new TransactionInstruction({
            keys: [
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                { pubkey: nameStore, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },

            ],
            programId,
            data
        });

        const tx = new Transaction().add(ix);
        tx.feePayer = payer.publicKey;
        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(payer);
        const res = svm.sendTransaction(tx);

        // console.log("Sig: ",res.toString());


        //Fetch account:
        const nameAccount = svm.getAccount(nameStore);
        const nameInfo = coder.accounts.decode("OnchainName",Buffer.from(nameAccount.data));

        console.log(nameInfo);
        assert.equal("solwarrior",nameInfo.name);
    })
}) 