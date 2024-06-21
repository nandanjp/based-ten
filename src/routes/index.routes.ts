import { Router } from 'express'
import anime_router from './anime.routes'
import movie_router from './movies.routes'
import song_router from './songs.routes'
import user_router from './users.routes'
import games_router from './video-games.routes'

const router = Router()
router.use('/api/anime', anime_router)
router.use('/api/movie', movie_router)
router.use('/api/song', song_router)
router.use('/api/user', user_router)
router.use('/api/video-game', games_router)

export default router

/*
Users:
- get all users
- get a user by email, displayName
- get user's lists
- create a user
- update user email, displayName, or password
- delete user
- create a new list
- add items to a list
- update an item in the list


Followers:
- get list of followers for a given user
- get list of users that a user is following
- follow a user
- unfollow a user

Likes:
- like a list
- remove liked list
- get all liked lists of a user
- get top liked lists
- get all media show
- get most liked media from all lists in a group

Movies, Songs, VideoGames, Anime:
- get all media
- get media by id
- create new media
- update existing media
- delete existing media

Groups:
- create a new group
- add a user to a group
- get all members in a group
- get all lists in a group
- update group name
- delete an existing group
- remove a user from a group
*/
