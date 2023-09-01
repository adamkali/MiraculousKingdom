/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { RewardTypes } from './RewardTypes'

/**
 * The message given at the end of the round of play
 * it contains the winner of the round and the rewardt
 */
export type EpisodeResultItem = {
    /**
     * the name of the ability used
     */
    ability_name: string
    /**
     * the reward of the winner
     */
    reward: Array<RewardTypes>
    /**
     * the name of the winner
     */
    winner_name: string
    /**
     * the roll of the winner that got them there
     */
    winner_roll: number
}
