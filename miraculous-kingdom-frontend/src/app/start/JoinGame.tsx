"use client";

// external
import React from "react";

// internal
import { Models, Routes } from "../../index";

const getGames = async (): Promise<Models.GameInfo[]> => {
    let API = new Routes.ApigameApiApi();
    let response: Models.GamesInfoDetailedResponse
        = API.getGames();
    if (response.success) {
        return response.data;
    } else {
        throw new Error(response.message);
    }
}

export default async function JoinGame() {
    const games = await getGames();

    return (
        <div className="flex flex-col">
            {games.length > 0 
              ? games.map((game) => {
                return (
                    <div className="grid grid-rows-2 rounded-lg bg-slate-400 text-slate-600">
                        <div className="grid-cols-2">
                            <div className="flex flex-row justify-center">
                                <div className="text-red-600">Country: </div>
                                {game.game_name}
                            </div>
                            <div className="flex flex-row justify-center">
                                <div className="text-red-600">Ruler: </div>
                                {game.game_ruler}
                            </div>
                        </div>
                        <div className="flex flex-col justify-start">
                            <div className="text-red-600">Characters: </div>
                            { game.game_chars.length > 0 
                              ? game.game_chars.map((name) => {
                                    return (<div>
                                        {name}
                                    </div>);
                                })
                              : (<div>Please Create a Character for this Game</div>)
                            }
                        </div>
                    </div> 
                );
            })
              : <div>Make sure to start a game before joining one!</div>
            }
        </div>
    )
}
