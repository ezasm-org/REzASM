import React from 'react'
import { useLocation } from "react-router-dom";
import { disablePageScroll, enablePageScroll } from "scroll-lock";
import { navigation } from "../constants";
import ezasm from "../assets/white.svg";
import { useState } from "react";

const Header = () => {
    const pathname = useLocation();

    const [openNavigation, setOpenNavigation] = useState(false);

    const toggleNavigation = () => {
        if (openNavigation) {
            setOpenNavigation(false);
            enablePageScroll();
        } else {
            setOpenNavigation(true);
            disablePageScroll();
        }
    }

    const handleClick = () => {
        if (!openNavigation) {
            return;
        }
        enablePageScroll();
        setOpenNavigation(false);
    }

    return (
        <div 
            style={{ boxShadow: '0 10px 20px rgba(0, 0, 0, 0.3)' }}
            className={`fixed top-0 left-0 w-full z-50 border-b lg:bg-n-8/90 lg:backdrop-blur-sm bg-black
            ${openNavigation ? "bg-n-8" : "bg-n-8/90 backdrop-blur-sm"}`} 
        >
            <div className="flex items-center px-6 lg:px-12 xl:px-20 max-lg:py-4">
                <a className="flex items-center w-[12rem] xl:mr-8 ml-10 lg:ml-16" href="#hero">
                    <img src={ezasm} width={70} height={40} alt="ezasm" />
                    <span className="ml-4 text-2xl lg:text-4xl font-bold text-white">
                        EZASM
                    </span>
                </a>

                <nav
                    className={`${openNavigation ? "flex" : "hidden"} 
                    fixed top-[5rem] left-0 right-0 bottom-0 bg-black lg:static 
                    lg:flex lg:bg-transparent w-full justify-end`}
                >
                    <div className="relative z-2 flex flex-col items-center justify-center w-full lg:w-auto lg:flex-row lg:ml-auto">
                        {navigation.map((item) => (
                            <a 
                                key={item.id}
                                href={item.url}
                                onClick={handleClick}
                                className={`block relative font-code text-4xl uppercase text-n-1 transition-colors hover:text-color-1
                                ${item.onlyMobile ? "lg:hidden" : ""}
                                px-6 py-6 md:py-8 lg:-mr-0.25 lg:text-base lg:font-semibold
                                ${item.url === pathname.hash ? "z-2 lg:text-n-1" : "lg:text-n-1/50"}
                                lg:leading-5 lg:hover:text-n-1 xl:px-12`}
                            >
                                {item.title}
                            </a>
                        ))}
                    </div>
                </nav>

                <button  
                    className="ml-auto lg:hidden p-3 overflow:hidden"
                    onClick={toggleNavigation}
                >
                    {openNavigation ? (
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="white"
                            width="30px"
                            height="30px"
                        >
                            <path d="M18.3 5.71a1 1 0 0 0-1.42 0L12 10.59 7.12 5.71a1 1 0 1 0-1.42 1.42L10.59 12 5.7 16.88a1 1 0 1 0 1.42 1.42L12 13.41l4.88 4.88a1 1 0 0 0 1.42-1.42L13.41 12l4.88-4.88a1 1 0 0 0 0-1.42z"/>
                        </svg>
                    ) : (
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="white"
                            width="30px"
                            height="30px"
                        >
                            <path d="M3 6h18v2H3zm0 5h18v2H3zm0 5h18v2H3z"/>
                        </svg>
                    )}
                </button>
                
            </div>
            
        </div>
    )
}

export default Header