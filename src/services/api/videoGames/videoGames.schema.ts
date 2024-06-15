// // For more information about this file see https://dove.feathersjs.com/guides/cli/service.schemas.html
import { resolve } from '@feathersjs/schema'
import { Type, getValidator, querySyntax } from '@feathersjs/typebox'
import type { Static } from '@feathersjs/typebox'

import type { HookContext } from '../../../declarations'
import { dataValidator, queryValidator } from '../../../validators'
import type { VideoGamesService } from './videoGames.class'

// Main data model schema
export const videoGamesSchema = Type.Object(
  {
    video_game_id: Type.Number(),
    title: Type.String(),
    media_image: Type.String(),
    created_at: Type.Date(),
    platform: Type.Union([
      Type.Literal('nintendo-switch'),
      Type.Literal('nintendo-wiiu'),
      Type.Literal('nintendo-wii'),
      Type.Literal('nintendo-gamecube'),
      Type.Literal('nintendo-64'),
      Type.Literal('ps5'),
      Type.Literal('ps4'),
      Type.Literal('ps3'),
      Type.Literal('ps2'),
      Type.Literal('xbox-x'),
      Type.Literal('xbox-s'),
      Type.Literal('xbox-one'),
      Type.Literal('xbox')
    ])
  },
  { $id: 'VideoGames', additionalProperties: false }
)
export type VideoGames = Static<typeof videoGamesSchema>
export const videoGamesValidator = getValidator(videoGamesSchema, dataValidator)
export const videoGamesResolver = resolve<VideoGames, HookContext<VideoGamesService>>({})

export const videoGamesExternalResolver = resolve<VideoGames, HookContext<VideoGamesService>>({})

// Schema for creating new entries
export const videoGamesDataSchema = Type.Pick(videoGamesSchema, ['title', 'media_image', 'platform'], {
  $id: 'VideoGamesData'
})
export type VideoGamesData = Static<typeof videoGamesDataSchema>
export const videoGamesDataValidator = getValidator(videoGamesDataSchema, dataValidator)
export const videoGamesDataResolver = resolve<VideoGames, HookContext<VideoGamesService>>({})

// Schema for updating existing entries
export const videoGamesPatchSchema = Type.Partial(videoGamesSchema, {
  $id: 'VideoGamesPatch'
})
export type VideoGamesPatch = Static<typeof videoGamesPatchSchema>
export const videoGamesPatchValidator = getValidator(videoGamesPatchSchema, dataValidator)
export const videoGamesPatchResolver = resolve<VideoGames, HookContext<VideoGamesService>>({})

// Schema for allowed query properties
export const videoGamesQueryProperties = Type.Pick(videoGamesSchema, ['video_game_id', 'title', 'platform'])
export const videoGamesQuerySchema = Type.Intersect(
  [
    querySyntax(videoGamesQueryProperties),
    // Add additional query properties here
    Type.Object({}, { additionalProperties: false })
  ],
  { additionalProperties: false }
)
export type VideoGamesQuery = Static<typeof videoGamesQuerySchema>
export const videoGamesQueryValidator = getValidator(videoGamesQuerySchema, queryValidator)
export const videoGamesQueryResolver = resolve<VideoGamesQuery, HookContext<VideoGamesService>>({})
