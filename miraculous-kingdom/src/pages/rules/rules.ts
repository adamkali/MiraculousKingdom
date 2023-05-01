import {
    GiMuscleUp,
    GiPaintBrush,
    GiMaterialsScience,
    GiJerusalemCross,
    GiPapers,
    GiSpy,
} from "react-icons/gi";

export type Rule = {
    id: number;
    title: string;
    ele: JSX.Element;
    searchableName: string[];
    continue: JSX.Element;
};

//
//    {
//        id: n,
//        title: "",
//        ele: (
//            <div className="flex flex-col justify-center items-center mx-8 pb-8 text-xl"> <div className="text-2xl text-blue-500">
//            <div className="flex flex-row w-full justify-start">
//                |doing|
//            </div>
//            <div className="flex flex-row w-full justify-start">
//                |doing|
//            </div>
//        ),
//        searchableName: [""],
//        continue: (
//           <div
//                className="flex flex-col justify-center items-center mx-8 pb-8 text-xl"
//           >
//           </div>
//        )
//    }

const RulesConst: Rule[] = [
    {
        id: 0,
        title: "Starting a Game",
        ele: (
            <Link
                href="/rules/0"
                className="mx-8 flex flex-col items-center justify-center pb-8 text-xl"
            >
                <div className="w-2/3 justify-center">
                    Starting a Game is a very simple process. One of the players
                    presses <b className="text-red-700">Start Game</b> in the
                    game menu. From there they will get a code like{" "}
                    <b className="text-red-700">0000AAA</b>. This will be what
                    you put in when you click{" "}
                    <b className="text-red-700">Join</b>. It will then ask for a
                    passcode...
                </div>
            </Link>
        ),
        searchableName: ["Set Up", "Setting Up", "Start"],
        continue: (
            <div className="mx-8 flex flex-col items-center justify-center pb-8 text-xl">
                <div className="w-2/3">
                    You should take a look at
                    <Link className="text-blue-700" href="/rules/1">
                        Character Creation
                    </Link>
                    To find out how to create a character.
                </div>
            </div>
        ),
    },
    {
        id: 1,
        title: "Character Creation",
        ele: (
            <Link
                href="/rules/1"
                className="mx-8 flex flex-col items-center justify-center pb-8 text-xl"
            >
                <div className="w-2/3">
                    <div className="pb-4">
                        You will enter in a name, and choose a{" "}
                        <Link href="/rule/2" className="text-red-600">
                            Class
                        </Link>
                    </div>
                    <div className="pb-4">
                        Each character must roll a 6-sided die for each of the
                        six stats.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiMuscleUp className="mr-2 text-blue-700" />
                        Military Might,
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiJerusalemCross className="mr-2 text-blue-700" />
                        Religious Might,
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiMaterialsScience className="mr-2 text-blue-700" />
                        Scientific Might,
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiPaintBrush className="mr-2 text-blue-700" />
                        Cultural Might,
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiPapers className="mr-2 text-blue-700" />
                        Diplomatic Might,
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        <GiSpy className="mr-2 text-blue-700" />
                        Espionage Might...
                    </div>
                </div>
            </Link>
        ),
        searchableName: [
            "Creation",
            "Characters",
            "Create",
            "Character",
            "Might",
        ],
        continue: (
            <div className="mx-8 flex flex-col items-center justify-center pb-8 text-xl">
                <div className="w-2/3">
                    When you are finished press save and make sure to save a
                    password. This will be needed for you to get your Characters
                    later.
                </div>
            </div>
        ),
    },
    {
        id: 2,
        title: "Classes",
        ele: (
            <Link
                href="/rules/2"
                className="mx-8 flex flex-col items-center justify-center pb-8 text-xl"
            >
                <div className="w-2/3">
                    There are 6 classes made for this proof of concept:
                    <div className="flex w-full flex-row justify-start">
                        The War General: Focus on Combat abilities.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        The Cultural Aficianado: Focus on using Tokens and
                        resource managment.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        The Royal Scientist: Focus on counters to another
                        players abilities.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        The Cardinal: Focus on using clocks to subjugate other
                        players.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        The Spy Master: Focus on sabatoge tokens and strategems.
                    </div>
                    <div className="flex w-full flex-row justify-start">
                        The Diplomat: Use this class' passive to chain abilties.
                    </div>
                </div>
            </Link>
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
        continue: (
            <div className="mx-8 flex flex-col items-center justify-center pb-8 text-xl">
                <div className="w-2/3">
                    Every Class has its own abilites that can be used during the
                    game to allow you to win.
                </div>
            </div>
        ),
    },
];

export { RulesConst };
