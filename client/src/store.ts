import { create } from "zustand";

type UserStore = {
  user_token: string;
  set_token: (token: string) => void;
  has_token: () => boolean;
};

export const useUserStore = create<UserStore>((set, get) => ({
  user_token: "",
  set_token: (token) => {
    set({ user_token: token });
  },
  has_token: () => get().user_token !== "",
}));
