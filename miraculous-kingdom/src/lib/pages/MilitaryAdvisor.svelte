<script lang="ts">
    import GiAlarmClock from 'svelte-icons/gi/GiAlarmClock.svelte'
    import GiChemicalBolt from 'svelte-icons/gi/GiChemicalBolt.svelte';
    import GiAcousticMegaphone from 'svelte-icons/gi/GiAcousticMegaphone.svelte';

    let card: HTMLDivElement
    const ansii: string[] = ["1","2","3","4","5","6","7","8"]
    const handleWheel = (
        node: WheelEvent & { currentTarget: EventTarget & HTMLDivElement }
        ) => {
    if (!node.deltaY) {
        return
    }
    card.style.scrollBehavior = 'smooth'

        let change =
            (node.deltaY / Math.abs(node.deltaY)) *
            Math.floor(card.offsetWidth / 4)
        card.scrollLeft += change
    }

    const handleKBDown = (id: KeyboardEvent) => {
        if ( !ansii.includes(id.key)) {
            return null;
        }
        
        let position = 
            Math.floor(card.offsetWidth *(1/4)) 
            * (+id.key-1);

        let ids  : string = 'ab_'   + id.key;
        let ids_c: string = 'ab_c_' + id.key;
        let ids_d: string = 'ab_d_' + id.key;

        const element  : HTMLElement = document.getElementById(ids);
        const element_c: HTMLElement = document.getElementById(ids_c);
        const element_d: HTMLElement = document.getElementById(ids_d);

        element.classList.add('char-ability__pressed')
        element_c.classList.add('char-ability-card__pressed');
        element_d.classList.add('char-ability-detail__pressed');

        card.scrollTo(position, 0);
    }

    const handleKBUp = (id: KeyboardEvent) => {
         if ( !ansii.includes(id.key)) {
            return null;
        }       

        let ids  : string = 'ab_'   + id.key;
        let ids_c: string = 'ab_c_' + id.key;
        let ids_d: string = 'ab_d_' + id.key;

        const element  : HTMLElement = document.getElementById(ids);
        const element_c: HTMLElement = document.getElementById(ids_c);
        const element_d: HTMLElement = document.getElementById(ids_d);

        element.classList.remove('char-ability__pressed')
        element_c.classList.remove('char-ability-card__pressed');
        element_d.classList.remove('char-ability-detail__pressed');
    }
</script>

