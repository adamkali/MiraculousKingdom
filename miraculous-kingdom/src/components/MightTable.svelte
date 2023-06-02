<script lang="ts">
    import type { Might } from '../models'
    import GiDiceTwentyFacesTwenty from 'svelte-icons/gi/GiDiceTwentyFacesTwenty.svelte'
    import GiWinchesterRifle from 'svelte-icons/gi/GiWinchesterRifle.svelte'
    import GiFleurDeLys from 'svelte-icons/gi/GiFleurDeLys.svelte'
    import GiMaterialsScience from 'svelte-icons/gi/GiMaterialsScience.svelte'
    import GiCrossShield from 'svelte-icons/gi/GiCrossShield.svelte'
    import GiSecretBook from 'svelte-icons/gi/GiSecretBook.svelte'
    import GiSpy from 'svelte-icons/gi/GiSpy.svelte'

    export let might: Might
    export let changeMight: (might: Might) => void = () => {}
</script>

<div class="group relative h-full w-full">
    <div
        class="absolute -inset-0.5 h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
    />
    <table
        class="relative w-full table-auto border-spacing-8 items-center rounded-lg bg-black px-8 py-4 text-xl leading-none text-slate-300"
    >
        <div
            class="absolute -inset-0.5 h-full w-full rounded bg-gradient-to-r from-fuchsia-600 to-blue-600 opacity-75 blur"
        />
        <thead
            class="w-full rounded border border-amber-50/0 bg-black px-4 pt-2"
        >
            <tr
                class="relative h-full w-full bg-black text-3xl text-fuchsia-600"
            >
                <th
                    class="flex h-16 flex-col justify-center border border-amber-50/0 text-blue-700"
                >
                    <img
                        class="h-16 p-4"
                        src="../public/mk.svg"
                        alt="Miraculous Kingdom"
                    />
                </th>
                <th class="h-16 border border-amber-50/0"> MIGHT </th>
                <th class="h-16 border border-amber-50/0"> CHANGE EXP </th>
                <th class="h-16 border border-amber-50/0"> EXP </th>
                <th class="h-16 border border-amber-50/0"> CHANGE TOKEN </th>
                <th class="h-16 border border-amber-50/0"> MIGHT TOKEN </th>
            </tr>
        </thead>
        <tbody class="w-full border border-amber-50/0 bg-black px-4 pt-2">
            <tr
                class="relative h-full w-full border-x border-white/0 bg-black text-2xl"
            >
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-red-600">
                        <GiWinchesterRifle />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_military.stat_enum}</span>
                        <span>{might.might_military.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_military.stat_exp =
                                might.might_military.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_military.stat_exp =
                                might.might_military.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_military.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_military.stat_token =
                                might.might_military.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_military.stat_token =
                                might.might_military.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_military.stat_token}
                </td>
            </tr>
            <tr
                class="relative h-full w-full border-x border-white/0 bg-black text-2xl"
            >
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-pink-600">
                        <GiFleurDeLys />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_culture.stat_enum}</span>
                        <span>{might.might_culture.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_culture.stat_exp =
                                might.might_culture.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_culture.stat_exp =
                                might.might_culture.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_culture.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_culture.stat_token =
                                might.might_culture.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_culture.stat_token =
                                might.might_culture.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_culture.stat_token}
                </td>
            </tr>
            <tr
                class="relative h-full w-full border-x border-white/0 bg-black text-2xl"
            >
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-blue-600">
                        <GiMaterialsScience />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_science.stat_enum}</span>
                        <span>{might.might_science.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_science.stat_exp =
                                might.might_science.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_science.stat_exp =
                                might.might_science.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_science.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_science.stat_token =
                                might.might_science.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_science.stat_token =
                                might.might_science.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_science.stat_token}
                </td>
            </tr>

            <tr
                class="relative h-full w-full border border-white/0 bg-black text-2xl"
            >
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-cyan-400">
                        <GiCrossShield />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_religion.stat_enum}</span>
                        <span>{might.might_religion.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_religion.stat_exp =
                                might.might_religion.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_religion.stat_exp =
                                might.might_religion.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_religion.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_religion.stat_token =
                                might.might_religion.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_religion.stat_token =
                                might.might_religion.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_religion.stat_token}
                </td>
            </tr>
            <tr class="relative h-full w-full bg-black text-2xl">
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-yellow-400">
                        <GiSecretBook />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_diplomacy.stat_enum}</span>
                        <span>{might.might_diplomacy.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_diplomacy.stat_exp =
                                might.might_diplomacy.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_diplomacy.stat_exp =
                                might.might_diplomacy.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_diplomacy.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_diplomacy.stat_token =
                                might.might_diplomacy.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_diplomacy.stat_token =
                                might.might_diplomacy.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_diplomacy.stat_token}
                </td>
            </tr>
            <tr class="relative h-full w-full bg-black text-2xl">
                <td class="border-x border-white/0">
                    <button class="h-16 bg-black text-emerald-500">
                        <GiSpy />
                    </button>
                </td>
                <td class="border-x border-white/0">
                    <div class="divide divide-white/0">
                        <span>{might.might_espionage.stat_enum}</span>
                        <span>{might.might_espionage.stat_value}</span>
                    </div>
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_espionage.stat_exp =
                                might.might_espionage.stat_exp + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_espionage.stat_exp =
                                might.might_espionage.stat_exp - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_espionage.stat_exp}
                </td>
                <td class="border-x border-white/0">
                    <button
                        on:click={() => {
                            might.might_espionage.stat_token =
                                might.might_espionage.stat_token + 1
                            changeMight(might)
                        }}
                    >
                        +
                    </button>
                    <button
                        on:click={() => {
                            might.might_espionage.stat_token =
                                might.might_espionage.stat_token - 1
                            changeMight(might)
                        }}
                    >
                        -
                    </button>
                </td>
                <td class="border-x border-white/0">
                    {might.might_espionage.stat_token}
                </td>
            </tr>
        </tbody>
    </table>
</div>
