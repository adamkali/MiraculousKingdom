/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Clock } from './Clock';
import type { MightRequirement } from './MightRequirement';
import type { RewardTypes } from './RewardTypes';

export type Ability = {
    ability_clock?: Clock | null;
    ability_desc: string;
    ability_name: string;
    ability_rewards: Array<RewardTypes>;
    ability_unlock: MightRequirement;
};
