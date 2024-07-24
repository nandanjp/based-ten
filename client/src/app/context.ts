import { createContext, Dispatch } from "react";
import { UserType } from "../../services/api.types";

type UserState = {
  user: UserType | undefined;
  setUser: Dispatch<UserType>;
};

export const UserContext = createContext<UserState>({
  user: undefined,
  setUser: () => {},
});
