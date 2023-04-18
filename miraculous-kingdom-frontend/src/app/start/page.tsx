import React from "react";
import StartGame from "./StartGame";
import JoinGame from "./JoinGame";

export default async function Game() {
    return (
        <div className="flex flex-row">
            <StartGame />
            {/* @ts-expect-error Server Component */}
            <JoinGame />
        </div>
   );
}
