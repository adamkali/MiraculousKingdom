/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { SeasonEnum } from './SeasonEnum';

export type GameInfo = {
    game_chars: Array<string>;
    game_name: string;
    game_pass: string;
    game_ruler: string;
    game_season: SeasonEnum;
};
