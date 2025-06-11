import type { ReplicacheType } from "~/contexts/ReplicacheContext";
import type { Replicache } from "replicache";
import { create } from "zustand";
import { listChats, type Chat } from "~/domain/chat";

interface State {
  data: Chat[];
  setData: (d: Chat[]) => void;

  _unsubscribe: (() => void) | null;
  sync: (rep: Replicache<ReplicacheType>) => void;
  cleanup: () => void;
}

export const useChatStore = create<State>((set, get) => ({
  data: [],
  setData: (data) => set({ data }),

  _unsubscribe: null,

  sync: (rep) => {
    get().cleanup();

    console.log("Syncing chats");

    const unsubscribe = rep.subscribe((tx) => listChats(tx), {
      onData: (data) => {
        console.debug("Chat store updated from subscription");
        set({ data });
      },
    });
    set({ _unsubscribe: unsubscribe });
  },

  cleanup: () => {
    const unsubscribe = get()._unsubscribe;
    if (unsubscribe) {
      console.log("Cleaning up chat subscription");

      unsubscribe();
      set({ _unsubscribe: null, data: [] });
    }
  },
}));
