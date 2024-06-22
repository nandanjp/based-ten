import axios from 'axios'
import { AnimeIdSearchType, GetAnimeType, GetTopAnimeType } from './search.schema'

const client = axios.create({
  baseURL: 'https://api.jikan.moe/v4'
})

const BASE_URL = 'https://api.jikan.moe/v4' as const

export const GetAnimeByIdJikan = async (id: AnimeIdSearchType) => {
  return (await client.get(`anime/${id.id}/full`)).data
}

export const GetTopAnimeJikan = async (filter: GetTopAnimeType) => {
  let params = new URLSearchParams({
    type: filter.type,
    limit: filter.limit
  })
  if (filter.filter) {
    params = new URLSearchParams({
      ...params,
      filter: filter.filter
    })
  }
  if (filter.rating) {
    params = new URLSearchParams({
      ...params,
      rating: filter.rating
    })
  }
  return (await client.get(`top/anime?${params.toString()}`)).data
}

export const GetAnimeFromJikan = async (filter: GetAnimeType) => {
  let params = new URLSearchParams({
    limit: filter.limit,
    page: filter.page,
    status: filter.status,
    rating: filter.rating,
    order_by: filter.order_by,
    sort: filter.sort
  })
  if (filter.type) {
    params = new URLSearchParams({
      ...params,
      type: filter.type
    })
  }
  if (filter.score) {
    params = new URLSearchParams({
      ...params,
      rating: filter.score
    })
  }
  if (filter.min_score) {
    params = new URLSearchParams({
      ...params,
      rating: filter.min_score
    })
  }
  if (filter.max_score) {
    params = new URLSearchParams({
      ...params,
      rating: filter.max_score
    })
  }
  return (await client.get(`anime?${params.toString()}`)).data
}

export const GetRandomAnimeFromJikan = async () => {
  return (await client.get(`random/anime`)).data
}
