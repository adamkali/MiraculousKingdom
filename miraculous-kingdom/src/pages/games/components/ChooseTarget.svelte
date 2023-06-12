<script lang="ts">
    import { 
        type Might, 
        type SeasonResponse as Season 
    } from "../../../models";
    import * as Components from "../../../components";

    export let might: Might
    export let season: Season
</script>


<div class="grid-row-4 grid gap-8 p-4">
    <div class="w-full justify-center">
        <Components.MightTable {might} />
    </div>
    <div class="flex w-full flex-row">
        <div class="group relative">
            <div
                class="lg absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
            />
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
                {#if season.event_reward !== 'None' && isAbility(season.event_reward)}
                    <div class="flex w-1/2 flex-col">
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Ability
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.ability_name}
                        </p>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.ability_desc}
                        </p>
                    </div>
                {:else if season.event_reward !== 'None' && isMight(season.event_reward)}
                    <div class="flex w-1/2 flex-col">
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Experience
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.stat_name} + {season
                                .event_reward.stat_exp}
                        </p>
                    </div>
                {:else if season.event_reward !== 'None' && isClock(season.event_reward)}
                    <div class="flex w-1/2 flex-col">
                        <div class="text-2xl font-bold text-blue-400">
                            Reward:
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.clock_name}
                        </p>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.clock_desc}
                        </p>
                    </div>
                {:else}
                    <div class="flex w-1/2 flex-col">
                        <div class="text-2xl font-bold text-blue-400">
                            No Reward
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
