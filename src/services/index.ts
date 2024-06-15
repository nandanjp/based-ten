import { videoGames } from './api/videoGames/videoGames'
import { songs } from './api/songs/songs'
import { movies } from './api/movies/movies'
import { anime } from './api/anime/anime'
// For more information about this file see https://dove.feathersjs.com/guides/cli/application.html#configure-functions
import type { Application } from '../declarations'

export const services = (app: Application) => {
  app.configure(videoGames)
  app.configure(songs)
  app.configure(movies)
  app.configure(anime)
  // All services will be registered here
}
