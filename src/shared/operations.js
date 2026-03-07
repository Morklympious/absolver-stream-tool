/**
 * Take a svelte store and information about a player, a player shape and
 * update the store based on that.
 * @param {import("svelte/store").Writable<*>} store
 * @param {number} index
 * @param {*} info
 * @returns {void}
 */
const name = (store, index = 0, updated) => {
    /** If the index is 0, we're using player1, player2 otherwise. */
    const key = !index ? "player1" : "player2";

    const payload = { name : updated };

    /** Basically update player1 or player2 */
    store.update((current) => {
        Object.assign(current[key], payload);
        
        return current;
    });

    return;
};

/**
 *
 * @param {import("svelte/store").Writable<*>} store
 * @param {number} count
 */
const box = (store, count) => {
    store.update((current) => {
        current.bestOf = count;
    });
};

export default {
    update : {
        name,
        box,
    },
};

