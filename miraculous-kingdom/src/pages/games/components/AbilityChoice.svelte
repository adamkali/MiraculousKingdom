<script lang="ts">
    import GiCardDraw from 'svelte-icons/gi/GiCardDraw.svelte'
    import GiRollingDiceCup from 'svelte-icons/gi/GiRollingDiceCup.svelte'
    import * as Components from '../../../components'
    import {
        type Ability,
        ApiSeasonApiService,
        type Might,
        type SeasonResponse,
    } from '../../../models'

    export let hand: Ability[] = []
    export let season: SeasonResponse = {} as SeasonResponse
    export let might: Might = {} as Might
    export let asyncDiscard: (ability: Ability) => Promise<void>
    export let asyncDraw: () => Promise<void> = async () => {}
    export let asyncPlay: (ability: Ability) => Promise<void> = async () => {}
</script>

<div class="grid-row-4 grid">
    <div class="w-full justify-center">
        <Components.MightTable {might} />
    </div>
    <div class="flex w-full flex-row">
        <div class="group relative h-[20rem] w-[24rem]">
            <div
                class="lg absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
            />
            <div
                class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-black px-4 py-2 text-justify text-sm backdrop-blur"
            >
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
</div>
