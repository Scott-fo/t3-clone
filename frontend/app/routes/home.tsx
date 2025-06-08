import ChatInput from "~/components/chat-input";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { useSubscribe } from "replicache-react";
import type { ReadTransaction } from "replicache";
import { listMessagesForChat } from "~/domain/message";
import { useCallback, useEffect, useRef } from "react";
import { useAuth } from "~/contexts/AuthContext";
import { nanoid } from "nanoid";
import { MessageBubble } from "~/components/message-bubble";
import type { Route } from "./+types/chat";

export default function Page({ params }: Route.ComponentProps) {
  const rep = useReplicache();
  const { user } = useAuth();

  const messages = useSubscribe(
    rep,
    async (tx: ReadTransaction) => listMessagesForChat(tx, params.thread_id),
    {
      default: [],
      dependencies: [params.thread_id],
    }
  );

  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (messages.length === 0) return;
    containerRef.current?.scrollTo({
      top: containerRef.current.scrollHeight,
      behavior: "smooth",
    });
  }, [messages]);

  const onSendMessage = useCallback(
    async (msg: string) => {
      if (!user) return;
      const now = new Date().toISOString();

      // if this is the first message in a new chat
      if (messages.length === 0) {
        rep.mutate.createChat({
          id: params.thread_id,
          user_id: user.id,
          title: "test",
          archived: false,
          created_at: now,
          updated_at: now,
          version: 1,
        });
      } else {
        rep.mutate.updateChat({
          id: params.thread_id,
          updated_at: now,
        });
      }

      await rep.mutate.createMessage({
        id: nanoid(),
        chat_id: params.thread_id,
        user_id: user.id,
        role: "user",
        body: msg,
        created_at: now,
        updated_at: now,
        version: 1,
      });
      // startStream(params.threadId);
    },
    [messages.length, params.thread_id, rep, user?.id]
  );

  return (
    <div className="h-full max-h-screen h-screen w-full mx-auto flex flex-col overflow-hidden">
      <div
        ref={containerRef}
        className="flex-1 overflow-y-auto px-4 py-4 space-y-10 custom-scrollbar"
      >
        {messages.map((msg) => (
          <MessageBubble
            key={msg.id}
            id={msg.id}
            role={msg.role}
            msg={msg.body}
          />
        ))}
      </div>

      <div className="w-full max-w-3xl mx-auto shrink-0 pt-2">
        <div className="relative">
          <div
            className="absolute -top-2 -left-2 -right-2 bottom-0
                         bg-accent/60 rounded-tl-[1.25rem] rounded-tr-[1.25rem]
                         ring-1 ring-black/10 pointer-events-none
                         z-0"
            aria-hidden="true"
          />

          <div
            className="relative z-10 rounded-tl-xl rounded-tr-xl
                         p-2 bg-primary-foreground"
          >
            <ChatInput
              handleSubmit={onSendMessage}
              chatId={params.thread_id}
              disabled={false}
            />
          </div>
        </div>
      </div>
    </div>
  );
}
