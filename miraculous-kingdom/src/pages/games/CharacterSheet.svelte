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
        ClassEnum,
        type QueueResonse,
        ApiSeasonApiService,
        ApiQueueApiService,
        type TurnRequest,
    } from '../../models'
    import { currentGame, gameCharacter, queue } from '../../store'
    import GiCardPlay from 'svelte-icons/gi/GiCardPlay.svelte'
    import Wrapper from '../../components/Wrapper.svelte'
    import SeasonRoll from './components/SeasonRoll.svelte'
    import AbilityChoice from './components/AbilityChoice.svelte'

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    let queueres: QueueResonse = queue.get()
    $: hand = character.char_hand
    $: might = character.char_might
    //$: clocks = character.char_clock
    $: season = queueres.season

    const asyncDiscard = async (ability: Ability) => {
        const res = await ApiCharacterApiService.discardCard(
            character.secret,
            game.game_pass,
            ability,
        )
        if (res.success === 'Succeeding') {
            gameCharacter.set(res.data)
            character = gameCharacter.get()
        } else {
            throw new Error(res.success.Failing.message)
        }
    }

    const asyncInit = async () => {
        let q_res = await ApiQueueApiService.getQueue(game.game_pass)
        if (q_res.success === 'Succeeding') {
            queue.set(q_res.data)
            queueres = queue.get()
        } else {
            throw new Error(q_res.success.Failing.message)
        }
        if (!character.char_hand.length) {
            let res = await ApiCharacterApiService.getCharacterForGame(
                character.secret,
                game.game_pass,
            )
            if (
                res.success === 'Succeeding' &&
                res.data.char_hand.length === 0
            ) {
                res = await ApiCharacterApiService.initHand(
                    character.secret,
                    game.game_pass,
                )
                if (res.success === 'Succeeding') {
                    gameCharacter.set(res.data)
                    character = gameCharacter.get()
                } else {
                    throw new Error(res.success.Failing.message)
                }
            } else if (res.success === 'Succeeding') {
                gameCharacter.set(res.data)
                character = gameCharacter.get()
            } else {
                throw new Error(res.success.Failing.message)
            }
        }
        hand = gameCharacter.get().char_hand
    }

    const asyncDraw = async () => {
        const drawAmount = character.char_class === ClassEnum.SCIENTIST ? 2 : 1
        const res = await ApiCharacterApiService.drawCard(
            drawAmount,
            character.secret,
            game.game_pass,
        )
        if (res.success === 'Succeeding') {
            gameCharacter.set(res.data)
            character = gameCharacter.get()
        } else {
            throw new Error(res.success.Failing.message)
        }
    }

    const asyncRollSeason = async () => {
        const res = await ApiSeasonApiService.roll()
        if (res.success === 'Succeeding') {
            const res1 = await ApiQueueApiService.setSeason(game.game_pass, res.data)
            if (res1.success === 'Succeeding') {
                queue.set(res1.data)
                queueres = queue.get()
            } else {
                throw new Error(res1.success.Failing.message)
            }
        } else {
            throw new Error(res.success.Failing.message)
        }
    }

    const take_turn = async (ability: Ability) => {
        const res = await ApiQueueApiService.takeTurn(game.game_pass, {
            character: character,
            game: game.game_pass,
            ability: ability,
            initiatve: 0,
        } as TurnRequest)
        if (res.success === 'Succeeding') {
            queue.set(res.data)
            // find the character in the queue with the same secret as the character
            queue.get().queue.forEach((a) => {
                if (a.queue_char.secret === character.secret) {
                    gameCharacter.set(a.queue_char) 
                }
            })
            queueres = queue.get() 
        } else {
            throw new Error(res.success.Failing.message)
        }
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
        <div>...waiting</div>
    {:then}
        <div class="flex w-full flex-col">
            <div class="mb-8 flex h-24 flex-row items-center justify-center">
                <div class="mx-4">
                    <Components.Input
                        label="Name"
                        placeholder=""
                        value={character.char_name}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Class"
                        placeholder=""
                        value={character.char_class}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Country Name"
                        placeholder=""
                        value={game.game_name}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
                    />
                </div>
                <div class="mx-4">
                    <Components.Input
                        label="Ruler"
                        placeholder=""
                        value={game.game_ruler}
                        onChange={(_value) => {}}
                        inputType="text"
                        disabled={true}
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
            {#if season.event_name === ''}
                <SeasonRoll
                    {hand}
                    {might}
                    asyncDiscard={async () => {}}
                    {asyncRollSeason}
                />
            {:else}
                <AbilityChoice
                    {hand}
                    {season}
                    {might}
                    {asyncDiscard}
                    asyncPlay={take_turn}
                    {asyncDraw}
                />
            {/if}
        </div>
    {:catch err}
        {err}
    {/await}
</div>
