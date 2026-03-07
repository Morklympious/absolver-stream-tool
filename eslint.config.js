import prettier from "eslint-config-prettier";
import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import svelteConfig from "./svelte.config.js";
import tivac from "@tivac/eslint-config";


export default [
	js.configs.recommended,
	...svelte.configs.recommended,
	prettier,
	tivac,
	{
		languageOptions : {
			globals : { ...globals.browser, ...globals.node },
		},
	},
	{
    extends         : [ "@tivac" ],
		files           : [ "**/*.svelte", "**/*.svelte.js" ],
		languageOptions : { parserOptions : { svelteConfig } },
	},
];
