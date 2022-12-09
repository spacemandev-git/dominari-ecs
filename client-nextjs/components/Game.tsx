import { useRef, useState, useContext, useReducer } from "react"
import styles from '../styles/Game.module.css'
import * as dominari from 'dominari';
import { DominariContext } from "../pages";
import { WORLD_KEY } from "../constants";

export default function Game(){
    const gameIDRef = useRef<HTMLInputElement>(null);
    const [isGame, setIsGame] = useState(false);
    const [instance, setInstance] = useState({} as dominari.GameInstance);
    const {network} = useContext(DominariContext);

    const tryJoinGame = () => {
        console.log(`Trying to connect to Game ID ${gameIDRef.current?.value} @ World ${WORLD_KEY} on RPC ${network}`);
        dominari.GameInstance.new(network, WORLD_KEY, BigInt(gameIDRef.current?.value as string))
        .then((gameInstance) => {
            setInstance(gameInstance)
            setIsGame(true);
        })
        .catch((err) => {alert!(err)})
    }

    if(!isGame){
        return(
            <div>
                <div>
                    <input defaultValue={"GameID"} type="text" ref={gameIDRef}></input>
                    <button onClick={()=>{tryJoinGame()}}>Join Game</button>
                </div>
                <div>
                    <input type="file"></input>
                    <button>Create Game</button>
                </div>
            </div>
        )
    } else {
        return(
            <div className={styles.gameContainer}>
                
            </div>
        )
    }

}

function Hand(){
    return(
    <div>

    </div>
    )
}

function Map(){
    return(
    <div>

    </div>
    )
}
