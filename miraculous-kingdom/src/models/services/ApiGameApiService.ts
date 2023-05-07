/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CharAddedDetailedResponse } from '../models/CharAddedDetailedResponse'
import type { GameCreation } from '../models/GameCreation'
import type { GameInfoDetailedResponse } from '../models/GameInfoDetailedResponse'
import type { GamesInfoDetailedResponse } from '../models/GamesInfoDetailedResponse'
import type { NewCharacter } from '../models/NewCharacter'
import type { PassDetailedResponse } from '../models/PassDetailedResponse'

import type { CancelablePromise } from '../core/CancelablePromise'
import { OpenAPI } from '../core/OpenAPI'
import { request as __request } from '../core/request'

export class ApiGameApiService {
    /**
     * @returns GamesInfoDetailedResponse Game found
     * @throws ApiError
     */
    public static getGames(): CancelablePromise<GamesInfoDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/game',
            errors: {
                500: `Internal Server Error`,
            },
        })
    }

    /**
     * @param requestBody
     * @returns PassDetailedResponse Found class from database
     * @throws ApiError
     */
    public static startGame(
        requestBody: GameCreation,
    ): CancelablePromise<PassDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/game',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                304: `Could not find class from database`,
                500: ` Internal error occured`,
            },
        })
    }

    /**
     * @param pass Password for entering the game.
     * @returns GameInfoDetailedResponse Found class from database
     * @throws ApiError
     */
    public static getGame(
        pass: string,
    ): CancelablePromise<GameInfoDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/game/{pass}',
            path: {
                pass: pass,
            },
            errors: {
                404: `Could not find class from database`,
                500: ` Internal error occured`,
            },
        })
    }

    /**
     * @param pass Password for entering the game.
     * @param requestBody
     * @returns CharAddedDetailedResponse Added Character to Game
     * @throws ApiError
     */
    public static addCharacter(
        pass: string,
        requestBody: NewCharacter,
    ): CancelablePromise<CharAddedDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/game/{pass}',
            path: {
                pass: pass,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request`,
                404: `Could not find class from database`,
                500: ` Internal error occured`,
            },
        })
    }
}
