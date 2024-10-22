import React from 'react'
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";

const Button = () => {
    const navigate = useNavigate();

    return (
        <button onClick={() => navigate(CODE_PATH)}>Take me to the code</button>
    );
  
}

export default Button