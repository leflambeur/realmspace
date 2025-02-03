/** @type {import('tailwindcss').Config} */

module.exports = {
    content: {
        files: ["*.html", "./app/src/**/*.rs"],
        transform: {
            'rs': (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
        presets: [],
    },
    plugins: [],
    theme: {
        colors: {
            p1: '#3f6d9e',
            p2: '#151640',
            p3: '#f783b0',
            p4: '#e6f2ef',
        },
        fontFamily: {
            sans: ['Stepalange', 'sans-serif'],
        },
        extend: {
            boxShadow: {},
        },
    },
}
