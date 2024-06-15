// // For more information about this file see https://dove.feathersjs.com/guides/cli/service.schemas.html
import { resolve } from '@feathersjs/schema'
import { Type, getValidator, querySyntax } from '@feathersjs/typebox'
import type { Static } from '@feathersjs/typebox'

import type { HookContext } from '../../../declarations'
import { dataValidator, queryValidator } from '../../../validators'
import type { AnimeService } from './anime.class'

// Main data model schema
export const animeSchema = Type.Object(
  {
    anime_id: Type.Number(),
    title: Type.String(),
    media_image: Type.String(),
    created_at: Type.Date(),
    episodes: Type.Integer()
  },
  { $id: 'Anime', additionalProperties: false }
)
export type Anime = Static<typeof animeSchema>
export const animeValidator = getValidator(animeSchema, dataValidator)
export const animeResolver = resolve<Anime, HookContext<AnimeService>>({})

export const animeExternalResolver = resolve<Anime, HookContext<AnimeService>>({})

// Schema for creating new entries
export const animeDataSchema = Type.Pick(animeSchema, ['title', 'media_image', 'episodes'], {
  $id: 'AnimeData'
})
export type AnimeData = Static<typeof animeDataSchema>
export const animeDataValidator = getValidator(animeDataSchema, dataValidator)
export const animeDataResolver = resolve<Anime, HookContext<AnimeService>>({})

// Schema for updating existing entries
export const animePatchSchema = Type.Partial(animeSchema, {
  $id: 'AnimePatch'
})
export type AnimePatch = Static<typeof animePatchSchema>
export const animePatchValidator = getValidator(animePatchSchema, dataValidator)
export const animePatchResolver = resolve<Anime, HookContext<AnimeService>>({})

// Schema for allowed query properties
export const animeQueryProperties = Type.Pick(animeSchema, ['anime_id', 'title', 'episodes'])
export const animeQuerySchema = Type.Intersect(
  [
    querySyntax(animeQueryProperties),
    // Add additional query properties here
    Type.Object({}, { additionalProperties: false })
  ],
  { additionalProperties: false }
)
export type AnimeQuery = Static<typeof animeQuerySchema>
export const animeQueryValidator = getValidator(animeQuerySchema, queryValidator)
export const animeQueryResolver = resolve<AnimeQuery, HookContext<AnimeService>>({})
