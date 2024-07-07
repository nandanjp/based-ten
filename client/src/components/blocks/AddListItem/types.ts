export enum MediaType {
  ANIME = 'anime',
  VIDEO_GAME = 'videogame',
  MOVIE = 'movie',
  SONG = 'song'
}

export type AddListItemProps = {
  mediaType?: MediaType
  onClick?: () => void
}
