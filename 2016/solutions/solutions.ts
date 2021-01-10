import {solve as d1} from "./day1/sol.ts";
import {solve as d2} from "./day2/sol.ts";

const solutions: {[key: string]: () => Promise<void>}  = {
    d1,
    d2
}

export { solutions }