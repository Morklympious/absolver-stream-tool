import { resolveResource as resolver } from "@tauri-apps/api/path";
import { readTextFile as text } from "@tauri-apps/plugin-fs";

/**
 * Resolve a file, more specifically a text file.
 * @param {String} filename 
 * @returns A valid file's contents
 */
export const resolve = async (filename) => {
    const path = await resolver(filename);

    if(!path) {
        console.log("path was invalid, we basically found nothing? do you have your path right?", { path })
    }

    return await text(path);
}