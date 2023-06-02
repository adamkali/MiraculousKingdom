/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Clock } from './Clock';
import type { QueueItem } from './QueueItem';
import type { SeasonEnum } from './SeasonEnum';
import type { SeasonResponse } from './SeasonResponse';

export type QueueResonse = {
    clocks: Array<Clock>;
    game: string;
    queue: Array<QueueItem>;
    season: SeasonResponse;
    turn_state: SeasonEnum;
};
