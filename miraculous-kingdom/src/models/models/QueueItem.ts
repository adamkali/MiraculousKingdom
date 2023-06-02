/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { CharacterResponse } from './CharacterResponse';

export type QueueItem = {
    queue_ability: Ability;
    queue_char: CharacterResponse;
    queue_initiative: number;
};
