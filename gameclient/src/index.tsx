import * as React from "react";
import * as ReactDOM from "react-dom";
import axios from "axios";

import { Grid } from "./components/Grid";
import { Config } from './config';

declare let config: Config; 
let positionUri = config.api_root + ":" + config.api_port + "/position";
let entitiyNum = Number.MAX_VALUE;
console.log(config);

let waitingForInputResponse = false;

window.onkeyup = function(e){
    if(waitingForInputResponse) return;

    let key = e.keyCode ? e.keyCode : e.which;
    if (key == 38) {
        movePos("down");
    }
    else if (e.keyCode == 40) {
        movePos("up");
    }
    else if (e.keyCode == 37) {
        movePos("left");
    }
    else if (e.keyCode == 39) {
        movePos("right");
    }
    // enter key code
    else if (e.keyCode == 13) {
        requestStateUpdate();
    }
}

interface Entity{
    position: number[];
}

function main(){
    initEntity();
    requestStateUpdate();
    initializeSocket(config.ws_root + ":" + config.ws_port)
}


let stateSocket: WebSocket;
function initializeSocket(address: string) {
    stateSocket = new WebSocket(address);
    stateSocket.onopen = () => { }
    stateSocket.onmessage = (msg) => {
        console.log(msg.data);
        update(JSON.parse(msg.data));
    }
}

function requestStateUpdate(){
    console.log("Updating position");
    axios.get(positionUri)
        .then(res => update(res.data))
}

function reset(){
    console.log("Resetting");
    axios.get(positionUri + "/reset")
         .then(resolve => {
             waitingForInputResponse = false;
             requestStateUpdate();
    });
}

function sleep(ms: Number){
    return new Promise(resolve => setTimeout(resolve, ms));
}

function initEntity(){
    console.log("Initializing entity");
    axios.get(positionUri + "/new")
            .then(res => {
                entitiyNum = res.data;
                requestStateUpdate();
            })
            .catch(() => initEntity())
}

function movePos(direction: string){
    waitingForInputResponse = true;
    axios.get(positionUri + "/" + entitiyNum + "/" + direction)
        .then(res => {
            waitingForInputResponse = false;
        })
}

function update(entities: Entity[]) {
    let contents: boolean[][] = [];
    console.log("Positions: " +entities.length)
    for(let i = 0; i < 10; i++){
        let row: boolean[] = [];
        for(let x = 0; x < 10; x++){
            row.push(false);
        }
        contents.push(row);
    }

    for(let i = 0; i < entities.length; i++)
    {
        let entity = entities[i];
        console.log(entity);
        contents[entity.position[0]][entity.position[1]] = true;
    }

    ReactDOM.render(
        <div>
            <Grid width={10} height={10} cellContents={contents}/>
            <span>
                <button onClick={initEntity}>Start</button>
                <button onClick={reset}>Reset</button>
            </span>
        </div>,
        document.getElementById("example")
    );
}

main();
