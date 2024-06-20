import { Platforms } from '../../services/games/games.schema'

interface Anime {
  anime_id: number
  name: string
  image: string
  studio: string
  created_at: Date
  episodes: number
}

interface Movie {
  id: number
  name: string
  image: string
  created_at: Date
}

interface Song {
  id: number
  name: string
  creator: string
  album: string
  image: string
  created_at: Date
}

interface User {
  name: string
  display_name: string
  email: string
  password: string
}

interface VideoGame {
  id: number
  name: string
  image: string
  created_at: Date
  console: Platforms
}

export const SeedAnime: Anime[] = [
  {
    anime_id: 1,
    name: 'Naruto',
    studio: 'Pierrot',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/94/NarutoCoverTankobon1.jpg',
    created_at: new Date('2002-10-03'),
    episodes: 220
  },
  {
    anime_id: 2,
    name: 'Attack on Titan',
    studio: 'Wit Studio',
    image: 'https://upload.wikimedia.org/wikipedia/en/6/68/Attack_on_Titan_cover.jpg',
    created_at: new Date('2013-04-07'),
    episodes: 87
  },
  {
    anime_id: 3,
    name: 'One Piece',
    studio: 'Toei Animation',
    image: 'https://upload.wikimedia.org/wikipedia/en/4/4b/OnePieceVolume61Cover.jpg',
    created_at: new Date('1999-10-20'),
    episodes: 1061
  },
  {
    anime_id: 4,
    name: 'Death Note',
    studio: 'Madhouse',
    image: 'https://upload.wikimedia.org/wikipedia/en/6/6f/Death_Note_Vol_1.jpg',
    created_at: new Date('2006-10-03'),
    episodes: 37
  },
  {
    anime_id: 5,
    name: 'Fullmetal Alchemist',
    studio: 'Bones',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/99/Fullmetal_Alchemist_-_Volume_1.jpg',
    created_at: new Date('2003-10-04'),
    episodes: 51
  },
  {
    anime_id: 6,
    name: 'Dragon Ball Z',
    studio: 'Toei Animation',
    image: 'https://upload.wikimedia.org/wikipedia/en/7/7d/Dragon_Ball_Z_vol._1_DVD_cover.jpg',
    created_at: new Date('1989-04-26'),
    episodes: 291
  },
  {
    anime_id: 7,
    name: 'My Hero Academia',
    studio: 'Bones',
    image: 'https://upload.wikimedia.org/wikipedia/en/a/af/My_Hero_Academia,_Volume_1.jpg',
    created_at: new Date('2016-04-03'),
    episodes: 88
  },
  {
    anime_id: 8,
    name: 'Demon Slayer',
    studio: 'Ufotable',
    image: 'https://upload.wikimedia.org/wikipedia/en/2/22/Demon_Slayer_-_Kimetsu_no_Yaiba%2C_volume_1.jpg',
    created_at: new Date('2019-04-06'),
    episodes: 44
  },
  {
    anime_id: 9,
    name: 'Sword Art Online',
    studio: 'A-1 Pictures',
    image: 'https://upload.wikimedia.org/wikipedia/en/b/b2/Sword_Art_Online_light_novel_volume_1_cover.jpg',
    created_at: new Date('2012-07-08'),
    episodes: 96
  },
  {
    anime_id: 10,
    name: 'One Punch Man',
    studio: 'Madhouse',
    image: 'https://upload.wikimedia.org/wikipedia/en/7/72/OnePunchMan_manga_cover.png',
    created_at: new Date('2015-10-04'),
    episodes: 24
  }
]

