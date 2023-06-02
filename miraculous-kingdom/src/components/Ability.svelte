<script lang="ts">
    import { type Ability, MightEnum, RollTier } from '../models'
    import GiWinchesterRifle from 'svelte-icons/gi/GiWinchesterRifle.svelte'
    import GiFleurDeLys from 'svelte-icons/gi/GiFleurDeLys.svelte'
    import GiMaterialsScience from 'svelte-icons/gi/GiMaterialsScience.svelte'
    import GiCrossShield from 'svelte-icons/gi/GiCrossShield.svelte'
    import GiSecretBook from 'svelte-icons/gi/GiSecretBook.svelte'
    import GiSpy from 'svelte-icons/gi/GiSpy.svelte'
    import FaVolumeMute from 'svelte-icons/fa/FaVolumeMute.svelte'

    export let ability: Ability
    export let send: (a: Ability) => Promise<void>
</script>

<div class="group relative h-[20rem] w-[24rem]">
    <div
        class="lg absolute h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur transition duration-150 ease-in-out group-hover:from-fuchsia-500 group-hover:to-blue-500 group-hover:opacity-100 group-hover:blur-xl"
    />
    <div
        class="mx-2 flex h-full w-full flex-col justify-evenly rounded-lg bg-slate-300 dark:bg-black px-4 py-2 text-justify text-sm backdrop-blur"
        on:click={async () => await send(ability)}
    >
        <div class="mb-2 text-2xl text-fuchsia-600">
            {ability.ability_name}
        </div>
        <p class="mb-2">
            {ability.ability_desc}
        </p>
        {#if ability.ability_clock}
            <p>
                <b>{ability.ability_clock.clock_name}</b>
                {ability.ability_clock.clock_desc}
            </p>
            <div
                class="flex h-10 flex-row place-items-start justify-start text-lg text-blue-600"
            >
                {#if ability.ability_clock.clock_conf}
                    <FaVolumeMute />
                {/if}
                {' ' +
                    ability.ability_clock.clock_remaining +
                    '/' +
                    ability.ability_clock.clock_duration}
            </div>
        {/if}
        <div class="place-items-end">
            <div class="items flex flex-row text-lg">
                {#if ability.ability_unlock.might == MightEnum.MILITARY}
                    <span class="h-10 text-red-600"><GiWinchesterRifle /></span>
                {:else if ability.ability_unlock.might == MightEnum.CULTURE}
                    <span class="h-10 text-purple-600"><GiFleurDeLys /></span>
                {:else if ability.ability_unlock.might == MightEnum.RELIGION}
                    <span class="h-10 text-cyan-400"><GiCrossShield /></span>
                {:else if ability.ability_unlock.might == MightEnum.SCIENCE}
                    <span class="h-10 text-blue-600"
                        ><GiMaterialsScience /></span
                    >
                {:else if ability.ability_unlock.might == MightEnum.DIPLOMACY}
                    <span class="h-10 text-yellow-400"><GiSecretBook /></span>
                {:else if ability.ability_unlock.might == MightEnum.ESPIONAGE}
                    <span class="h-10 text-emerald-500"><GiSpy /></span>
                {/if}
                {ability.ability_unlock.unlock}
            </div>
            {#if ability.ability_unlock.roll_tier === RollTier.FAIL}
                <div
                    class="place-items-end rounded-xl bg-white px-2 py-1 text-center text-black"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else if ability.ability_unlock.roll_tier === RollTier.BAD}
                <div
                    class="place-items-end rounded-xl bg-green-500 px-2 py-1 text-center text-black"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else if ability.ability_unlock.roll_tier === RollTier.NEUTRAL}
                <div
                    class="w-full place-items-end rounded-xl bg-blue-500 px-2 py-1 text-center text-black"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else if ability.ability_unlock.roll_tier === RollTier.GOOD}
                <div
                    class="w-full place-items-end rounded-xl bg-purple-700 px-2 py-1 text-center text-white"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else if ability.ability_unlock.roll_tier === RollTier.GREAT}
                <div
                    class="w-full place-items-end rounded-xl bg-orange-600 px-2 py-1 text-center text-yellow-200"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else if ability.ability_unlock.roll_tier === RollTier.FANTASTIC}
                <div
                    class="w-96 place-items-end rounded-xl bg-red-600 px-2 py-1 text-center text-blue-300"
                >
                    {ability.ability_unlock.roll_tier}
                </div>
            {:else}
                <div>{' '}</div>
            {/if}
        </div>
    </div>
</div>
