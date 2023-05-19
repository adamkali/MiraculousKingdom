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
        ClassEnum,
    } from '../../models'
    import { currentGame, gameCharacter } from '../../store'
    import GiCardPlay from 'svelte-icons/gi/GiCardPlay.svelte'
    import Wrapper from '../../components/Wrapper.svelte'

    let game: GameInfo = currentGame.get()
    let character: CharacterResponse = gameCharacter.get()
    let hand: Ability[] = [] as Ability[]

    const asyncDiscard = (ability: Ability) => {}

    const asyncInit = async () => {
        if (!character.char_hand.length) {
            const res = await ApiCharacterApiService.initHand(
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
        hand = character.char_hand
    }

    const asyncDraw = async () => {
        const drawAmount = character.char_class === ClassEnum.SCIENTIST ? 2 : 1
        const res = await ApiCharacterApiService.drawCard(
            character.secret,
            game.game_pass,
            drawAmount,
        )
        if (res.success === 'Succeeding') {
            gameCharacter.set(res.data)
            character = gameCharacter.get()
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
            <div class="h-4/5 w-full justify-center">
                <Components.MightTable might={character.char_might} />
            </div>
            <div class="h-1/5 w-full flex flex-row">
            </div>
            <div class="h-4/5 w-full justify-center items-center">
                <Components.Hand hand={hand} />
            </div>
        </div>
    {:catch err}
        {err}
    {/await}
</div>
