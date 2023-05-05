/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { CharacterState } from './CharacterState';
import type { ClassEnum } from './ClassEnum';
import type { Clock } from './Clock';
import type { Might } from './Might';

export type CharacterResponse = {
    char_class: ClassEnum;
    /**
     * The character's clocks.
     */
    char_clocks: Array<Clock>;
    /**
     * The character's abilities.
     */
    char_deck: Array<Ability>;
    char_discard: Array<Ability>;
    char_hand: Array<Ability>;
    char_might: Might;
    /**
     * The name of the character.
     */
    char_name: string;
    char_state: CharacterState;
    /**
     * A secret lock to the character.
     */
    secret: string;
};