<div class="rounded-xl bg-slate-700 py-8 mx-16 text-red-700 max-h-fit md:scale-75 sm:scale-50 xl:scale-110 overflow-hidden">
    <div
        class="flex flex-row justify-center rounded-xl border-4 border-slate-900 bg-slate-800 p-4 mx-16 text-3xl"
    >
        <h1>Military Advisor</h1>
    </div>
    <div
        class="my-4 mx-12 grid grid-cols-3 justify-evenly text-justify text-orange-600"
    >
        <div class="char-detail hover:translate-x-96">
            <div class="flex flex-col">
                <div class="char-detail-title">Description</div>
                <div class="px-2 pb-4">
                    There is no one in the kingdom as knowlegeble as the <b>Military Advisor</b> for bloodshed and Honor. 
                    <b>Military Advisor</b> gameplay centers around building stacks of <b>Arms</b>. 
                    Build your army, and show the rest of the kingdom what true honor means.
                </div>
            </div>
        </div>
        <div class="char-detail">
            <div class="flex flex-col ">
                <div class="char-detail-title">Experience</div>
                <div class="px-2 pb-4">
                    The <b>Military Advisor</b> gains experience in any
                    <b>Might</b>
                    by spending a stack of <b>Arms</b>
                </div>
            </div>
        </div>
        <div class="char-detail hover:-translate-x-96">
            <div class="flex flex-col ">
                <div class="char-detail-title">Class Perk</div>
                <div class="px-2 pb-4">
                    The <b>Military Advisor</b> at the begining of every round, choose one of the following:
                    <ul>
                        <li>Gain a stack of <b>Arms</b></li>
                        <li>Spend <b>n Arms</b> to add to your roll.</li>
                        <li>Spend <b>5 Arms</b> to gain <b>King's favor</b></li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
    <div
        class="mb-4 mt-4 mx-16 flex flex-col items-center rounded-xl border-4 border-slate-900 bg-slate-800 p-4"
    >
        <!-- <h1>Abilities</h1> -->
        <span class=" text-purple-500">Abilities: These can be used during an event's lifetime. After using one of these abilities, you must set a <b>4 Clock</b>. See the Manual <em>Press M</em> for more information. Navigate abilities by <em>Pressing [1-8]</em> or use the scroll wheel on a mouse.</span>
    </div>
    <div
        class="mx-24 px-16 flex snap-x flex-row overflow-hidden"
        on:wheel|preventDefault={(e) => handleWheel(e)}
        bind:this={card}
    >
        <div class="char-ability-container invisible">
            <section class="char-ability">
                <div class="char-ability-title">Cultural Preservation</div>
                <div class="char-ability-detail">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Officia earum placeat culpa modi recusandae quaerat. Similique, non natus ullam rem quia nemo tenetur exercitationem nihil veritatis officia. Odit, earum animi.
                </div>
            </section>
        </div>
        <div class="char-ability-container invisible">
            <section class="char-ability">
                <div class="char-ability-title">Cultural Preservation</div>
                <div class="char-ability-detail"> Lorem ipsum dolor sit amet consectetur adipisicing elit. Officia earum placeat culpa modi recusandae quaerat. Similique, non natus ullam rem quia nemo tenetur exercitationem nihil veritatis officia. Odit, earum animi.
                </div>
            </section>
        </div>
        <div id="ab_1" class="char-ability-container">
            <section id="ab_c_1" class="char-ability">
                <div class="char-ability-title h-10 flex flex-row">
                    <span>ATTENTION!</span>
                </div>
                <div id="ab_d_1" class="char-ability-detail">
                    Spend <b>2 Arms</b> to force a <b>Tiered Roll</b> up a tier, or in the case of a <b>Success Roll</b> add <em>+ 3</em> to a future roll. <em>You can only activate this ability if you have used the reward</em>.
                </div>
            </section>
        </div>
        <div id="ab_2" class="char-ability-container">
            <section id="ab_c_2" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Route the enemy</span>
                </div>
                <div id="ab_d_2" class="char-ability-detail">
                    When another player is going to <b>Sabotage</b> you, spend <b>4 Arms</b> to negate the effect. Optionally, spend another <b>2 Arms</b> to return the <b>Sabotage</b> to the initiating player. <em>This does not affect <b>Sabotages</b> that are a part of an ability, only the <b>Sabotages</b> from the resources</em>.
                </div>
            </section>
        </div>
        <div id="ab_3" class="char-ability-container">
            <section id="ab_c_3" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiAcousticMegaphone />
                    </div>
                    <span>Orders</span>
                </div>
                <div id="ab_d_3" class="char-ability-detail">
                    <ul>
                        <li>When rolling a <b>Success Roll</b> spend <b>2 Arms</b> and any roll lower than ten, becomes 10.</li>
                        <li>When rolling a <b>Tiered Roll</b> spend <b>2 Arms</b> and any tier lower than Neutral becomes Neutral.</li>
                    </ul>
                </div>
            </section>
        </div>
        <div id="ab_4" class="char-ability-container">
            <section id="ab_c_4" class="char-ability" >
                <div class="char-ability-title">Military Training</div>
                <div id="ab_d_4" class="char-ability-detail">
                    <b>Sabotage</b> YOUR next roll. You can then roll a 6 sided die to gain the number rolled as stacks of <b>Arms</b>. <em>This ability can be done at any time and does not have a cooldown</em>.
                </div>
            </section>
        </div>
        <div id="ab_5" class="char-ability-container">
            <section id="ab_c_5" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiAcousticMegaphone />
                    </div>
                    <span>Espionage</span>
                </div>
                <div id="ab_d_4" class="char-ability-detail">
                    Make a <b>Criminal Might Tiered Roll</b>. On a good or better roll, call two numbers. If your result for a future roll matches one of the numbers you choose, then those rolls are considered Perfect.
                </div> 
            </section>
        </div>
        <div id="ab_6" class="char-ability-container">
            <section id="ab_c_6" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiAlarmClock />
                    </div>
                    <span>Conscription</span>
                </div>
                <div id="ab_d_6" class="char-ability-detail">
                    Choose a roll to fail in the future. When that roll is failed, start a <b>3 Clock</b>. Any time a player <em>including you</em> succeeds on a roll, gain a stack of <b>Arms</b>.
                </div>
            </section>
        </div>
        <div id="ab_7" class="char-ability-container">
            <section id="ab_c_7" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Total War</span>
                </div>
                <div id="ab_d_7" class="char-ability-detail">
                    <b><em>You must have a Military score of 5 or more to use this ability</em></b>. 
                    <b><em>This skill can be activated when someone Sabotages you.</em></b> 
                    Make a <b>Military Might 18 Roll</b>, you can spend any number of <b>Arms</b> stacks to steal their tokens as <b>King's Favor</b>. <b>Optionally</b> you can also spend spend any number of <b>Tokens</b> to increase the roll of any roll in the future.
                </div>
            </section>
        </div>
        <div id="ab_8" class="char-ability-container">
            <section id="ab_c_8" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiAlarmClock />
                    </div>
                    <span>Siege Warfare</span>
                </div>
                <div id="ab_d_5" class="char-ability-detail">
                    <b><em>You must have a Military score of 5 or more to use this ability</em></b>. 
                    At the end of an event, where you were not awarded <b>King's Favor</b> in the resolution round. You may start a <b>5 Clock</b>. Any time <b>King's Favor</b> would be rewarded, you are rewarded a stack of <b>Arms</b>. At any point in time, a player may challenge you to a <b>Military Might Contest</b> and reduce the clock. If you loose this <b>Contest</b> They gain the number of <b>Arms</b> stacks you accumilated during the Siege as <b>King's Favor</b>. When the clock reaches zero. You gain the accumilated <b>Arms</b> as <b>King's Favor</b>.
                </div>
            </section>
        </div>
        <div class="char-ability-container invisible">
            <section class="char-ability">
                <div class="char-ability-title">Cultural Preservation</div>
                <div class="char-ability-detail">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Officia earum placeat culpa modi recusandae quaerat. Similique, non natus ullam rem quia nemo tenetur exercitationem nihil veritatis officia. Odit, earum animi.
                </div>
            </section>
        </div>
        <div class="char-ability-container invisible">
            <section class="char-ability">
                <div class="char-ability-title">Cultural Preservation</div>
                <div class="char-ability-detail">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Officia earum placeat culpa modi recusandae quaerat. Similique, non natus ullam rem quia nemo tenetur exercitationem nihil veritatis officia. Odit, earum animi.
                </div>
            </section>
        </div>
    </div>
</div>

<svelte:window on:keydown|preventDefault={handleKBDown} on:keyup|preventDefault={handleKBUp} />
