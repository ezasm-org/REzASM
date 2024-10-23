/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/components/*.jsx", 
        "./src/App.tsx"],
    theme: {
        extend: {
            colors: {
                'gpt-dark': '#27282c',
              },
        },
    },
    plugins: [],
};