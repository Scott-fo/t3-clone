import type { ReplicacheType } from "~/contexts/ReplicacheContext";
import type { Replicache } from "replicache";
import { create } from "zustand";
import { listMessagesForChat, type Message } from "~/domain/message";

interface State {
  data: Message[];

  syncedChatId: string | null;

  setData: (d: Message[]) => void;
  appendMessage: (message: Message) => void;

  _unsubscribe: (() => void) | null;
  sync: (rep: Replicache<ReplicacheType>, chat_id: string) => void;
  cleanup: () => void;
}

export const useMessageStore = create<State>((set, get) => ({
  data: [],
  syncedChatId: null,
  setData: (data) => set({ data }),

  appendMessage: (message) => {
    if (get().syncedChatId === message.chat_id) {
      console.log(
        `Optimistically appending message to chat ${message.chat_id}`
      );
      set((state) => ({ data: [...state.data, message] }));
    } else {
      console.log(
        `Ignoring optimistic append for message to non-synced chat ${
          message.chat_id
        }. Current is ${get().syncedChatId}.`
      );
    }
  },

  _unsubscribe: null,

  sync: (rep, chat_id) => {
    get().cleanup();

    console.log("Syncing messages");

    const unsubscribe = rep.subscribe(
      (tx) => listMessagesForChat(tx, chat_id),
      {
        onData: (data) => {
          console.debug("Message store updated from subscription");
          set({ data, syncedChatId: chat_id });
        },
      }
    );
    set({ _unsubscribe: unsubscribe });
  },

  cleanup: () => {
    const unsubscribe = get()._unsubscribe;
    if (unsubscribe) {
      console.log("Cleaning up message subscription");

      unsubscribe();
      set({ _unsubscribe: null, data: [], syncedChatId: null });
    }
  },
}));
