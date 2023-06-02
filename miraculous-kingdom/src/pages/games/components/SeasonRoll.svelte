<script lang="ts">
    import GiCardDraw from 'svelte-icons/gi/GiCardDraw.svelte'
    import GiRollingDiceCup from 'svelte-icons/gi/GiRollingDiceCup.svelte'
    import * as Components from '../../../components'
    import { Clock, type Ability, type Might } from '../../../models'

    export let hand: Ability[] = []
    export let clocks: Clock[] = [] as Clock[]
    export let might: Might = {} as Might
    export let asyncDiscard: (ability: Ability) => Promise<void>
    export let secret: string = ""
    export let asyncRollSeason: () => Promise<void> = async () => {}
</script>

<div class="grid-row-3 grid gap-8 p-4">
    <div class="h-full w-full justify-center">
        <Components.MightTable {might} />
    </div>
    <div class="h-full w-full flex-row">
        <Components.Button
            onClick={async () => {
                await asyncRollSeason()
            }}
        >
            <div class="h-24 w-full p-4">
                <GiRollingDiceCup />
                Roll Season
            </div>
        </Components.Button>
    </div>
    <div class="flex h-full w-full flex-row">
        <div class="h-4/5 w-full items-center justify-center">
            <Components.Hand
                {hand}
                discard={asyncDiscard}
                send={async () => {}}
            />
        </div>
    </div>
    <div class="flex h-full w-full flex-row">
        {#each clocks as clock }
            <Components.Clock clock={clock} secret={secret}/>
        {/each}
    </div>
</div>
