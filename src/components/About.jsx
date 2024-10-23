import React from 'react';
import {useNavigate} from "react-router-dom";
import {CODE_PATH} from "../App.tsx";
import editor from "../assets/editor.png";
import rust from "../assets/rust.png";

const About = () => {
  const navigate = useNavigate();

  return (
    <div className="relative bg-black flex justify-center items-center">

      <div className="relative z-10 max-w-[62rem] w-full px-8 md:px-16 text-left mb-[3.875rem] md:mb-20 lg:mb-[6.25rem] text-white mt-4">
        <h1 className="text-5xl font-bold mb-4 leading-tight">
          ABOUT
        </h1>

        <div className="flex flex-col lg:flex-row lg:items-center">
          <p>
            EZASM is a small-instruction-set assembly-like programming language interpreter written in Rust. We will ship an IDE-like GUI interface for programming, running code, and inspecting the current 
            state of the environment. This simple interpreted language would be able to demonstrate the concepts of a lower level assembly language while still being simple to write. The instructions 
            would be intuitive and simple compared to MIPS (e.g., no system calls or immediate limits) and act upon registers akin to other assembly languages.
          </p>

          <img src={rust} alt="rust" className="lg:w-6/12 w-full"/>

        </div>
        
      </div>
    </div>
  );
}

export default About;
