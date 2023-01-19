<script lang="ts">
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

<div class="rounded-xl bg-slate-700 py-8 mx-16 text-purple-700 max-h-fit md:scale-75 sm:scale-50 xl:scale-110 overflow-x-hidden">
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
                    The <b>Cultural Aficionado</b> is a dangerous noble none for
                    caring about the finer things in life. The
                    <b>Cultural Aficionado</b>
                    dispises the Military boars rutting around playing silly games.
                    The <b>Cultural Aficionado</b> always trys to make Kingdom as
                    Miraculous as their art.
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
                    rolls, gain <b>3 King's Favor</b>. Can only be done again
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
        <span class=" text-purple-500">Abilities: These can be used during an event's lifetime. After using one of these abilities, you must set a <b>4 Clock</b>. See the Manual <em>Press M</em> for more information. Navigate abilities: <em>Press [1-8]</em> or use the scroll wheel on a mouse.</span>
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
                <div class="char-ability-detail">
                Lorem ipsum dolor sit amet consectetur adipisicing elit. Officia earum placeat culpa modi recusandae quaerat. Similique, non natus ullam rem quia nemo tenetur exercitationem nihil veritatis officia. Odit, earum animi.
                </div>
            </section>
        </div>
        <div id="ab_1" class="char-ability-container">
            <section id="ab_c_1" class="char-ability">
                <div class="char-ability-title">Cultural Preservation</div>
                <div id="ab_d_1" class="char-ability-detail">
                    The Cultural Aficionado can preserve and promote the
                    kingdom's culture, increasing the kingdom's cultural might
                    for the event. Gain 1 King's Favor.
                </div>
            </section>
        </div>
        <div id="ab_2" class="char-ability-container">
            <section id="ab_c_2" class="char-ability" >
                <div class="char-ability-title">Artistic Endeavors</div>
                <div id="ab_d_2" class="char-ability-detail">
                    The Cultural Aficionado can organize and promote artistic
                    endeavors, increasing the chances of success in the event.
                    Use a Cultural Might roll to determine the outcome. A roll
                    of 12 or higher yields 2 tokens for the Cultural Aficionado.
                    If the Cultural Aficionado fails, they lose two favor.
                </div>
            </section>
        </div>
        <div id="ab_3" class="char-ability-container">
            <section id="ab_c_3" class="char-ability" >
                <div class="char-ability-title">Propaganda</div>
                <div id="ab_d_3" class="char-ability-detail">
                    The Cultural Aficionado can use propaganda to shape public
                    opinion and increase the chances of success in the event.
                    When a character makes a <b>Tiered Roll</b>, use a Cultural
                    Might roll to play the event to your favor. If you roll
                    higher than the opponent, take their rewards this round.
                </div>
            </section>
        </div>
        <div id="ab_4" class="char-ability-container">
            <section id="ab_c_4" class="char-ability" >
                <div class="char-ability-title">Cultural Diplomacy</div>
                <div id="ab_d_4" class="char-ability-detail">
                    The Cultural Aficionado can use culture to build diplomacy
                    with other kingdoms, increasing the chances of success in
                    the event. When you are supposed to make a <b
                        >Diplomatic Might</b
                    >
                    roll, you can make a <b>Cultural Migh</b> roll instead. On a
                    successful roll take a <b>Cultural Token</b>.
                </div>
            </section>
        </div>
        <div id="ab_5" class="char-ability-container">
            <section id="ab_c_5" class="char-ability" >
                <div class="char-ability-title">Cultural Heritage</div>
                <div id="ab_d_5" class="char-ability-detail">
                    The Cultural Aficionado can preserve and promote the
                    kingdom's heritage, increasing the chances of success in the
                    event. Use a Cultural Might roll to play the event to your
                    favor. Your fellow players decide the difficulty of the roll
                    based on the roll Tier system that you need to beat. A
                    success gains a heritage point (the ability to gain 1
                    experience point in Cultural might)
                </div>
            </section>
        </div>
        <div id="ab_6" class="char-ability-container">
            <section id="ab_c_6" class="char-ability" >
                <div class="char-ability-title">Cultural Revolution</div>
                <div id="ab_d_6" class="char-ability-detail">
                    The Cultural Aficionado can lead a cultural revolution,
                    promoting new and progressive ideas and increasing the
                    kingdom's cultural might for the event. Take a strategem
                    (the ability to roll two dice) for your next roll in the
                    round.
                </div>
            </section>
        </div>
        <div id="ab_7" class="char-ability-container">
            <section id="ab_c_7" class="char-ability" >
                <div class="char-ability-title">Cultural Fusion</div>
                <div id="ab_d_7" class="char-ability-detail">
                    The Cultural Aficionado can promote cultural fusion,
                    blending different cultures to create something new and
                    unique. Use a Cultural Might roll to play the event to your
                    favor. Your fellow players decide the difficulty of the roll
                    based on the roll Tier system that you need to beat. A
                    success gains a fusion point (the ability to gain 1
                    experience point in Cultural might)
                </div>
            </section>
        </div>
        <div id="ab_8" class="char-ability-container">
            <section id="ab_c_8" class="char-ability" >
                <div class="char-ability-title">Cultural Exchange</div>
                <div id="ab_d_8" class="char-ability-detail">
                    The Cultural Aficionado can lead a cultural exchange program
                    with other kingdoms. Use a Diplomatic Might Roll
                </div>
                <div>
                    <ul class="flex flex-col text-justify text-sm">
                        <li>
                            <b>Bad:</b> You can sabotage against another player.
                        </li>
                        <li>
                            <b>Poor:</b> You can level up a stat.
                        </li>
                        <li>
                            <b>Neutral:</b> Take a king's favor.
                        </li>
                        <li>
                            <b>Good:</b> Take a Strategem (the ability to roll two
                            dice) for a future roll.
                        </li>
                        <li>
                            <b>Great:</b> Steal a reward from a player later in the
                            game.
                        </li>
                    </ul>
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
