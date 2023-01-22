<script lang="ts">
    import GiAlarmClock from 'svelte-icons/gi/GiAlarmClock.svelte'
    import GiChemicalBolt from 'svelte-icons/gi/GiChemicalBolt.svelte';

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

<div class="rounded-xl bg-slate-700 py-8 mx-16 text-purple-700 max-h-fit md:scale-75 sm:scale-50 xl:scale-110 overflow-hidden">
    <div
        class="flex flex-row justify-center rounded-xl border-4 border-slate-900 bg-slate-800 p-4 mx-16 text-3xl"
    >
        <h1>Cultural Aficionado</h1>
    </div>
    <div
        class="my-4 mx-12 grid grid-cols-3 justify-evenly text-justify text-pink-600"
    >
        <div class="char-detail hover:translate-x-96">
            <div class="flex flex-col">
                <div class="char-detail-title">Description</div>
                <div class="px-2 pb-4">
                    The <b>Cultural Aficionado</b> is a dangerous noble known
                    caring about the finer things in life. The
                    <b>Cultural Aficionado</b> gameplay centers around gaining and spending <b>Cultural Tokens</b>. 
                    Think on your feet, react to those dimwitted boars, and dominate the court with exubirent showmanship.
                </div>
            </div>
        </div>
        <div class="char-detail">
            <div class="flex flex-col ">
                <div class="char-detail-title">Experience</div>
                <div class="px-2 pb-4">
                    The <b>Cultural Aficionado</b> gains experience in any
                    <b>Might</b>
                    by rolling a successful roll on a <b>Cultural Might</b>
                    roll, and sabotaging a <b>Military Might</b> roll.
                </div>
            </div>
        </div>
        <div class="char-detail hover:-translate-x-96">
            <div class="flex flex-col ">
                <div class="char-detail-title">Class Perk</div>
                <div class="px-2 pb-4">
                    The <b>Cultural Aficionado</b> can start an exhibition at
                    any point during the game. A <b>4 Clock</b> is started. If
                    the <b>Cultural Aficionado</b> successful on 3
                    <b>Cultural Might</b>
                    rolls, gain <b>3 Cultural Tokens</b>. Can only be done again
                    <b>1 Round</b>
                    after the <b>Clock</b> runs out.
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
                    <div class="w-1/3">
                        <GiAlarmClock />
                    </div>
                    <span>Magnum Opus</span>
                </div>
                <div id="ab_d_1" class="char-ability-detail">
                    The <b>Cultural Aficionado</b> can create a magnum opus.
                    During an event, declare that you will be constructing the <b>Magnum Opus</b>.
                    Declare a <b>Might</b>, and a <b>4n clock</b> where n is the difficulty.
                    Any player can contrubute to this <b>clock</b> when ever a roll is the same declared <b>Might</b>. 
                    When the <b>clock</b> reaches zero, everyone who contrubuted gains <b>n Tokens</b> of their choice.
                </div>
            </section>
        </div>
        <div id="ab_2" class="char-ability-container">
            <section id="ab_c_2" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Conservatorium</span>
                </div>
                <div id="ab_d_2" class="char-ability-detail">
                     When you are to make a roll that is not a part of another ability that you started this turn,
                     Freeze the result of that roll. You can then switch the result of the roll you made with another roll later. <em>You cannot store two or more at once.</em>
                </div>
            </section>
        </div>
        <div id="ab_3" class="char-ability-container">
            <section id="ab_c_3" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Propaganda</span>
                </div>
                <div id="ab_d_3" class="char-ability-detail">
                    The Cultural Aficionado can use propaganda to shape public
                    opinion and increase the chances of success in the event.
                    When a character makes a <b>Tiered Roll</b>, use a <b>Cultural
                    Might Roll</b> to play the event to your favor. If your result is 
                    higher than the opponent, take their rewards this round.
                </div>
            </section>
        </div>
        <div id="ab_4" class="char-ability-container">
            <section id="ab_c_4" class="char-ability" >
                <div class="char-ability-title">Gossip</div>
                <div id="ab_d_4" class="char-ability-detail">
                    As a result of the someone else's turn <b>Choose One:</b>
                    <ul>
                        <li>
                             Roll a <b>Diplomatic Might Great Roll</b>. 
                            
                        </li> 
                        <li>
                            Roll a <b>Criminal Might Good Roll</b> and pay a <b>Cultural Might Token</b> 
                        </li>
                    </ul>
                    Give that player <b>Public Disaproval</b>. (See Public Opinion Rules for more information)
                </div>
            </section>
        </div>
        <div id="ab_5" class="char-ability-container">
            <section id="ab_c_5" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Denounce!</span>
                </div>
                <div id="ab_d_5" class="char-ability-detail">
                    When a player declares an ability, spend <b>2 Cultural Tokens</b> to denounce them. 
                    Denounce them, and make a <b>Competeing Might Roll</b>. 
                    If you win, give that player <b>Public Disaproval</b>. (See Public Opinion Rules for more information)
                </div>
            </section>
        </div>
        <div id="ab_6" class="char-ability-container">
            <section id="ab_c_6" class="char-ability" >
                <div class="char-ability-title h-10 flex flex-row">
                    <div class="w-1/3">
                        <GiChemicalBolt />
                    </div>
                    <span>Influence</span>
                </div>
                <div id="ab_d_6" class="char-ability-detail">
                    <b>The Cultural Aficionado</b> Influences an event. 
                    <b>Choose One:</b>
                    <ul>
                        <li>Give a <b>Strategem</b> to a player to be used during this event. If they succeed on an roll this event, gain <b>2 King's Favor</b></li>
                        <li><b>Sabotage</b> a player's roll during this event. If they fail their roll, increase a <b>Might</b> </li>
                    </ul> 
                </div>
            </section>
        </div>
        <div id="ab_7" class="char-ability-container">
            <section id="ab_c_7" class="char-ability" >
                <div class="char-ability-title">Debochery</div>
                <div id="ab_d_7" class="char-ability-detail">
                    <b><em>You must have a Cultural score of 5 or more to use this ability</em></b>.
                    Spend <em>n</em> amount of <b>Gold Token(s)</b>. While this ability is recharging, all <b>Might Rolls</b> have <em>+n</em> to their results.
                </div>
            </section>
        </div>
        <div id="ab_8" class="char-ability-container">
            <section id="ab_c_8" class="char-ability" >
                <div class="char-ability-title">Renaissance</div>
                <div id="ab_d_8" class="char-ability-detail">
                    <b><em>You must have a Cultural score of 5 or more to use this ability</em></b>. 
                    As a result of the someone else's turn <b>Choose One:</b>
                    <ul>
                        <li>Make a <b>Scientific Might</b> roll, add your <b>Cultural Might Modifier</b>. On a success gain <b>4 Cultural Tokens</b></li>
                        <li>Make a <b>Scientific Might</b> roll, add your <b>Cultural Might Modifier</b>. On a success gain <b>4 Cultural Tokens</b></li>
                        <li>Make a <b>Scientific Might</b> roll, add your <b>Cultural Might Modifier</b>. On a success gain <b>4 Cultural Tokens</b></li>
                    </ul>
                    <em>A success on this roll is at least a 20 result</em>
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
