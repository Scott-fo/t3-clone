import {
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { useReplicache } from "./ReplicacheContext";
import { useSSE } from "./SSEContext";
import type { Message } from "~/domain/message";
import { useMessageStore } from "~/stores/message";
import { nanoid } from "nanoid";
import { useUserStore } from "~/stores/user";

type StreamState = {
  pending: string;
};

interface PendingResponse {
  content: string;
}

interface ChatStreamContextType {
  pendingResponses: Record<string, PendingResponse>;
  startStream: (chat_id: string) => void;
}

const ChatStreamContext = createContext<ChatStreamContextType | null>(null);

type Response = {
  type: string;
  data: any;
};

interface Props {
  children: ReactNode;
}

export const ChatStreamProvider: React.FC<Props> = ({ children }) => {
  const rep = useReplicache();
  const sse = useSSE();
  const user = useUserStore((state) => state.data);

  const [pendingResponses, setPendingResponses] = useState<
    Record<string, PendingResponse>
  >({});

  const streams = useRef<Record<string, StreamState>>({}).current;

  useEffect(() => {
    const handleStreamChunk = (r: Response) => {
      const { chat_id, chunk } = r.data;

      if (!streams[chat_id]) {
        streams[chat_id] = { pending: "" };
      }

      const state = streams[chat_id];
      state.pending += chunk;

      setPendingResponses((p) => ({
        ...p,
        [chat_id]: {
          content: state.pending,
        },
      }));
    };

    const handleStreamDone = (r: Response) => {
      console.log("Handling stream done: ", r);

      const { chat_id, msg_id } = r.data;
      const state = streams[chat_id];

      if (state && user) {
        const now = new Date().toISOString();

        const id = msg_id ? msg_id : nanoid();

        const final_msg = {
          id,
          chat_id,
          user_id: user?.id,
          role: "assistant",
          body: state.pending,
          created_at: now,
          updated_at: now,
          version: 1,
        } as Message;

        useMessageStore.getState().appendMessage(final_msg);

        rep.mutate.createMessage(final_msg);
        rep.mutate.updateChat({
          id: chat_id,
          updated_at: now,
        });

        delete streams[chat_id];
        setPendingResponses((p) => {
          const { [chat_id]: _, ...rest } = p;
          return rest;
        });
      }
    };

    const handleStreamError = (r: Response) => {
      console.log("Handling stream error: ", r);

      const { chat_id, error } = r.data;
      const state = streams[chat_id];

      if (state && user) {
        state.pending = `Error: ${error}`;
        setPendingResponses((p) => ({
          ...p,
          [chat_id]: {
            content: state.pending,
          },
        }));

        const now = new Date().toISOString();
        rep.mutate.createMessage({
          id: `${chat_id}-error-${Date.now()}`,
          chat_id,
          user_id: user.id,
          role: "assistant",
          body: state.pending,
          created_at: now,
          updated_at: now,
          version: 1,
        });

        delete streams[chat_id];
        setPendingResponses((p) => {
          const { [chat_id]: _, ...rest } = p;
          return rest;
        });
      }
    };

    sse.addEventListener("chat-stream-chunk", handleStreamChunk);
    sse.addEventListener("chat-stream-done", handleStreamDone);
    sse.addEventListener("chat-stream-error", handleStreamError);

    return () => {
      sse.removeEventListener("chat-stream-chunk", handleStreamChunk);
      sse.removeEventListener("chat-stream-done", handleStreamDone);
      sse.removeEventListener("chat-stream-error", handleStreamError);
    };
  }, [sse, rep, streams]);

  async function startStream(chat_id: string) {
    streams[chat_id] = { pending: "" };
    setPendingResponses((p) => ({
      ...p,
      [chat_id]: { content: "" },
    }));
  }

  return (
    <ChatStreamContext.Provider value={{ pendingResponses, startStream }}>
      {children}
    </ChatStreamContext.Provider>
  );
};

export function useChatStream(): ChatStreamContextType {
  const ctx = useContext(ChatStreamContext);
  if (!ctx) {
    throw new Error("useChatStream must be used within a ChatStreamProvider");
  }

  return ctx;
}
