import React from "react";

const Footer = () => {
  return (
    <div className="w-full flex justify-center items-center bg-black py-4">
      <p className="caption text-n-4 lg:block text-white">
        © {new Date().getFullYear()}. All rights reserved.
      </p>
    </div>
  );
};

export default Footer