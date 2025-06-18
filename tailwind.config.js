/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			screens: {
				"md-hdpi": {raw: "(min-width: 768px) and (min-resolution: 1.01dppx)"},
				"lg-hdpi": {raw: "(min-width: 1024px) and (min-resolution: 1.01dppx)"},
				"md-xhdpi": {raw: "(min-width: 768px) and (min-resolution: 1.5dppx)"},
				"lg-xhdpi": {raw: "(min-width: 1024px) and (min-resolution: 1.5dppx)"},
			}
		},
		// animatedSettings: {
		//     classes: ["fadeIn"],
		// },
		// plugins: [require("tailwindcss-animatecss")],
	},
};
