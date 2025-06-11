import ChatInput from "~/components/chat-input";
import { useReplicache } from "~/contexts/ReplicacheContext";
import {
  memo,
  startTransition,
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { nanoid } from "nanoid";
import { MessageBubble } from "~/components/message-bubble";
import type { Route } from "./+types/chat";
import { useChatStream } from "~/contexts/ChatStreamContext";
import { useMessageStore } from "~/stores/message";
import type { Message } from "~/domain/message";
import { useUserStore } from "~/stores/user";

export default function Page({ params }: Route.ComponentProps) {
  const rep = useReplicache();
  const { startStream, pendingResponses } = useChatStream();

  const user = useUserStore((state) => state.data);
  const messages = useMessageStore((state) => state.data);
  const [showMessages, setShowMessages] = useState(false);

  const { sync, cleanup, appendMessage } = useMessageStore.getState();

  const containerRef = useRef<HTMLDivElement>(null);
  const pendingRef = useRef<HTMLDivElement>(null);
  const hasScrolledToBottomRef = useRef<boolean>(false);

  useEffect(() => {
    sync(rep, params.thread_id);

    return () => {
      cleanup();
    };
  }, [rep, params.thread_id, sync, cleanup]);

  useLayoutEffect(() => {
    setShowMessages(false);
    startTransition(() => setShowMessages(true));

    hasScrolledToBottomRef.current = false;
  }, [params.thread_id]);

  useEffect(() => {
    if (
      !showMessages ||
      messages.length === 0 ||
      hasScrolledToBottomRef.current
    ) {
      return;
    }

    containerRef.current?.scrollTo({
      top: containerRef.current.scrollHeight,
      behavior: "auto",
    });

    hasScrolledToBottomRef.current = true;
  }, [showMessages, messages]);

  const isPending =
    pendingResponses[params.thread_id] !== undefined ||
    (messages &&
      messages.length > 0 &&
      messages[messages.length - 1].role === "user");

  useEffect(() => {
    if (isPending && pendingRef.current) {
      const animationFrameId = requestAnimationFrame(() => {
        pendingRef.current?.scrollIntoView({
          behavior: "smooth",
          block: "end",
        });
      });

      return () => cancelAnimationFrame(animationFrameId);
    }
  }, [isPending, pendingRef]);

  const onSendMessage = useCallback(
    async (msg: string) => {
      if (!user) return;
      const now = new Date().toISOString();

      // if this is the first message in a new chat
      if (messages.length === 0) {
        rep.mutate.createChat({
          id: params.thread_id,
          user_id: user.id,
          forked: false,
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

      const usr_msg = {
        id: nanoid(),
        chat_id: params.thread_id,
        user_id: user.id,
        role: "user",
        body: msg,
        created_at: now,
        updated_at: now,
        version: 1,
      } as Message;

      appendMessage(usr_msg);
      rep.mutate.createMessage(usr_msg);

      startStream(params.thread_id);
    },
    [messages.length, params.thread_id, rep, user?.id]
  );

  return (
    <div className="h-full max-h-screen h-screen w-full mx-auto flex flex-col overflow-hidden">
      <div
        ref={containerRef}
        style={{
          contain: "layout paint",
        }}
        className="flex-1 overflow-y-auto px-4 py-4 space-y-10 custom-scrollbar"
      >
        {showMessages && <MessageList messages={messages} />}
        {showMessages && isPending && (
          <MessageBubble
            chat_id={params.thread_id}
            ref={pendingRef}
            key="pending"
            id="pending"
            role="assistant"
            msg={pendingResponses[params.thread_id]?.content ?? ""}
          />
        )}
      </div>

      <div className="w-full max-w-3xl mx-auto shrink-0 pt-2">
        <div className="relative">
          <div
            className="absolute -top-2 -left-2 -right-2 bottom-0
                         bg-primary/30 rounded-tl-[1.25rem] rounded-tr-[1.25rem]
                         ring-1 ring-black/10 pointer-events-none
                         z-0"
            aria-hidden="true"
          />

          <div
            className="relative z-10 rounded-tl-xl rounded-tr-xl
                         p-2 bg-primary-foreground"
          >
            <ChatInput handleSubmit={onSendMessage} disabled={isPending} />
          </div>
        </div>
      </div>
    </div>
  );
}

const MessageList = memo(({ messages }: { messages: Message[] }) => {
  return messages.map((msg) => (
    <MessageBubble
      key={msg.id}
      id={msg.id}
      chat_id={msg.chat_id}
      role={msg.role}
      msg={msg.body}
    />
  ));
});
