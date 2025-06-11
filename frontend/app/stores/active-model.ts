import type { ReplicacheType } from "~/contexts/ReplicacheContext";
import type { Replicache } from "replicache";
import { create } from "zustand";
import { getActiveModel, type ActiveModel } from "~/domain/active-model";

interface State {
  data: ActiveModel | null;
  setData: (d: ActiveModel) => void;

  _unsubscribe: (() => void) | null;
  sync: (rep: Replicache<ReplicacheType>) => void;
  cleanup: () => void;
}

export const useActiveModelStore = create<State>((set, get) => ({
  data: null,
  setData: (data) => set({ data }),

  _unsubscribe: null,

  sync: (rep) => {
    get().cleanup();

    console.log("Syncing active model");

    const unsubscribe = rep.subscribe((tx) => getActiveModel(tx), {
      onData: (data) => {
        console.debug("Active model store updated from subscription");
        set({ data });
      },
    });
    set({ _unsubscribe: unsubscribe });
  },

  cleanup: () => {
    const unsubscribe = get()._unsubscribe;
    if (unsubscribe) {
      console.log("Cleaning up active model subscription");

      unsubscribe();
      set({ _unsubscribe: null, data: null });
    }
  },
}));
