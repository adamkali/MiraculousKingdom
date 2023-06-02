/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { Clock } from './Clock';
import type { MightStat } from './MightStat';

export type RewardTypes = ('None' | {
Ability: Ability;
} | {
Experience: MightStat;
} | {
Clock: Clock;
});
