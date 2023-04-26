"use client"

import React, {useEffect, useState} from "react";
import { MdSick, MdBrokenImage } from "react-icons/md";

export default function Error({
    error,
    reset,
}: {
    error: Error;
    reset: () => void;
}) {
    const [errorMessage, setErrorMessage] = useState("");

    useEffect(() => {
       setErrorMessage(error.message);
    }, [error]);

    return (
        <div className="flex flex-col p-16">
            <div className="text-red-600 text-4xl">
                <MdBrokenImage />
            </div>
            <h1 className="text-2xl">{errorMessage}</h1>
            <div className="bottom-0 relative group">
                <div className="absolute inset-0 bg-gradient-to-tl blur-lg rounded-lg from-fuchsia-400 to-cyan-400 group-hover:animate-ping group-hover:scale-105"/>
                <button 
                    onClick={() => reset()}
                    className="flex-row items-center rounded-lg bg-black px-8 py-7 text-lg leading-none group-hover:scale-105"
                >
                    <MdSick />
                    <span>Oops</span> 
                </button>
            </div>
        </div>
    );
}
