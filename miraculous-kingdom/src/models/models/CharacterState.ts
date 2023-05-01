/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * This enum represents the possible states of a character in a websocket game.
 * - Waiting: the character is waiting for its turn to come. - Going: the character is currently taking its turn.
 * - Gone: the character has already taken its turn and is no longer in play.
 */
export enum CharacterState {
    WAITING = 'Waiting',
    GOING = 'Going',
    GONE = 'Gone',
}
