import React, { createContext, useContext, useMemo, useRef, useState } from 'react';
import { ConnectionProvider, useConnection, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter, UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-wallets';
import {
    WalletModalProvider,
    WalletDisconnectButton,
    WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';
import styles from '../styles/Index.module.css'
import Game from '../components/Game';
import * as dominari from 'dominari';

// Default styles that can be overridden by your app
require('@solana/wallet-adapter-react-ui/styles.css');

/*

Dominari                                    [Connect Wallet][Disconnect]
Join Game [GameId] -> If no Player, ask them to send Player Name to Join Game
________________________________________________________________________
Hand
[][][][]                                           Events as Popup Cards
Map
[][][][][][][][]
[]  -> Tiles show "simple info"
[]  -> Hover/Click over them to show "advanced info"
[]
[]
[]
[]
[]
_______________________________________________________________________
                        Leaderboard
                    [Name] [Score] [Kills]

*/

export const DominariContext = createContext({} as any);

export default function Wallet() {
    const [network, setNetwork] = useState("http://64.227.14.242:8899");
    const [instance, setInstance] = useState({} as dominari.GameInstance);

    const wallets = useMemo(
        () => [
            /**
             * Wallets that implement either of these standards will be available automatically.
             *
             *   - Solana Mobile Stack Mobile Wallet Adapter Protocol
             *     (https://github.com/solana-mobile/mobile-wallet-adapter)
             *   - Solana Wallet Standard
             *     (https://github.com/solana-labs/wallet-standard)
             *
             * If you wish to support a wallet that supports neither of those standards,
             * instantiate its legacy wallet adapter here. Common legacy adapters can be found
             * in the npm package `@solana/wallet-adapter-wallets`.
             */
            new PhantomWalletAdapter(),
            new UnsafeBurnerWalletAdapter(),
        ],
        // eslint-disable-next-line react-hooks/exhaustive-deps
        [network]
    );

    const rpcRef = useRef<HTMLInputElement>(null);

    return (
        <ConnectionProvider endpoint={network}>
            <WalletProvider wallets={wallets} autoConnect>
                <WalletModalProvider>
                <DominariContext.Provider 
                    value={{
                        network,
                        instance,
                        setInstance
                    }}
                >
                    <div className={styles.container}>
                        <div className={styles.header}>
                            <h1>Dominari</h1>
                            <div></div>
                            <input type="text" defaultValue={network} ref={rpcRef}></input>
                            <button onClick={() => {setNetwork(rpcRef.current?.value!);}}>Save RPC</button>
                            <WalletMultiButton />
                            <WalletDisconnectButton />
                        </div>
                        <div className={styles.game}>
                            <Game />
                        </div>
                        <div className={styles.footer}>
                            <h1>Leaderboard</h1>
                        </div>
                    </div>
                </DominariContext.Provider>
                </WalletModalProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
};