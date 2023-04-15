"use client";
import React, { useState, useEffect } from "react";

import { BsSearch } from "react-icons/bs";
import {
    GiMuscleUp,
    GiPaintBrush,
    GiMaterialsScience,
    GiJerusalemCross,
    GiPapers,
    GiSpy,
} from "react-icons/gi";

type Rule = { ele: Node; searchableName: string[] };

export default function Rules() {
    const rulesConst = [
        {
            ele: () => (
                <div className="flex flex-row text-xl">
                    <a className="mr-16 text-blue-500">Starting a Game</a>
                    <p>
                        Starting a Game is a very simple process. One of the
                        players presses{" "}
                        <b className="text-red-700">Start Game</b> in the game
                        menu. From there they will get a code like{" "}
                        <b className="text-red-700">0000AAA</b>. This will be
                        what you put in when you click{" "}
                        <b className="text-red-700">Join</b>. It will then ask
                        for a passcode...
                    </p>
                </div>
            ),
            searchableName: ["Set Up", "Setting Up", "Start"],
        },
        {
            ele: () => (
                <div className="flex flex-row text-xl">
                    <a className="mr-16 text-blue-500">Character Creation</a>
                    <div>
                        Each character must roll a 6-sided die for each of the
                        six stats:
                        <GiMuscleUp className="mr-2" />
                        Military Might,
                        <GiJerusalemCross className="mr-2" />
                        Religious Might,
                        <GiMaterialsScience className="mr-2" />
                        Scientific Might,
                        <GiPaintBrush className="mr-2" />
                        Cultural Might,
                        <GiPapers className="mr-2" />
                        Diplomatic Might, and <GiSpy className="mr-2" />
                        Espionage Might...
                    </div>
                </div>
            ),
            searchableName: ["Creation", "Characters", "Create", "Character"],
        },
        {
            ele: () => (
                <div className="flex flex-row text-xl">
                    <a className="mr-16 text-blue-500">Classes:</a>
                    <a>
                        There are 6 classes made for this proof of concept: The
                        War General, The Cultural Aficianado, The Royal
                        Scientist, The Cardinal, The Spy Master, and The
                        Diplomat.
                    </a>
                </div>
            ),
            searchableName: [
                "Classes",
                "War General",
                "Cultural Aficianado",
                "Royal Scientist",
                "Cardinal",
                "Spy Master",
                "Diplomat",
            ],
        },
    ];

    const [ruleSearch, setRuleSearch] = useState("");
    const [rules, setRules] = useState(rulesConst);

    useEffect(() => {
        const newRules: Rule[] = [];
        if (ruleSearch !== "") {
            rules.forEach((rule) => {
                rule.searchableName.forEach((name) => {
                    if (name.toLowerCase().includes(ruleSearch.toLowerCase())) {
                        let temp = {
                            node: rule.ele,
                            searchableName: rule.searchableName,
                        } as unknown;
                        newRules.push(temp as Rule);
                    }
                });
            });
            setRules(newRules as any);
        }
    }, [ruleSearch]);

    return (
        <div className="flex flex-col justify-center overflow-scroll">
            <div className="py-8 text-6xl text-red-700">Rules</div>
            <div className="w-2/3 rounded-full bg-slate-700 p-4 text-blue-500">
                <BsSearch />
                <input
                    className="bg-slate0700 text-xl outline-none"
                    value={ruleSearch}
                    onChange={(e) => setRuleSearch(e.currentTarget.value)}
                />
            </div>
            <div className="flex flex-col overflow-y-auto overflow-x-hidden text-center text-lg">
                {rules.map((node) => {
                    return node.ele();
                })}
            </div>
        </div>
    );
}
