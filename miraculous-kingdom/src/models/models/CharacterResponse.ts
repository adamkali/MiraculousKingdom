/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { CharacterState } from './CharacterState';
import type { ClassResponse } from './ClassResponse';
import type { Clock } from './Clock';
import type { Might } from './Might';

export type CharacterResponse = {
    /**
     * The character's abilities.
     */
    abilities: Array<Ability>;
    char_class: ClassResponse;
    /**
     * The character's clocks.
     */
    char_clocks: Array<Clock>;
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
