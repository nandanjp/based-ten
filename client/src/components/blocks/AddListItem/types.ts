import { MediaType } from "@/../services/api.types";

export type AddListItemProps = {
  listItem?: MediaType;
  list?: MediaType[];
  onClick?: (newItem: MediaType) => void;
};
