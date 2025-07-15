import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NonCustodialEscrow } from "../target/types/non_custodial_escrow";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token";
import { SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
describe("non-custodial-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .nonCustodialEscrow as Program<NonCustodialEscrow>;

  const seller = provider.wallet.publicKey;
  const buyer = anchor.web3.Keypair.generate();
  const payer = (provider.wallet as NodeWallet).payer;
  const escrowedXTokens = anchor.web3.Keypair.generate();
  let x_mint: anchor.web3.PublicKey;
  let y_mint: anchor.web3.PublicKey;

  let seller_x_token: anchor.web3.PublicKey;
  let seller_y_token: anchor.web3.PublicKey;
  let buyer_x_token: anchor.web3.PublicKey;
  let buyer_y_token: anchor.web3.PublicKey;
  let escrow: anchor.web3.PublicKey;

  before(async () => {
    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("escrow"), seller.toBuffer()],
      program.programId
    );

    x_mint = await createMint(provider.connection, payer, seller, null, 6);
    y_mint = await createMint(provider.connection, payer, seller, null, 6);
    seller_x_token = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        x_mint,
        seller
      )
    ).address;
    await mintTo(
      provider.connection,
      payer,
      x_mint,
      seller_x_token,
      payer,
      10_000_000_000
    );

    seller_y_token = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        y_mint,
        seller
      )
    ).address;
    await mintTo(
      provider.connection,
      payer,
      y_mint,
      seller_y_token,
      payer,
      10_000_000_000
    );

    buyer_x_token = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        x_mint,
        buyer.publicKey
      )
    ).address;
    await mintTo(
      provider.connection,
      payer,
      x_mint,
      buyer_x_token,
      payer,
      10_000_000_000
    );

    buyer_y_token = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        y_mint,
        buyer.publicKey
      )
    ).address;
    await mintTo(
      provider.connection,
      payer,
      y_mint,
      buyer_y_token,
      payer,
      10_000_000_000
    );
  });

  it("Is initialized!", async () => {
    const x_amount = new anchor.BN(100);
    const y_amount = new anchor.BN(40);
    const sellerXBefore = (
      await getAccount(provider.connection, seller_x_token)
    ).amount;

    const tx = await program.methods
      .initialize(x_amount, y_amount)
      .accounts({
        seller,
        xMint: x_mint,
        yMint: y_mint,
        sellerXToken: seller_x_token,
        escrow,
        escrowedXTokens: escrowedXTokens.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([escrowedXTokens])
      .rpc();
    const sellerXAfter = (await getAccount(provider.connection, seller_x_token))
      .amount;
    const escrowedXAfter = (
      await getAccount(provider.connection, escrowedXTokens.publicKey)
    ).amount;
    console.log(
      "Seller X Before:",
      sellerXBefore.toString(),
      "After:",
      sellerXAfter.toString()
    );
    console.log(
      "Escrowed X Before: Does not exist",
      "Escrowed X After:",
      escrowedXAfter.toString()
    );
  });

  it("Is Accepted!", async () => {
    const sellerXBefore = (
      await getAccount(provider.connection, seller_x_token)
    ).amount;
    const buyerXBefore = (await getAccount(provider.connection, buyer_x_token))
      .amount;
    const sellerYBefore = (
      await getAccount(provider.connection, seller_y_token)
    ).amount;
    const buyerYBefore = (await getAccount(provider.connection, buyer_y_token))
      .amount;
    const tx = await program.methods
      .accept()
      .accounts({
        buyer: buyer.publicKey,
        sellerYToken: seller_y_token,
        buyerXToken: buyer_x_token,
        buyerYToken: buyer_y_token,
        escrow,
        escrowedXTokens: escrowedXTokens.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();
    const sellerXAfter = (await getAccount(provider.connection, seller_x_token))
      .amount;
    const buyerXAfter = (await getAccount(provider.connection, buyer_x_token))
      .amount;
    const sellerYAfter = (await getAccount(provider.connection, seller_y_token))
      .amount;
    const buyerYAfter = (await getAccount(provider.connection, buyer_y_token))
      .amount;
    const escrowedXAfter = (
      await getAccount(provider.connection, escrowedXTokens.publicKey)
    ).amount;
    console.log(
      "Seller X Before:",
      sellerXBefore.toString(),
      "After:",
      sellerXAfter.toString()
    );
    console.log(
      "Buyer X Before:",
      buyerXBefore.toString(),
      "After:",
      buyerXAfter.toString()
    );
    console.log(
      "Seller Y Before:",
      sellerYBefore.toString(),
      "After:",
      sellerYAfter.toString()
    );
    console.log(
      "Buyer Y Before:",
      buyerYAfter.toString(),
      "After:",
      buyerYAfter.toString()
    );
    console.log(
      "Escrowed X Before: Does not exist",
      "Escrowed X After:",
      escrowedXAfter.toString()
    );
  });

  it("Is Cancel!", async () => {
    const sellerXBefore = (
      await getAccount(provider.connection, seller_x_token)
    ).amount;

    const tx = await program.methods
      .cancel()
      .accounts({
        seller,
        escrow,
        sellerXToken: seller_x_token,
        escrowedXTokens: escrowedXTokens.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  });
});
