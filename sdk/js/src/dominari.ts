import * as anchor from '@project-serum/anchor';
import fs from 'fs';
import { Dominarisystems as ActionBundleTypes } from '../../../target/types/dominarisystems';
import { Ecs as CoreTypes } from '../../../target/types/ecs';
import { CORE, DOMINARIACTIONBUNDLE } from './constants';


/**
 * This class CREATING Transactions and FETCHING and UPDATING (on event input) state from Solana
 * 
 */
export class Dominari {
    actions: anchor.Program<ActionBundleTypes>;
    core: anchor.Program<CoreTypes>;
    instance: anchor.BN;
    
    constructor(
        connection: anchor.web3.Connection,
        instance: string
    ){
        const ActionBundleIDL = JSON.parse(fs.readFileSync('../../../target/idl/dominarisystems.json').toString());
        const CoreIDL = JSON.parse(fs.readFileSync('../../../target/idl/ecs.json').toString());

        this.actions = new anchor.Program<ActionBundleTypes>(
            ActionBundleIDL,
            DOMINARIACTIONBUNDLE,
            new anchor.AnchorProvider(
                connection,
                new anchor.Wallet(new anchor.web3.Keypair()),
                {}));

        this.core = new anchor.Program<CoreTypes>(
            CoreIDL,
            CORE,
            new anchor.AnchorProvider(
                connection,
                new anchor.Wallet(new anchor.web3.Keypair()),
                {}));

        this.instance = new anchor.BN(instance);
    }   

    async getInstanceIndex() {

    }

    // Fetch Instance Index & State
    // Update Index & State based on Event
    // Fetch Tile Info
    // Fetch Unit Info
    // Fetch Map Info

    // Start Game
    // Join Lobby

    // Attack
    // Spawn
    // Move
    
    

}