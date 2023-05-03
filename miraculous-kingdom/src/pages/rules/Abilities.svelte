<script lang="ts">
    import Ability from '../../components/Ability.svelte'
    import Special from '../../components/Special.svelte'
    import {
        MightEnum,
        type Clock,
        type MightRequirement,
        RollTier,
        type Ability as AbilityType,
    } from '../../models'

    export const abilityConst: AbilityType = {
        ability_name: 'A. Example Name',
        ability_desc: 'B. This is the description of the example',
        ability_clock: {
            clock_conf: true,
            clock_name: 'C. This is always the same as the ability name',
            clock_desc:
                'D. This will be a part of the description detailing what happens or must happen for the ability to take effect or what happens when it takes effect.',
            clock_remaining: 1,
            clock_duration: 5,
        } as Clock,
        ability_unlock: {
            might: MightEnum.MILITARY,
            roll_tier: RollTier.GREAT,
            unlock: 3,
        } as MightRequirement,
    }
</script>

<div
    class="flex min-h-screen w-10/12 flex-col items-center justify-center rounded-xl bg-slate-600/70 px-4 py-16 text-center text-xl text-slate-300 backdrop-blur-lg"
>
    <h1 class="top-0 text-5xl text-red-600">An Ability</h1>
    <p class="mb-4">
        An Ability is the main crux of the game. As a deck building game, you
        are tasked with creating a deck that matches your playstyle as well as
        the play style of the class you have picked. As a result, it is
        important to know things about the abilities. On the <Special
            href="/rules/classes">Class</Special
        > Page, you can see that there are abilities for each of the classes in Miraculous
        kingdom. And they will look something like this:
    </p>
    <div class="flex w-96 flex-col justify-center">
        <Ability ability={abilityConst} />
    </div>
    There are 10 main parts.
    <ul class="my-8 list-none">
        <li><b>A</b> The title of the card.</li>
        <li>
            <b>B</b> The description of the card. When reading the card it will tell
            you everything that you should know about the card. At any point a player
            can ask to here the description of the card.
        </li>
        <li>
            <b>C & D</b> The description of the <Special href="/rules/clocks"
                >Clock</Special
            > applied to the card. Not all Cards have clocks with their card. They
            are a special part of the game where their effects can last much longer
            than the round of play and even a season so play them accordingly.
        </li>
        <li>
            <b>The Clock Counter</b> The Clock counter is denoted by a two blue numbers
            seperated by a "/". The first number will determine by the Clock Counter
            in the game how many rounds left there are. The second number denotes
            the total amount. Note: there are ways that the clock can go up from
            events in the game. So be careful! Also note the symbol next to the Clock
            counter. if you see it, that means that only you should know that it
            is active. So keep it a secret!
        </li>
        <li>
            <b>The Might Requirement</b> There icon next to a number is the
            Might Requirement. See the <Special href="">Might</Special> Rule Page
            for more information about wich one is which; however, this means that
            you must have that much of that Specified Might in order to play the
            card.
        </li>
        <li>
            <b>Roll Tier</b> A roll tier denotes what must be rolled in order to
            activate the card. If there is none then the effect happens automatically.
            That being said, there are 7 types of Roll tiers: None, the effect happens
            automatically; Fail, The effect happens will happen on a one; Bad, the
            will effect will happen when a 2-6 is rolled; Neutral: The Effect will
            happen when a 7-11 is rolled; Good, The effect will happen a 12-16 is
            rolled; Great, The effect will activate when a 17-19 is rolled; And a
            Fantastic, The roll happens when 20 or higher is rolled.
        </li>
    </ul>
</div>
