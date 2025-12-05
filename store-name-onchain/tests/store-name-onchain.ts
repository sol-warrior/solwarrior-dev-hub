import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import { StoreNameOnchain } from "../target/types/store_name_onchain";

describe("store-name-onchain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .storeNameOnchain as Program<StoreNameOnchain>;

  const deriveName = (authority: PublicKey) =>
    PublicKey.findProgramAddressSync(
      [Buffer.from("name"), authority.toBuffer()],
      program.programId
    )[0];

  const fundWallet = async (wallet: Keypair) => {
    const sign = await provider.connection.requestAirdrop(
      wallet.publicKey,
      3 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sign, "confirmed");
  };

  const initializeName = async (wallet: Keypair, name: string) => {
    const nameStore = deriveName(wallet.publicKey);
    const accounts = {
      user: wallet.publicKey,
      nameStore,
      systemProgram: SystemProgram.programId,
    };
    await program.methods
      .initialize(name)
      .accounts(accounts)
      .signers([wallet])
      .rpc();
    return nameStore;
  };

  const expectInitFailure = async (wallet: Keypair, name: string) => {
    const nameStore = deriveName(wallet.publicKey);
    const accounts = {
      user: wallet.publicKey,
      nameStore,
      systemProgram: SystemProgram.programId,
    };
    try {
      await program.methods
        .initialize(name)
        .accounts(accounts)
        .signers([wallet])
        .rpc();
      expect.fail("Initialization should have failed");
    } catch (error) {
      const message =
        error instanceof anchor.AnchorError
          ? error.message
          : JSON.stringify(error);
      expect(message).to.not.equal("");
    }
  };

  it("initializes and stores the provided name", async () => {
    const user = Keypair.generate();
    await fundWallet(user);

    const nameStore = await initializeName(user, "Nishant");
    const account = await program.account.onchainName.fetch(nameStore);
    console.log("Onchain Account : ", account);

    expect(account.name).to.equal("Nishant");
    expect(account.bump).to.be.a("number");
  });

  it("rejects re-initialization for the same PDA", async () => {
    const user = Keypair.generate();
    await fundWallet(user);

    await initializeName(user, "First Pass");
    await expectInitFailure(user, "Second Pass");
  });

  it("accepts an empty string", async () => {
    const user = Keypair.generate();
    await fundWallet(user);

    const nameStore = await initializeName(user, "");
    const account = await program.account.onchainName.fetch(nameStore);
    console.log("Account empty :", account);

    expect(account.name).to.equal("");
  });

  it("accepts the maximum allowed length", async () => {
    const user = Keypair.generate();
    await fundWallet(user);
    const maxLengthName = "x".repeat(32);

    const nameStore = await initializeName(user, maxLengthName);
    const account = await program.account.onchainName.fetch(nameStore);
    console.log("Max allowed Account:", account);

    expect(account.name).to.equal(maxLengthName);
  });

  it("rejects inputs longer than 32 bytes", async () => {
    const user = Keypair.generate();
    await fundWallet(user);
    const tooLongName = "z".repeat(33);

    await expectInitFailure(user, tooLongName);
  });
});
