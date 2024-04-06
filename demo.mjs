// import { Oogway } from "oogway"
import { Oogway } from "./index.js"
import {createInterface} from "node:readline/promises"


const readline = createInterface({
    input: process.stdin,
    output: process.stdout
});

const cli_example=async()=>{
    let oogway_ai = new Oogway();
    
    console.log("\n\n Chat with Master Oogway AI \n");

    while (true) {
        let question = await readline.question('> You : ');
        process.stdout.write("> Master Oogway : ");
        let answer = await oogway_ai.ask(question);
        process.stdout.write(`${answer}\n`);
    }
}

cli_example()