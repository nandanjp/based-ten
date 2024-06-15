interface Song {
  id: number
  name: string
  creator: string
  album: string
  image: string
  created_at: Date
}

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
