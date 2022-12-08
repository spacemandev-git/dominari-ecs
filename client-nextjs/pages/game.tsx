import Head from 'next/head';
import * as dominari from 'dominari';
import { useEffect } from 'react';
import * as anchor from '@project-serum/anchor';
import {decode} from 'bs58';

export default function Game() {
    const rpc = "http://64.227.14.242:8899"
    const world = "H5mieGWWK6qukHoNzbR6ysLxReeQC4JHZcNM6JkPQnm3";
    const instance = BigInt(1); 
    const gameInstance = new dominari.GameInstance(rpc, world, BigInt(instance));
    // Apollo Testing Keypair
    console.log("KEY: ", process.env.NEXT_PUBLIC_KEY);
    const keypair = anchor.web3.Keypair.fromSecretKey(decode(process.env.NEXT_PUBLIC_KEY as string));
    console.log("Keypair: ", keypair.publicKey.toString());
    const connection = new anchor.web3.Connection(rpc);
    // load blueprint names

    
    useEffect(() => {
        
        (async () => {
            //const airdropSig = await connection.requestAirdrop(keypair.publicKey, 10e9);
            //await connection.confirmTransaction(airdropSig);
            await gameInstance.build_game_state(instance);           
            let blueprintNames = await (await fetch('blueprints.json')).json()
            console.log(blueprintNames);
            await gameInstance.load_blueprints(blueprintNames);
        
            
            let instructions = gameInstance.spawn_unit(
                keypair.publicKey.toBase58(),
                BigInt("6579140947125516265"),
                1,
                1,
                "Scout"
            );
            console.log("Old Ixs: ", instructions);
            
            let new_ixs = instructions.map( (ix: any) => {
                return new anchor.web3.TransactionInstruction({
                    programId: new anchor.web3.PublicKey(ix.program_id as Uint8Array), 
                    keys: ix.accounts.map((account:any) => {
                        account.isSigner = account.is_signer,
                        account.isWritable = account.is_writable,
                        account.pubkey = new anchor.web3.PublicKey(account.pubkey as Uint8Array);
                        return account;
                    }),
                    data: ix.data as Buffer,
                });
            });
            console.log("New Ixs: ", new_ixs);

            const tx = new anchor.web3.Transaction();
            for (let ix of new_ixs){
                tx.add(ix);
            }

            const sig = await anchor.web3.sendAndConfirmTransaction(
                connection,
                tx,
                [keypair],
                {skipPreflight: true}
            );
            console.log(sig);
        })()

        return () => {}
    }); 


    return (
        <div></div>
    )
}