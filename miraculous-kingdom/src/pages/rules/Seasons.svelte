<script lang="ts">
    import { type Experience, MightEnum, type SeasonResponse } from "../../models";


   let season = {
        event_name: "Season of the Witch",
        event_desc: "The Season of the Witch is a time of magic and mystery. During this season, players can use their magic to cast spells and enchantments. The Season of the Witch lasts for 3 turns.",
        event_length: 3,
        event_reward: {
            Experience: {
                exp_amount: 2,
                exp_type: MightEnum.MILITARY
            } as Experience
        }
    } as SeasonResponse;
</script>

<div
    class="flex min-h-[90rem] w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-justify align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 flex flex-col rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        <h1 class="text-center">Seasons</h1>
    </div>
    <div class="rounded-xl bg-black p-16">
        During a game of Miraculous Kingdom, the players will experience many Seasons. Each Season has there own special mecanics to them. Here is at example of one:
    </div>

    <div
        class="mx-2 flex h-full w-full flex-row justify-evenly rounded-lg bg-black px-4 py-2 text-justify text-sm backdrop-blur"
    >
        <div class="ml-4 flex w-1/2 flex-col">
            <div class="text-2xl font-bold text-blue-400">
                {season.event_name}
            </div>
            <p class="text-xl font-bold text-slate-400">
                {season.event_desc}
            </p>
            <div
                class="place-item-left text-xl font-bold text-slate-400"
            >
                {season.event_length}
            </div>
        </div>
    </div>
</div>