export const SeedGames: VideoGame[] = [
  {
    id: 1,
    name: 'The Last of Us',
    image: 'https://upload.wikimedia.org/wikipedia/en/a/a9/The_Last_of_Us.jpg',
    created_at: new Date('2013-06-14'),
    console: 'PS3'
  },
  {
    id: 2,
    name: 'Halo Infinite',
    image: 'https://upload.wikimedia.org/wikipedia/en/0/01/Halo_Infinite.png',
    created_at: new Date('2021-12-08'),
    console: 'XBOX'
  },
  {
    id: 3,
    name: 'Breath of the Wild',
    image: 'https://upload.wikimedia.org/wikipedia/en/0/0e/The_Legend_of_Zelda_Breath_of_the_Wild.jpg',
    created_at: new Date('2017-03-03'),
    console: 'NINTENDO_SWITCH'
  },
  {
    id: 4,
    name: 'God of War',
    image: 'https://upload.wikimedia.org/wikipedia/en/a/ac/God_of_War_4_cover.jpg',
    created_at: new Date('2018-04-20'),
    console: 'PS5'
  },
  {
    id: 5,
    name: 'Gears 5',
    image: 'https://upload.wikimedia.org/wikipedia/en/e/e3/Gears_5.jpg',
    created_at: new Date('2019-09-10'),
    console: 'XBOX'
  },
  {
    id: 6,
    name: 'Animal Crossing',
    image: 'https://upload.wikimedia.org/wikipedia/en/3/3b/Animal_Crossing_New_Horizons.jpg',
    created_at: new Date('2020-03-20'),
    console: 'NINTENDO_SWITCH'
  },
  {
    id: 7,
    name: 'Uncharted 4',
    image: 'https://upload.wikimedia.org/wikipedia/en/6/6f/Uncharted_4_box_artwork.jpg',
    created_at: new Date('2016-05-10'),
    console: 'PS4'
  },
  {
    id: 8,
    name: 'Forza Horizon 5',
    image: 'https://upload.wikimedia.org/wikipedia/en/e/ec/Forza_Horizon_5_cover_art.jpg',
    created_at: new Date('2021-11-09'),
    console: 'XBOX'
  },
  {
    id: 9,
    name: 'Mario Kart 8',
    image: 'https://upload.wikimedia.org/wikipedia/en/2/23/MarioKart8Boxart.jpg',
    created_at: new Date('2014-05-29'),
    console: 'NINTENDO_SWITCH'
  },
  {
    id: 10,
    name: 'Spider-Man',
    image: 'https://upload.wikimedia.org/wikipedia/en/0/00/Spider-Man_PS4_cover.jpg',
    created_at: new Date('2018-09-07'),
    console: 'PS4'
  }
]

export const SeedMovies: Movie[] = [
  {
    id: 1,
    name: 'Oppenheimer',
    image:
      'https://m.media-amazon.com/images/M/MV5BMDBmYTZjNjUtN2M1MS00MTQ2LTk2ODgtNzc2M2QyZGE5NTVjXkEyXkFqcGdeQXVyNzAwMjU2MTY@._V1_.jpg',
    created_at: new Date('2023-07-21')
  },
  {
    id: 2,
    name: 'Barbie',
    image:
      'https://m.media-amazon.com/images/M/MV5BNjU3N2QxNzYtMjk1NC00MTc4LTk1NTQtMmUxNTljM2I0NDA5XkEyXkFqcGdeQXVyODE5NzE3OTE@._V1_.jpg',
    created_at: new Date('2023-07-21')
  },
  {
    id: 3,
    name: 'Top Gun: Maverick',
    image:
      'https://m.media-amazon.com/images/M/MV5BZWYzOGEwNTgtNWU3NS00ZTQ0LWJkODUtMmVhMjIwMjA1ZmQwXkEyXkFqcGdeQXVyMjkwOTAyMDU@._V1_.jpg',
    created_at: new Date('2022-05-27')
  },
  {
    id: 4,
    name: 'Everything Everywhere All at Once',
    image:
      'https://m.media-amazon.com/images/M/MV5BYTdiOTIyZTQtNmQ1OS00NjZlLWIyMTgtYzk5Y2M3ZDVmMDk1XkEyXkFqcGdeQXVyMTAzMDg4NzU0._V1_FMjpg_UX1000_.jpg',
    created_at: new Date('2022-03-11')
  },
  {
    id: 5,
    name: 'Toy Story',
    image:
      'https://m.media-amazon.com/images/M/MV5BMDU2ZWJlMjktMTRhMy00ZTA5LWEzNDgtYmNmZTEwZTViZWJkXkEyXkFqcGdeQXVyNDQ2OTk4MzI@._V1_.jpg',
    created_at: new Date('1995-11-22')
  },
  {
    id: 6,
    name: "Five Nights at Freddy's",
    image:
      'https://upload.wikimedia.org/wikipedia/en/thumb/d/d6/Five_Nights_At_Freddy%27s_poster.jpeg/220px-Five_Nights_At_Freddy%27s_poster.jpeg',
    created_at: new Date('2023-10-27')
  },
  {
    id: 7,
    name: 'Spider-Man: Across the Spider-Verse',
    image: 'https://upload.wikimedia.org/wikipedia/en/b/b4/Spider-Man-_Across_the_Spider-Verse_poster.jpg',
    created_at: new Date('2023-06-02')
  },
  {
    id: 8,
    name: 'Avengers: Endgame',
    image: 'https://upload.wikimedia.org/wikipedia/en/0/0d/Avengers_Endgame_poster.jpg',
    created_at: new Date('2019-04-26')
  },
  {
    id: 9,
    name: 'Forrest Gump',
    image:
      'https://upload.wikimedia.org/wikipedia/en/thumb/6/67/Forrest_Gump_poster.jpg/220px-Forrest_Gump_poster.jpg',
    created_at: new Date('1994-07-06')
  },
  {
    id: 10,
    name: 'Green Book',
    image: 'https://upload.wikimedia.org/wikipedia/en/5/5b/Green_Book_%282018_poster%29.png',
    created_at: new Date('2018-09-11')
  }
]

