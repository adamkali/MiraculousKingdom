/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Ability } from './Ability';
import type { Clock } from './Clock';
import type { DrawCard } from './DrawCard';
import type { Experience } from './Experience';
import type { PayToken } from './PayToken';
import type { Token } from './Token';

export type RewardTypes = ('None' | {
Ability: Ability;
} | {
Experience: Experience;
} | {
Clock: Clock;
} | {
Token: Token;
} | {
DrawCard: DrawCard;
} | {
PayToken: PayToken;
});
