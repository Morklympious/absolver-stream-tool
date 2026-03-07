// @ts-nocheck (lol fuck you)
import { writable, get } from "svelte/store";
/**
 * Initial data shape that the update function is going to take, there WILL be fallbacks without any other provision.
 */
export const initial = () => ({
    player1 : { name : "no-name", score : 0, style : "none" },
    player2 : { name : "no-name", score : 0, style : "none" },
    bestOf  : 3,
});

const defaults = {
    player : () => ({ name : "no-name", score : 0, style : "none" })
}

/** Initial state object that components are going to modify. */
export const store = writable(initial());

export const update = (updated) => store.update((state) => {
    Object.assign(state, updated);
    console.log({update : state});
    return state;
});

export const updatePlayer = (idx = 0, info) => {
    return update({
        [!idx ? "player1" : "player2"] : Object.assign(Object.create(null), defaults.player(), info)
    });
}
