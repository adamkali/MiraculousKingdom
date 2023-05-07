/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { APIError } from './APIError'

export type Progress =
    | 'Succeeding'
    | {
          Failing: APIError
      }
