"use client";

// external
import React, { useState } from "react";
import Link from "next/link";

// internal
import { GameInfo, GameAPI } from "../../../index";
import { FaDiceD20 } from "react-icons/fa";

const start = async (name: string, ruler: string): Promise<String> => {
    let response = await GameAPI.start({
        game_name: name,
        game_ruler: ruler,
        game_num_players: 0,
    });
    if (response.success != "Succeeding") {
        throw new Error(response.success.Failing.message);
    }
    return response.data;
};

export default async function StartGame() {
    const [name, setName] = useState("");
    const [ruler, setRuler] = useState("");

    return (
        <div className="flex flex-col">
            <div className="text-3xl text-sky-600">Start a Game</div>
            <form
                onSubmit={() => {
                    let pass = "";
                    start(name, ruler).then((res) => {
                        pass = res.toString();
                    });

                    location.href = `/app/game/create_character/${pass}`
                }}
            >
                <div className="flex-row">
                    <input
                        className="mr-4 flex h-24 w-1/2 transform-gpu flex-col text-lg rounded-lg bg-slate-800 px-8 py-2 text-blue-600 placeholder-gray-300 duration-200 focus:scale-110 focus:shadow-xl focus:shadow-cyan-200"
                        placeholder={"name"}
                        onChange={(e) => setName(e.currentTarget.value)}
                    />
                    <input
                        className="flex h-24 w-1/2 transform-gpu flex-col rounded-lg text-lg bg-slate-800 px-8 py-2 text-blue-600 placeholder-gray-300 duration-200 focus:scale-110 focus:shadow-xl focus:shadow-cyan-200"
                        placeholder={"ruler"}
                        onChange={(e) => setRuler(e.currentTarget.value)}
                    />
                </div>
                <div className="relative group">
                    <div className="absolute inset-0 bg-gradient-to-tl blur-lg rounded-lg from-fuchsia-400 to-cyan-400 group-hover:animate-ping group-hover:scale-105"/>
                    <button className="flex-row items-center rounded-lg bg-black px-8 py-7 text-lg leading-none group-hover:scale-105">
                        <FaDiceD20 />
                        <span>Start!</span> 
                    </button>
                </div>
            </form>
        </div>
    );
}
