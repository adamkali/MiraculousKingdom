// external
import React from "react";
import Link from "next/link";

// internal
import { GameInfo, GameAPI } from "../../../index";

const getGames = async (): Promise<GameInfo[]> => {
    let response = await GameAPI.getAll();
    if (response.success != "Succeeding") {
        throw new Error(response.success.Failing.message);
    }
    return response.data;
};

export default async function JoinGame() {

    const games = await getGames();

    const onClick = (game_pass: string) => {
        location.href = `/app/games/create_character/${game_pass}`;
    };

    return (
        <div className="flex flex-col">
            {games.length > 0 ? (
                games.map((game) => {
                    return (
                        <div
                            onClick={() => {
                                onClick(game.game_pass);
                            }}
                            className="grid grid-rows-2 rounded-lg bg-slate-400 text-slate-600"
                        >
                            <div className="grid-cols-2">
                                <div className="flex flex-row justify-center">
                                    <div className="text-red-600">
                                        Country:{" "}
                                    </div>
                                    {game.game_name}
                                </div>
                                <div className="flex flex-row justify-center">
                                    <div className="text-red-600">Ruler: </div>
                                    {game.game_ruler}
                                </div>
                            </div>
                            <div className="flex flex-col justify-start">
                                <div className="text-red-600">Characters: </div>
                                {game.game_chars.map((name: string) => {
                                    return <div>{name}</div>;
                                })}
                            </div>
                        </div>
                    );
                })
            ) : (
                <div className="flex flex-col text-justify">
                    <div className="text-3xl text-rose-600">Oops!</div>
                    <p className="text-sky-600">
                        There are no games you can join. Make sure: You have
                        started a{" "}
                        <Link className="text-sky-600" href="/app/start/create">
                            Game
                        </Link>
                    </p>
                </div>
            )}
        </div>
    );
}
