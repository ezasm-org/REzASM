import React from 'react'
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";

const Button = () => {
    const navigate = useNavigate();

    return (
        
        <button onClick={() => navigate(CODE_PATH)}>Try our Code Playground!</button>
    );
  
}

export default Button