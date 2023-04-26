import React, { } from "react";

import Link from "next/link";

export default async function Game() {
    return (
        <div className="flex flex-col w-4/5">
            <div className="relative group">
                <div className="absolute w-full flex-inset-0 bg-gradient-to-tl blur-lg rounded-lg from-fuchsia-400 to-cyan-400 group-hover:animate-ping group-hover:scale-105"/>
                <Link
                    href={"/app/start/create"}
                    className="flex-row items-center rounded-lg bg-black px-8 py-7 text-lg leading-none group-hover:scale-105"
                >
                    <span>Create a Game to Play.</span>
                </Link>
            </div>
            <div className="relative group">
                <div className="absolute w-full flex-inset-0 bg-gradient-to-tl blur-lg rounded-lg from-fuchsia-400 to-cyan-400 group-hover:animate-ping group-hover:scale-105"/>
                <Link
                    href={"/app/start/join"}
                    className="flex-row items-center rounded-lg bg-black px-8 py-7 text-lg leading-none group-hover:scale-105"
                >
                    <span>Join a Game!</span>
                </Link>
            </div>
            <div className="relative group">
                <div className="absolute w-full flex-inset-0 bg-gradient-to-tl blur-lg rounded-lg from-fuchsia-400 to-cyan-400 group-hover:animate-ping group-hover:scale-105"/>
                <Link
                    href={"/app/start/get_in"}
                    className="flex-row items-center rounded-lg bg-black px-8 py-7 text-lg leading-none group-hover:scale-105"
                >
                    <span>Get back into a Game!</span>
                </Link>
            </div>
        </div>
    );
}
