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
    export let secret: string = ''
    export let asyncDiscard: (ability: Ability) => Promise<void>
    export let asyncDraw: () => Promise<void> = async () => {}
    export let asyncPlay: (ability: Ability) => Promise<void> = async () => {}
</script>

<div class="grid-row-4 grid gap-8 p-4">
    <div class="w-full justify-center">
        <Components.MightTable {might} />
    </div>
    <div class="w-full justify-center">
        <Components.Season {season} />
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
        {#each clocks as clock}
            <Components.Clock {clock} {secret} />
        {/each}
    </div>
</div>
