import { type Static } from "@sinclair/typebox";
import { Media } from "./media";

export const Movie = Media;
export type MovieType = Static<typeof Movie>;
