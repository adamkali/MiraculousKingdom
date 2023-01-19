<script lang="ts">
    import TiArrowForward from 'svelte-icons/ti/TiArrowForward.svelte'
</script>

<div>
    <div class="width-full flex h-48 justify-center pt-10 pb-0">
        <h1 class="flex justify-center text-3xl font-bold">
            Miraculous Kingdom
        </h1>
    </div>
    <div class="width-full flex justify-center py-0 px-16 text-xl">
        <p>
            The Miraculous Kingdom is a <b>TTRPG: Table Top Role Playing Game</b
            > designed behind subtrafuge, lies, and backstabbing. But more importantly,
            this is a game about having fun! The attide should be as follows:
        </p>
    </div>
    <div class="width-full flex-col justify-center py-12 px-24 text-lg">
        <ul>
            <li class="pb-4 no-underline">
                => Your friend backstabs you during an important roll.
            </li>
            <li class="pb-4 no-underline">
                => You pop off in game and sabotage your friend back!
            </li>
            <li class="pb-4 no-underline">
                => In reality, you are having fun because their character acted
                in game not your friend backstabbing in real life.
            </li>
        </ul>
    </div>
    <div class="width-full mb-24 flex justify-center px-16 text-xl">
        <p>
            At the end of the day yu are making a story for you and your
            friends. When playing the game, always frame what you are trying to
            accomplish, not what you are trying to do. This game is not made to
            be played with a game master, so you and your friends are in charge
            of coming to an agreement of what rewards should be dished out.
        </p>
    </div>
    <div class="width-full flex justify-center text-lg font-bold">
        <div
            class="grid h-24 grid-cols-2 
            gap-4 rounded-xl bg-indigo-800 py-2 
            px-2 transition duration-150 ease-out
            hover:scale-110 hover:bg-indigo-600 
            hover:text-cyan-400 hover:ease-in"
        >
            <div class="flex flex-col justify-center">
                <span class="-pr-10 pl-8 text-xl">Get Started</span>
            </div>
            <TiArrowForward />
        </div>
    </div>
</div>
