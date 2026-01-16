/**
 * @see https://prettier.io/docs/configuration
 * @type {import("prettier").Config}
 */
const config = {
	useTabs: true,
	singleQuote: true,
	trailingComma: 'none',
	printWidth: 100,
	plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss', '@prettier/plugin-oxc'],
	overrides: [
		{
			files: '*.svelte',
			options: {
				parser: 'svelte'
			}
		},
		{
			files: ['**/*.{js,mjs,cjs,jsx}'],
			options: {
				parser: 'oxc'
			}
		},
		{
			files: ['**/*.{ts,mts,cts,tsx}'],
			options: {
				parser: 'oxc-ts'
			}
		}
	]
};

export default config;
