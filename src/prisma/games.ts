import { Platforms } from '@prisma/client'

interface VideoGame {
  id: number
  name: string
  image: string
  created_at: Date
  console: Platforms
}

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
