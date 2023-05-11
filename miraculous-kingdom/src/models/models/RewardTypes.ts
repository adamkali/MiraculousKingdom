/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';

export type RewardTypes = ('None' | {
Ability: Ability;
} | {
Experience: number;
});
