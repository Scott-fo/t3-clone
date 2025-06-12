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

    const handleStreamDone = (r: Response) => {
      const { chat_id: chatId, msg_id } = r.data;

      setPendingResponses((prevStreams) => {
        const streamToFinalize = prevStreams[chatId];

        if (streamToFinalize && user) {
          const now = new Date().toISOString();
          const id = msg_id ? msg_id : nanoid();

          const final_msg = {
            id,
            chat_id: chatId,
            user_id: user?.id,
            role: "assistant",
            body: streamToFinalize.content,
            reasoning: streamToFinalize.reasoning,
            created_at: now,
            updated_at: now,
            version: 1,
          } as Message;

          useMessageStore.getState().appendMessage(final_msg);

          rep.mutate.createMessage(final_msg);
          rep.mutate.updateChat({
            id: chatId,
            updated_at: now,
          });
        }

        const { [chatId]: _, ...rest } = prevStreams;
        return rest;
      });
    };

    const handleStreamError = (r: Response) => {
      const { chat_id: chatId, error } = r.data;

      setPendingResponses((prevStreams) => {
        const streamWithError = prevStreams[chatId];

        if (streamWithError && user) {
          const now = new Date().toISOString();
          const errorMessage = `Error: ${error}`;

          rep.mutate.createMessage({
            id: `${chatId}-error-${Date.now()}`,
            chat_id: chatId,
            role: "assistant",
            body: errorMessage,
            created_at: now,
            updated_at: now,
          });
        }

        const { [chatId]: _, ...rest } = prevStreams;
        return rest;
      });
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
