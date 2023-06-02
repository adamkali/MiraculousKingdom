/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { QueueDetailedResponse } from '../models/QueueDetailedResponse';
import type { SeasonResponse } from '../models/SeasonResponse';
import type { TurnRequest } from '../models/TurnRequest';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ApiQueueApiService {

    /**
     * @param pass Password for entering the game.
     * @param requestBody 
     * @returns QueueDetailedResponse Listed classes from database
     * @throws ApiError
     */
    public static setSeason(
pass: string,
requestBody: SeasonResponse,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/queue/season/{pass}',
            path: {
                'pass': pass,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad Request: id`,
                500: `Internal error occured`,
            },
        });
    }

    /**
     * @param pass Password for entering the game.
     * @param requestBody 
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static takeTurn(
pass: string,
requestBody: TurnRequest,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/api/queue/turn/{pass}',
            path: {
                'pass': pass,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                404: `Could not find queue from database`,
                500: ` Internal error occured`,
            },
        });
    }

    /**
     * @param pass Password for entering the game.
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static getQueue(
pass: string,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/queue/{pass}',
            path: {
                'pass': pass,
            },
            errors: {
                404: `Could not find queue from database`,
                500: ` Internal error occured`,
            },
        });
    }

}
