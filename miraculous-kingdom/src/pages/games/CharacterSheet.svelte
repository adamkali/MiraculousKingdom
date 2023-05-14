<script lang="ts">
    import { onMount } from 'svelte'
    import * as Components from '../../components'
    import FaCaretLeft from 'svelte-icons/fa/FaCaretLeft.svelte'
    import FaCaretRight from 'svelte-icons/fa/FaCaretRight.svelte'
    import GiCardDraw from 'svelte-icons/gi/GiCardDraw.svelte'
    import {
        MightEnum,
        type CharacterResponse,
        type GameInfo,
        type Ability,
        ApiCharacterApiService,
        type MightRequirement,
        RollTier,
    } from '../../models'
    import { currentGame, gameCharacter } from '../../store'
    import GiCardPlay from 'svelte-icons/gi/GiCardPlay.svelte'

    let game: GameInfo = currentGame.get();
    let character: CharacterResponse = gameCharacter.get();
    let hand: [Ability[]] = [] as unknown as [Ability[]]
    let handPage: number = 0
    let handMaxPage: number

    const onDiscard = (ability: Ability) => {}

    const asyncInit = async () => {
        console.log(character);
        if (!character.char_hand.length) {
            const res = await ApiCharacterApiService.initHand(
                character.secret,
                game.game_pass,
            )
            if (res.success === 'Succeeding') {
                gameCharacter.set(res.data)
                console.log(res.data)
                character = gameCharacter.get()

                const chunkSize = 5
                const numChunks = Math.ceil(character.char_hand.length / chunkSize)
                handMaxPage = numChunks
                hand.pop()

                for (let i = 0; i < numChunks; i++) {
                    const startIndex = i * chunkSize
                    const endIndex = startIndex + chunkSize
                    const chunk = character.char_deck.slice(startIndex, endIndex)
                    if (chunk.length !== 5) {
                        const remain = 5 - chunk.length
                        for (let i = 0; i < remain; i++) {
                            chunk.push({
                                ability_name: 'None',
                                ability_desc: '',
                                ability_clock: null,
                                ability_unlock: {
                                    might: MightEnum.CULTURE,
                                    unlock: 0,
                                    roll_tier: RollTier.NONE,
                                } as MightRequirement,
                            } as Ability)
                        }
                    }
                    hand.push(chunk)
                }
            } else {
                throw new Error(res.success.Failing.message)
            }

        }
    }

    const onDraw = () => {

    }

    const handlePageUp = () => {
        handPage = (handPage + 1 + handMaxPage) % handMaxPage
    }

    const handlePageDown = () => {
        handPage = (handPage - 1 + handMaxPage) % handMaxPage
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center rounded-xl bg-slate-600/70 px-12 pt-6 text-center align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 w-full rounded-xl bg-slate-800 p-8 text-5xl font-bold text-blue-600"
    >
        Miraculous Kingdom
    </div>
    {#await asyncInit()}
        <div class="grid w-full grid-rows-5 gap-y-4">
            <div class="flex h-24 flex-row items-center justify-center">
                <div class="mx-4">
                    <Components.Input
                        label="Name"
                        placeholder=""
                        value={character.char_name}
                        onChange={(_value) => {}}
                        inputType="text"
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Class"
                        placeholder=""
                        value={character.char_class}
                        onChange={(_value) => {}}
                        inputType="text"
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Country Name"
                        placeholder=""
                        value={game.game_name}
                        onChange={(_value) => {}}
                        inputType="text"
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Ruler"
                        placeholder=""
                        value={game.game_ruler}
                        onChange={(_value) => {}}
                        inputType="text"
                    />
                </div>
                <div class="mx-4 h-full">
                    <Components.Button
                        onClick={() => {
                            currentGame.set(null)
                            gameCharacter.set(null)
                            window.location.href = '/'
                        }}
                    >
                        <span>Exit</span>
                    </Components.Button>
                </div>
            </div>
            <div class="row-span-4 w-full justify-center">
                <Components.MightTable might={character.char_might} />
            </div>
            <div
                class="mt-8 grid h-[40rem] w-full grid-cols-7 items-center justify-items-center"
            >
                <div class="h-full w-full p-8">
                    <Components.Button onClick={handlePageDown}>
                        <div
                            class="flex h-24 justify-center text-3xl text-red-600"
                        >
                            <GiCardDraw />
                            Draw An Ability
                        </div>
                    </Components.Button>
                </div>
                {#each hand[handPage] as ability}
                    {#if ability.ability_name === 'None'}
                        <div class="mx-18 relative h-full w-full p-8" />
                    {:else}
                        <div class="mx-18 relative h-full w-full p-8">
                            <div
                                class="absolute right-4 top-4 z-10 h-8 w-8 rounded-full bg-slate-600 text-slate-200 transition duration-100 hover:scale-150"
                                on:click={() => onDiscard(ability)}
                            >
                                x
                            </div>
                            <Components.Ability {ability} />
                        </div>
                    {/if}
                {/each}
                <div class="h-1/3 w-1/3 place-items-center">
                    <Components.Button onClick={() => handlePageUp()}>
                        <FaCaretRight />
                    </Components.Button>
                </div>
            </div>
        </div>
    {:catch err}
        {err}
    {/await}
</div>
