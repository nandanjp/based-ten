import { Type, type Static } from "@sinclair/typebox";
import { Media } from "./media";

const Anime = Type.Intersect([
  Media,
  Type.Object({
    numEpisodes: Type.Integer(),
  }),
]);

export type Anime = Static<typeof Anime>;
