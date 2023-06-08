/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { CharacterResponse } from './CharacterResponse';
import type { RollRequest } from './RollRequest';

export type QueueItem = {
    queue_ability: Ability;
    queue_char: CharacterResponse;
    queue_initiative: number;
    queue_roll?: RollRequest | null;
};
