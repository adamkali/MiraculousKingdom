import * as API from "./models";

export const CharacterAPI = {
    getAll: API.ApiCharacterApiService.getCharacters,
    get: API.ApiCharacterApiService.getCharacterForGame,
};

export const GameAPI = {
    getAll: API.ApiGameApiService.getGames,
    get: API.ApiGameApiService.getGame,
    start: API.ApiGameApiService.startGame,
    addChar: API.ApiCharacterApiService,
};

export const ClassAPI = {
    getAll: API.ApiClassApiService.getClasses,
    get: API.ApiClassApiService.getClass,
};

export const SeasonAPI = {
    getAll: API.ApiSeasonApiService.getSeasons,
    get: API.ApiSeasonApiService.getSeason,
};

export { ApiError } from "./models/core/ApiError";
export {
    CancelablePromise,
    CancelError,
} from "./models/core/CancelablePromise";
export { OpenAPI } from "./models/core/OpenAPI";
export type { OpenAPIConfig } from "./models/core/OpenAPI";

export type { Ability } from "./models/models/Ability";
export type { APIError } from "./models/models/APIError";
export type { CharacterResponse } from "./models/models/CharacterResponse";
export { CharacterState } from "./models/models/CharacterState";
export type { CharAddedDetailedResponse } from "./models/models/CharAddedDetailedResponse";
export type { CharDetialedResponse } from "./models/models/CharDetialedResponse";
export type { ClassDetailedResponse } from "./models/models/ClassDetailedResponse";
export { ClassEnum } from "./models/models/ClassEnum";
export type { ClassResponse } from "./models/models/ClassResponse";
export type { Clock } from "./models/models/Clock";
export type { GameCreation } from "./models/models/GameCreation";
export type { GameInfo } from "./models/models/GameInfo";
export type { GameInfoDetailedResponse } from "./models/models/GameInfoDetailedResponse";
export type { GamesInfoDetailedResponse } from "./models/models/GamesInfoDetailedResponse";
export type { Might } from "./models/models/Might";
export { MightEnum } from "./models/models/MightEnum";
export type { MightRequirement } from "./models/models/MightRequirement";
export type { MightStat } from "./models/models/MightStat";
export type { NewCharacter } from "./models/models/NewCharacter";
export type { PassDetailedResponse } from "./models/models/PassDetailedResponse";
export type { Progress } from "./models/models/Progress";
export type { RewardTypes } from "./models/models/RewardTypes";
export { RollTier } from "./models/models/RollTier";
export type { SeasonDetailedResponse } from "./models/models/SeasonDetailedResponse";
export type { SeasonResponse } from "./models/models/SeasonResponse";
export type { SeasonsDetailedResponse } from "./models/models/SeasonsDetailedResponse";
export type { VecCharDetailedResponse } from "./models/models/VecCharDetailedResponse";
export type { VecClassDetailedResponse } from "./models/models/VecClassDetailedResponse";
