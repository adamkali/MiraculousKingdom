<script lang="ts">
    import MightTable from '../../components/MightTable.svelte'
    import { MightEnum, type Might, type MightStat } from '../../models'

    const might = {
        might_culture: {
            stat_name: 'Culture',
            stat_enum: MightEnum.CULTURE,
            stat_value: 4,
            stat_exp: 0,
            stat_tokens: 3,
        } as unknown as MightStat,
        might_military: {
            stat_name: 'Military',
            stat_enum: MightEnum.MILITARY,
            stat_value: 5,
            stat_exp: 4,
            stat_tokens: 1,
        } as unknown as MightStat,
        might_science: {
            stat_name: 'Science',
            stat_enum: MightEnum.SCIENCE,
            stat_value: 3,
            stat_exp: 0,
            stat_tokens: 0,
        } as unknown as MightStat,
        might_religion: {
            stat_name: 'Religion',
            stat_enum: MightEnum.RELIGION,
            stat_value: 7,
            stat_exp: 4,
            stat_tokens: 10,
        } as unknown as MightStat,
        might_diplomacy: {
            stat_name: 'Diplomacy',
            stat_enum: MightEnum.DIPLOMACY,
            stat_value: 2,
            stat_exp: 0,
            stat_tokens: 0,
        } as unknown as MightStat,
        might_espionage: {
            stat_name: 'Espionage',
            stat_enum: MightEnum.ESPIONAGE,
            stat_value: 1,
            stat_exp: 0,
            stat_tokens: 0,
        } as unknown as MightStat,
    } as Might
</script>

<div
    class="flex min-h-[90rem] w-10/12 flex-col rounded-xl bg-slate-600/70 px-12 pt-6 text-justify align-top text-xl text-slate-300 backdrop-blur-lg"
>
    <div
        class="top-0 mb-8 flex flex-col rounded-xl bg-slate-800 p-8 text-7xl font-bold text-blue-600"
    >
        Character Might
    </div>
    <div class="rounded-xl bg-black p-16">
        <p class="text-3xl text-slate-400">
            The might in Miraculous Kingdom is a measure of how powerful a
            character is. There are six types of might in the game. The more
            likely you are likely your to win the game by winning the Season
            rolls. When you win a season roll you gain a King's favor. The
            player with the most king's favor at the end of the game wins. So by
            winning the season by using the game's various abilities, you will
            win the game. Now to explain the Character Might.
        </p>
        <ul>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Military Might</span> - This is the
                    might of your military.
                </p>
            </li>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Culture Might</span> - This is the
                    might of your cultural might in the Miraculous Kingdom.
                </p>
            </li>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Scientific Might</span> - This is
                    the might of your scientific might in the Miraculous Kingdom.
                </p>
            </li>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Religious Might</span> - This is
                    the might of your religious might in the Miraculous Kingdom.
                </p>
            </li>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Diplomatic Might</span> - This is
                    the might of your diplomatic might in the Miraculous Kingdom.
                </p>
            </li>
            <li>
                <p class="text-2xl text-slate-400">
                    <span class="text-blue-600">Espionage Might</span> - This is
                    the might of your economic might in the Miraculous Kingdom.
                </p>
            </li>
        </ul>
    </div>
    <MightTable {might} changeMight={() => {}} />
</div>
