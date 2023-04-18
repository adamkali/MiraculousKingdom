"use client";
import React, { useState, useEffect } from "react";

import { Rule, RulesConst } from "../rules";


export default function Rules({
    params
}: {
    params: { id: string }
}) {

    const [rule, setRule] = useState({} as Rule)

    useEffect(() => {
        setRule(RulesConst.find((rule) => {
            return rule.id === +params.id;
        })!);
    }, []);

    return (
        <div className="flex max-h-fit flex-col items-center justify-center overflow-y-scroll">
            <div className="py-8 text-6xl text-red-700">{rule.title}</div>
            <div className="flex flex-col overflow-y-auto overflow-x-hidden text-center text-lg">
                <div className="flex flex-col justify-center mx-8 pb-8 text-xl">
                    {rule.ele}
                    {rule.continue}
                </div>
            </div>
        </div>
    );
}
