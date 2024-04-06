import { Oogway } from "./index.js"


const example=async()=>{
    let ai = new Oogway();
    await ai.ask("What is the meaning of life?")
}

example()