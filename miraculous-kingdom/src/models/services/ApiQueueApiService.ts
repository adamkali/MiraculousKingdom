/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { QueueDetailedResponse } from '../models/QueueDetailedResponse';

import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';

export class ApiQueueApiService {

    /**
     * @returns any Found Queue from database
     * @throws ApiError
     */
    public static wsEntyrpoint(): CancelablePromise<any> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/api/queue',
            errors: {
                404: `Could not find queue from database`,
                500: ` Internal error occured`,
            },
        });
    }

    /**
     * @param pass Password of the game to be set
     * @returns QueueDetailedResponse Found Queue from database
     * @throws ApiError
     */
    public static setQueue(
pass: string,
): CancelablePromise<QueueDetailedResponse> {
        return __request(OpenAPI, {
            method: 'PUT',
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
