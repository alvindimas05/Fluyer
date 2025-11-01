/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			screens: {
				"md-mdpi": { raw: "(min-width: 768px) and (max-resolution: 1.0dppx)" },
				"lg-mdpi": { raw: "(min-width: 1024px) and (max-resolution: 1.0dppx)" },
				"xl-mdpi": { raw: "(min-width: 1280px) and (max-resolution: 1.0dppx)" },
				"2xl-mdpi": {
					raw: "(min-width: 1536px) and (max-resolution: 1.0dppx)",
				},

				"sm-hdpi": { raw: "(min-width: 640px) and (min-resolution: 1.01dppx)" },
				"md-hdpi": { raw: "(min-width: 768px) and (min-resolution: 1.01dppx)" },
				"lg-hdpi": {
					raw: "(min-width: 1024px) and (min-resolution: 1.01dppx)",
				},
				"xl-hdpi": {
					raw: "(min-width: 1280px) and (min-resolution: 1.01dppx)",
				},
				"2xl-hdpi": {
					raw: "(min-width: 1536px) and (min-resolution: 1.01dppx)",
				},

				"md-xhdpi": {
					raw: "(min-width: 768px) and (min-resolution: 2.01dppx)",
				},
				"lg-xhdpi": {
					raw: "(min-width: 1024px) and (min-resolution: 2.01dppx)",
				},
				"xl-xhdpi": {
					raw: "(min-width: 1280px) and (min-resolution: 2.01dppx)",
				},
				"2xl-xhdpi": {
					raw: "(min-width: 1536px) and (min-resolution: 2.01dppx)",
				},
			},
		},
		// animatedSettings: {
		//     classes: ["fadeIn"],
		// },
		// plugins: [require("tailwindcss-animatecss")],
	},
};
