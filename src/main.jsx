import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import './styles.css'
import { BrowserRouter as Router } from 'react-router-dom'
import '../dist/output.css'; 

ReactDOM.createRoot(document.getElementById("root")).render(
    <React.StrictMode>
            <App />
    </React.StrictMode>,
);
