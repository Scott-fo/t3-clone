import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";
import { useReplicache } from "./ReplicacheContext";
import { useSSE } from "./SSEContext";
import type { Message } from "~/domain/message";
import { useMessageStore } from "~/stores/message";
import { nanoid } from "nanoid";
import { useUserStore } from "~/stores/user";

interface PendingResponse {
  content: string;
  reasoning: string;
}

interface ChatStreamContextType {
  pendingResponses: Record<string, PendingResponse>;
  startStream: (chatId: string) => void;
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

  const startStream = useCallback((chatId: string) => {
    setPendingResponses((prev) => ({
      ...prev,
      [chatId]: { content: "", reasoning: "" },
    }));
  }, []);

  useEffect(() => {
    const handleStreamChunk = (r: Response) => {
      const {
        chat_id: chatId,
        chunk,
        reasoning,
      } = r.data as {
        chat_id: string;
        chunk?: string;
        reasoning?: string;
      };

      setPendingResponses((prev) => {
        const current = prev[chatId] ?? { content: "", reasoning: "" };

        return {
          ...prev,
          [chatId]: {
            content: chunk ? current.content + chunk : current.content,
            reasoning: reasoning
              ? current.reasoning + reasoning
              : current.reasoning,
          },
        };
      });
    };

    const handleStreamDone = async (r: Response) => {
      const { chat_id: chatId, msg_id } = r.data;

      const id = msg_id ?? nanoid();
      const now = new Date().toISOString();

      let finalMsg: Message | undefined;

      setPendingResponses((prev) => {
        const streamToFinalize = prev[chatId];

        if (streamToFinalize && user) {
          finalMsg = {
            id,
            chat_id: chatId,
            role: "assistant",
            body: streamToFinalize.content,
            reasoning: streamToFinalize.reasoning,
            created_at: now,
            updated_at: now,
          };

          useMessageStore.getState().appendMessage(finalMsg);
        }

        const { [chatId]: _removed, ...rest } = prev;
        return rest;
      });

      if (finalMsg) {
        await rep.mutate.createMessage(finalMsg);
        await rep.mutate.updateChat({ id: chatId, updated_at: now });
      }
    };

    const handleStreamError = async (r: Response) => {
      const { chat_id: chatId, error } = r.data;

      console.error("GOT STREAM ERROR:", r.data);

      const id = nanoid();
      const now = new Date().toISOString();

      let errorMsg: Message | undefined;

      setPendingResponses((prev) => {
        const streamWithError = prev[chatId];

        if (streamWithError && user) {
          errorMsg = {
            id,
            chat_id: chatId,
            role: "assistant",
            body: `Error: ${error}`,
            created_at: now,
            updated_at: now,
          };

          useMessageStore.getState().appendMessage(errorMsg);
        }

        const { [chatId]: _removed, ...rest } = prev;
        return rest;
      });

      if (errorMsg) {
        await rep.mutate.createMessage(errorMsg);
        await rep.mutate.updateChat({ id: chatId, updated_at: now });
      }
    };

    const handleStreamExit = (r: Response) => {
      const { chat_id: chatId } = r.data;

      setPendingResponses((prevStreams) => {
        const { [chatId]: _, ...rest } = prevStreams;
        return rest;
      });
    };

    sse.addEventListener("chat-stream-chunk", handleStreamChunk);
    sse.addEventListener("chat-stream-done", handleStreamDone);
    sse.addEventListener("chat-stream-error", handleStreamError);
    sse.addEventListener("chat-stream-exit", handleStreamExit);

    return () => {
      sse.removeEventListener("chat-stream-chunk", handleStreamChunk);
      sse.removeEventListener("chat-stream-done", handleStreamDone);
      sse.removeEventListener("chat-stream-error", handleStreamError);
      sse.removeEventListener("chat-stream-exit", handleStreamExit);
    };
  }, [sse, rep, user]);

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
