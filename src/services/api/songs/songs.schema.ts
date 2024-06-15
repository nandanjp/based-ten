// // For more information about this file see https://dove.feathersjs.com/guides/cli/service.schemas.html
import { resolve } from '@feathersjs/schema'
import { Type, getValidator, querySyntax } from '@feathersjs/typebox'
import type { Static } from '@feathersjs/typebox'

import type { HookContext } from '../../../declarations'
import { dataValidator, queryValidator } from '../../../validators'
import type { SongsService } from './songs.class'

// Main data model schema
export const songsSchema = Type.Object(
  {
    song_id: Type.Number(),
    title: Type.String(),
    author: Type.String(),
    album: Type.String(),
    media_image: Type.String(),
    created_at: Type.Date()
  },
  { $id: 'Songs', additionalProperties: false }
)
export type Songs = Static<typeof songsSchema>
export const songsValidator = getValidator(songsSchema, dataValidator)
export const songsResolver = resolve<Songs, HookContext<SongsService>>({})

export const songsExternalResolver = resolve<Songs, HookContext<SongsService>>({})

// Schema for creating new entries
export const songsDataSchema = Type.Pick(songsSchema, ['title', 'author', 'album', 'media_image'], {
  $id: 'SongsData'
})
export type SongsData = Static<typeof songsDataSchema>
export const songsDataValidator = getValidator(songsDataSchema, dataValidator)
export const songsDataResolver = resolve<Songs, HookContext<SongsService>>({})

// Schema for updating existing entries
export const songsPatchSchema = Type.Partial(songsSchema, {
  $id: 'SongsPatch'
})
export type SongsPatch = Static<typeof songsPatchSchema>
export const songsPatchValidator = getValidator(songsPatchSchema, dataValidator)
export const songsPatchResolver = resolve<Songs, HookContext<SongsService>>({})

// Schema for allowed query properties
export const songsQueryProperties = Type.Pick(songsSchema, ['song_id', 'title', 'author', 'album'])
export const songsQuerySchema = Type.Intersect(
  [
    querySyntax(songsQueryProperties),
    // Add additional query properties here
    Type.Object({}, { additionalProperties: false })
  ],
  { additionalProperties: false }
)
export type SongsQuery = Static<typeof songsQuerySchema>
export const songsQueryValidator = getValidator(songsQuerySchema, queryValidator)
export const songsQueryResolver = resolve<SongsQuery, HookContext<SongsService>>({})
