/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { CharacterResponse } from './CharacterResponse';
import type { RollResult } from './RollResult';

export type RollResponse = {
    roll_winner: CharacterResponse;
    rolls: Array<RollResult>;
};
