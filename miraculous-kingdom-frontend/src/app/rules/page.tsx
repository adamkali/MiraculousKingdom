"use client";
import React, { useState, useEffect } from "react";
import { BsSearch } from "react-icons/bs";

import { Rule, RulesConst } from "./rules";
import { rule } from "postcss";


export default function Page() {

    const [ruleSearch, setRuleSearch] = useState("");
    const [rules, setRules] = useState([] as Rule[]);

    useEffect(() => {
        if (ruleSearch !== "") {
            const newRules: Rule[] = [];
            RulesConst.forEach((rule) => {
                let arr: String[] = rule.searchableName;
                let flag: boolean = false;
                let item: number = 0;
                while (!flag) {
                    flag = arr[item].match(ruleSearch) !== null;
                    ++item;
                    if (item >= arr.length) {
                        break;
                    }
                }
                if (flag) {
                    newRules.push(rule);
                }
            });
            if (newRules.length === 0) {
                setRules(RulesConst as any);
                return;
            }
            setRules(newRules as any);
        } else {
            setRules(RulesConst as any);
        }
    }, [ruleSearch]);


    return (
        <div className="flex max-h-fit flex-col items-center justify-center overflow-y-scroll">
            <div className="py-8 text-6xl text-red-700">Rules</div>
            <div className="mb-8 flex w-2/3 flex-row items-center justify-start rounded-full bg-slate-700 p-4 text-xl text-blue-500">
                <BsSearch className="mr-8" />
                <input
                    placeholder="Search for a topic."
                    className="w-11/12 bg-slate-700 outline-none"
                    value={ruleSearch}
                    onChange={(e) => setRuleSearch(e.currentTarget.value)}
                />
            </div>
            <div className="flex flex-col overflow-y-auto overflow-x-hidden text-center text-lg">
                {rules.sort((rule0, rule1) => {
                    return rule0.title.localeCompare(rule1.title);
                }).map((node) => {
                    return (
                        <div className="flex flex-col justify-center mx-8 mb-4 pb-8 text-xl rounded-xl bg-slate-500/50">
                            <div className="text-2xl text-blue-500">
                                {node.title}
                            </div>
                            {node.ele}
                        </div>
                    );
                })}
            </div>
        </div>
    );
}
