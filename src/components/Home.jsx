import React from "react";
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";
import { DOWNLOAD_PATH } from "../App.tsx";
import Header from "./Header";

import Editor from "./Editor.jsx";

function Home() {
    const navigate = useNavigate();

    return (
        <div className="pt-[4.75rem] lg:pt-[5.25rem] overflow-hidden">
            <Header />
            <p>Home</p>
            <button onClick={() => navigate(CODE_PATH)}>Take me to the code</button>
            
        </div>
    );
}

export default Home;
