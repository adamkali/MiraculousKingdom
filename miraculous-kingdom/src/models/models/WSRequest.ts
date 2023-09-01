/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

import type { WSAbilityRequest } from './WSAbilityRequest'
import type { WSReadyToStart } from './WSReadyToStart'
import type { WSRollRequest } from './WSRollRequest'
import type { WSTargetRequest } from './WSTargetRequest'

export type WSRequest =
    | {
          READYTOSTART: WSReadyToStart
      }
    | {
          ABILITYREQUEST: WSAbilityRequest
      }
    | {
          CHARACTERREQUEST: WSTargetRequest
      }
    | {
          ROLLREQUEST: WSRollRequest
      }
