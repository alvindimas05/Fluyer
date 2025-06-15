/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			screens: {
				lg: {raw: "(min-width: 1024px) and (max-resolution: 1.0dppx)"},
				hdpi: {raw: "(min-width: 768px) and (max-width: 1920px) and (min-resolution: 1.01dppx) and (max-resolution: 1.49dppx)"},
				xhdpi: {raw: "(min-width: 768px) and (min-resolution: 1.5dppx)"},
			}
		},
		// animatedSettings: {
		//     classes: ["fadeIn"],
		// },
		// plugins: [require("tailwindcss-animatecss")],
	},
};
