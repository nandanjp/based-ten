// For more information about this file see https://dove.feathersjs.com/guides/cli/service.html

import { hooks as schemaHooks } from '@feathersjs/schema'

import {
  songsDataValidator,
  songsPatchValidator,
  songsQueryValidator,
  songsResolver,
  songsExternalResolver,
  songsDataResolver,
  songsPatchResolver,
  songsQueryResolver
} from './songs.schema'

import type { Application } from '../../../declarations'
import { SongsService, getOptions } from './songs.class'
import { songsPath, songsMethods } from './songs.shared'

export * from './songs.class'
export * from './songs.schema'

// A configure function that registers the service and its hooks via `app.configure`
export const songs = (app: Application) => {
  // Register our service on the Feathers application
  app.use(songsPath, new SongsService(getOptions(app)), {
    // A list of all methods this service exposes externally
    methods: songsMethods,
    // You can add additional custom events to be sent to clients here
    events: []
  })
  // Initialize hooks
  app.service(songsPath).hooks({
    around: {
      all: [schemaHooks.resolveExternal(songsExternalResolver), schemaHooks.resolveResult(songsResolver)]
    },
    before: {
      all: [schemaHooks.validateQuery(songsQueryValidator), schemaHooks.resolveQuery(songsQueryResolver)],
      find: [],
      get: [],
      create: [schemaHooks.validateData(songsDataValidator), schemaHooks.resolveData(songsDataResolver)],
      patch: [schemaHooks.validateData(songsPatchValidator), schemaHooks.resolveData(songsPatchResolver)],
      remove: []
    },
    after: {
      all: []
    },
    error: {
      all: []
    }
  })
}

// Add this service to the service type index
declare module '../../../declarations' {
  interface ServiceTypes {
    [songsPath]: SongsService
  }
}
