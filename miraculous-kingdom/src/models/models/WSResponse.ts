/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { Episode } from './Episode'
import type { IsReady } from './IsReady'
import type { SeasonResponse } from './SeasonResponse'

export type WSResponse =
    | {
          SEASONRESPONSE: SeasonResponse
      }
    | {
          EPISODERESPONSE: Episode
      }
    | {
          ISREADY: IsReady
      }
    | 'WAITING'
