"use client";

// external
import React, { useState, useEffect } from "react";

// internal
import { Models, Routes } from "../../index";

export default function StartGame(): JSX.Element {

    const [ create, setCreate ] = useState({
        game_name: "",
        game_ruler: "",
        game_num_players: 0
    } as Models.GameCreation);
    const [ ruler, setRuler ] = useState("");
    const [ country, setCountry ] = useState("");

    useEffect(() => {
        setCreate({
            game_num_players: 0,
            game_ruler: ruler,
            game_name: country
        } as Models.GameCreation) 
    }, [ruler, country])

    const handleSubmit = () => {
        console.log({"submitted": create});
    };

    return (
        <form 
            className="flex flex-col" 
            onSubmit={handleSubmit}
        >
            <label>
                Country Name 
                <input 
                    type="text"
                    value={country}
                    placeholder="Hamunaptra"
                    onChange={(e) => {
                        setCountry(e.currentTarget.value);
                    }}
                />
            </label>
            <label>
                Ruler Name 
                <input 
                    type="text"
                    value={ruler}
                    placeholder="Imohtep"
                    onChange={(e) => {
                        setRuler(e.currentTarget.value);
                    }}
                />
            </label>
            <button type="submit">
            </button>
        </form>
    );
}
