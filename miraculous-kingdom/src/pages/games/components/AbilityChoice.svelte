<script lang="ts">
    import GiCardDraw from 'svelte-icons/gi/GiCardDraw.svelte'
    import GiRollingDiceCup from 'svelte-icons/gi/GiRollingDiceCup.svelte'
    import * as Components from '../../../components'
    import {
        type Ability,
        ApiSeasonApiService,
        type Might,
        type SeasonResponse,
        type MightStat,
        type Clock,
    } from '../../../models'

    export let hand: Ability[] = []
    export let season: SeasonResponse = {} as SeasonResponse
    export let might: Might = {} as Might
    export let clocks: Clock[] = [] as Clock[]
    export let secret: string = ""
    export let asyncDiscard: (ability: Ability) => Promise<void>
    export let asyncDraw: () => Promise<void> = async () => {}
    export let asyncPlay: (ability: Ability) => Promise<void> = async () => {}


    // type gaurd for season.event_reward 
    // should be either Ability, MightStat, Clock
    function isAbility(ability: any): ability is Ability {
        return ability.type === 'Ability'
    } 

    function isMight(might: any): might is MightStat {
        return might.type === 'MightStat'
    }

    function isClock(clock: any): clock is Clock {
        return clock.type === 'Clock'
    }

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
                <div class="flex flex-col w-1/2 ml-4">
                    <div class="text-2xl font-bold text-blue-400">
                        {season.event_name}
                    </div>
                    <p class="text-xl font-bold text-slate-400">
                        {season.event_desc}
                    </p>
                    <div class="place-item-left text-xl font-bold text-slate-400">
                        {season.event_length}
                    </div>
                </div>
                {#if season.event_reward !== "None"
                && isAbility(season.event_reward)}
                    <div class="flex flex-col w-1/2">
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
                {:else if season.event_reward !== "None"
                && isMight(season.event_reward)}
                    <div class="flex flex-col w-1/2">
                        <div class="text-2xl font-bold text-blue-400">
                            Reward: Experience
                        </div>
                        <p class="text-xl font-bold text-slate-400">
                            {season.event_reward.stat_name} + {season.event_reward.stat_exp}
                        </p>
                    </div>
                {:else if season.event_reward !== "None"
                && isClock(season.event_reward)}
                    <div class="flex flex-col w-1/2">
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
                    <div class="flex flex-col w-1/2">
                        <div class="text-2xl font-bold text-blue-400">
                            No Reward
                        </div>
                    </div>
                {/if}
            </div>
        </div>
    </div>
    <div class="flex w-full flex-row">
        <div class="h-4/5 w-full items-center justify-center">
            <Components.Hand {hand} discard={asyncDiscard} send={asyncPlay} />
        </div>
    </div>
    <div
        class="my-auto ml-16 h-[20rem] w-[24rem] items-center align-middle text-red-600"
    >
        <Components.Button
            onClick={async () => {
                await asyncDraw()
            }}
        >
            <div class="h-24">
                <GiCardDraw />
            </div>
            <span>Draw</span>
        </Components.Button>
    </div>
    <div class="flex h-full w-full flex-row">
        {#each clocks as clock }
            <Components.Clock clock={clock} secret={secret}/>
        {/each}
    </div>
</div>
