import React from 'react'

const Contributors = () => {
  return (
    <div className="relative bg-black flex justify-center items-center"
     style={{
      background: 'radial-gradient(ellipse 50% 90% at center, rgba(0, 102, 250, 0.3), transparent 60%), black',
      position: 'relative'
    
    }}>
      <div className="relative z-10 max-w-[62rem] w-full px-8 md:px-16 text-left mb-[3.875rem] md:mb-20 lg:mb-[6.25rem] text-white mt-4">
        <h1 className="text-5xl font-bold mb-4 leading-tight">
          Contributions
        </h1>

        <p className="mb-4 text-lg">
          You can view the list of contributors who have helped build this project. If you would like to contribute yourself, feel free to check out our GitHub repository for more information.
        </p>

        <a
          href="https://github.com/ezasm-org/rezasm"
          target="_blank"
          className="text-blue-500 underline"
        >
          https://github.com/ezasm-org/rezasm
        </a>

      </div>
    </div>
  );
}

export default Contributors