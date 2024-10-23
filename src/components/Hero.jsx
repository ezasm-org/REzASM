import React from 'react';
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";
import editor from "../assets/editor.png";

const Hero = () => {
  const navigate = useNavigate();

  return (
    <div className="relative bg-black min-h-screen flex justify-center items-center"
        style={{
            background: 'radial-gradient(ellipse 50% 90% at center, rgba(0, 102, 250, 0.3), transparent 60%), black',
            position: 'relative'
    }}>

      <div className="relative z-10 max-w-[62rem] w-full px-8 md:px-16 text-left mb-[3.875rem] md:mb-20 lg:mb-[6.25rem] text-white mt-10">
        <h2 className="text-xl font-semibold mb-2">
          EZASM
        </h2>

        <h1 className="text-5xl font-bold mb-4 leading-tight">
           Assembly-like programming language for education
        </h1>

        <p className="text-lg text-gray-400 mb-8 max-w-3xl">
          The programming language that makes learning assembly a more enjoyable experience.
        </p>

        <button 
          onClick={() => navigate(CODE_PATH)} 
          className="bg-white text-black rounded-full px-6 py-3 font-semibold shadow-md border-black"
          style={{ boxShadow: '0 10px 20px rgba(0, 0, 0, 0.3)' }}>
          Try our Code Playground!
        </button>


        <div className="w-full max-w-[49rem] mt-8">
          <img src={editor} alt="Editor" className="rounded-lg w-full" />
        </div>

      </div>

    </div>
  );
}

export default Hero;