export const SeedSongs: Song[] = [
  {
    id: 1,
    name: 'First Love / Late Spring',
    creator: 'Mitski',
    album: 'Bury Me at Makeout Creek',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/95/Bury_Me_At_Makeout_Creek.jpg',
    created_at: new Date('2014')
  },
  {
    id: 2,
    name: 'NEW MAGIC WAND',
    creator: 'Tyler the Creator',
    album: 'Igor',
    image: 'https://upload.wikimedia.org/wikipedia/en/5/51/Igor_-_Tyler%2C_the_Creator.jpg',
    created_at: new Date('2019')
  },
  {
    id: 3,
    name: 'My Way',
    creator: 'Frank Sinatra',
    album: 'My Way',
    image: 'https://upload.wikimedia.org/wikipedia/en/b/b2/My_Way_-_Frank_Sinatra.jpg',
    created_at: new Date('1969')
  },
  {
    id: 4,
    name: 'One Kiss',
    creator: 'Dua Lipa',
    album: 'One Kiss',
    image:
      'https://upload.wikimedia.org/wikipedia/en/thumb/3/3e/Calvin_Harris_and_Dua_Lipa_One_Kiss.png/220px-Calvin_Harris_and_Dua_Lipa_One_Kiss.png',
    created_at: new Date('2018')
  },
  {
    id: 5,
    name: 'Lost',
    creator: 'Frank Ocean',
    album: 'Channel Orange',
    image: 'https://upload.wikimedia.org/wikipedia/en/2/28/Channel_ORANGE.jpg',
    created_at: new Date('2012')
  },
  {
    id: 6,
    name: 'Scott Street',
    creator: 'Phoebe Bridgers',
    album: 'Stranger in the Alps',
    image:
      'https://upload.wikimedia.org/wikipedia/en/f/f2/Phoebe_Bridgers_%E2%80%93_Stranger_in_the_Alps.png',
    created_at: new Date('2017')
  },
  {
    id: 7,
    name: 'Francis Forever',
    creator: 'Mitski',
    album: 'Bury Me at Makeout Creek',
    image: 'https://upload.wikimedia.org/wikipedia/en/9/95/Bury_Me_At_Makeout_Creek.jpg',
    created_at: new Date('2014')
  },
  {
    id: 8,
    name: 'Motion Sickness',
    creator: 'Phoebe Bridgers',
    album: 'Stranger in the Alps',
    image:
      'https://upload.wikimedia.org/wikipedia/en/f/f2/Phoebe_Bridgers_%E2%80%93_Stranger_in_the_Alps.png',
    created_at: new Date('2017')
  },
  {
    id: 9,
    name: 'EARFQUAKE',
    creator: 'Tyler the Creator',
    album: 'Igor',
    image: 'https://upload.wikimedia.org/wikipedia/en/5/51/Igor_-_Tyler%2C_the_Creator.jpg',
    created_at: new Date('2019')
  },
  {
    id: 10,
    name: 'See You Again',
    creator: 'Tyler the Creator',
    album: 'Flower Boy',
    image: 'https://upload.wikimedia.org/wikipedia/en/c/c3/Tyler%2C_the_Creator_-_Flower_Boy.png',
    created_at: new Date('2017')
  }
]

export const SeedUsers: User[] = [
  {
    name: 'Alice Johnson',
    display_name: 'AliceJ',
    email: 'alice.johnson@example.com',
    password: 'password123'
  },
  {
    name: 'Bob Smith',
    display_name: 'BobS',
    email: 'bob.smith@example.com',
    password: 'password123'
  },
  {
    name: 'Charlie Brown',
    display_name: 'CharlieB',
    email: 'charlie.brown@example.com',
    password: 'password123'
  },
  {
    name: 'Dana White',
    display_name: 'DanaW',
    email: 'dana.white@example.com',
    password: 'password123'
  },
  {
    name: 'Eve Davis',
    display_name: 'EveD',
    email: 'eve.davis@example.com',
    password: 'password123'
  },
  {
    name: 'Frank Moore',
    display_name: 'FrankM',
    email: 'frank.moore@example.com',
    password: 'password123'
  },
  {
    name: 'Grace Lee',
    display_name: 'GraceL',
    email: 'grace.lee@example.com',
    password: 'password123'
  },
  {
    name: 'Hank Miller',
    display_name: 'HankM',
    email: 'hank.miller@example.com',
    password: 'password123'
  },
  {
    name: 'Ivy Clark',
    display_name: 'IvyC',
    email: 'ivy.clark@example.com',
    password: 'password123'
  },
  {
    name: 'Jack Wilson',
    display_name: 'JackW',
    email: 'jack.wilson@example.com',
    password: 'password123'
  }
]
