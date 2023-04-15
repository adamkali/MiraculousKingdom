import React from "react";

export default function RulesLasout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className="mx-96 mt-24 h-2/3 rounded-3xl bg-slate-700/40 p-8 backdrop-blur-lg">
            {children}
        </div>
    );
}
