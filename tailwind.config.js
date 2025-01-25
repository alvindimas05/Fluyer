/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {
            screens: {
                lg: {
                    raw: "(min-width: 1024px) and (max-resolution: 1.0dppx)",
                },
                // tb: FHD Tablet
                tb: {
                    raw: "(min-width: 768px) and (max-width: 1920px) and (min-resolution: 1.01dppx)",
                },
            },
        },
        // animatedSettings: {
        //     classes: ["fadeIn"],
        // },
        // plugins: [require("tailwindcss-animatecss")],
    },
};
