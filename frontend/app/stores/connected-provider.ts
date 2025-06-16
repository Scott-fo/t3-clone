import { create } from "zustand";
import { persist } from "zustand/middleware";
import type { ApiKey } from "~/domain/api-key";

interface State {
  data: ApiKey[] | null;
  loading: boolean;

  setLoading: (l: boolean) => void;
  setData: (data: ApiKey[] | null) => void;

  clear: () => void;
}

export const useConnectedProviderStore = create<State>()(
  persist(
    (set) => ({
      data: null,
      loading: true,

      setLoading: (loading) => set({ loading }),
      setData: (data) => set({ data, loading: false }),

      clear: () => set({ data: null, loading: true }),
    }),
    {
      name: "connected-provider-storage",
    }
  )
);
