import axios from 'axios';
import { MediaResponse } from './types';

export const getAllMedia = async () => {
  const rawResult = await axios.get('http://127.0.0.1:5000/api/media');
  const result: MediaResponse = rawResult.data;
  return result;
};
