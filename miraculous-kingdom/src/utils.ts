import { type Episode, type IsReady, type SeasonResponse } from './models'

export function convertToWS(endoint: string): string {
    return endoint.replace('http', 'ws')
}

export function isSeasonResponse(data: any): data is SeasonResponse {
    return typeof data === 'object' && 'SEASONRESPONSE' in data
}

export function isEpisodeResponse(data: any): data is Episode {
    return typeof data === 'object' && 'EPISODERESPONSE' in data
}

export function isIsReady(data: any): data is IsReady {
    return typeof data === 'object' && 'ISREADY' in data
}

export function isWaiting(data: any): data is 'WAITING' {
    return data === 'WAITING'
}
