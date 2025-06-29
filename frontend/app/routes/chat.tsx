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
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar";
import { cn } from "~/lib/utils";
import { useChatStore } from "~/stores/chat";

export default function Page({ params }: Route.ComponentProps) {
  const rep = useReplicache();
  const sidebar = useSidebar();
  const { startStream, pendingResponses } = useChatStream();

  const user = useUserStore((state) => state.data);
  const messages = useMessageStore((state) => state.data);
  const chat = useChatStore((state) =>
    state.data.find((c) => c.id === params.thread_id)
  );

  const [showMessages, setShowMessages] = useState(false);

  const { sync, cleanup, appendMessage } = useMessageStore.getState();

  const containerRef = useRef<HTMLDivElement>(null);
  const pendingRef = useRef<HTMLDivElement>(null);
  const chatInputRef = useRef<HTMLTextAreaElement>(null);
  const hasScrolledToBottomRef = useRef<boolean>(false);

  useEffect(() => {
    sync(rep, params.thread_id);

    chatInputRef.current?.focus();

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
          forked: false,
          created_at: now,
          updated_at: now,
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
      await rep.mutate.createMessage(usr_msg);

      startStream(params.thread_id);
    },
    [messages.length, params.thread_id, rep, user?.id]
  );

  const forkChat = useCallback(
    (break_id: string) => {
      if (!chat) return;

      const new_id = nanoid();

      const idx = messages.findIndex((m) => m.id === break_id);
      const new_msgs = messages.slice(0, idx + 1).map((m) => ({
        ...m,
        id: nanoid(),
        chat_id: new_id,
      }));

      rep.mutate.forkChat({
        new_id,
        title: chat.title ?? "Forked chat",
        time: new Date().toISOString(),
        msgs: new_msgs,
      });
    },
    [chat, messages, rep]
  );

  return (
    <div className="relative h-dvh w-full mx-auto flex flex-col overflow-hidden">
      <SidebarTrigger
        variant="secondary"
        className={cn(
          `z-50 absolute opacity-100 top-3 left-3 transition-opacity duration-200`,
          sidebar.open && !sidebar.isMobile && "opacity-0"
        )}
      />
      <div
        ref={containerRef}
        className="z-10 flex-1 overflow-y-auto overflow-x-none px-4 py-4 space-y-2 mt-2 custom-scrollbar"
      >
        {showMessages && (
          <MessageList forkChat={forkChat} messages={messages} />
        )}
        {showMessages && isPending && (
          <MessageBubble
            ref={pendingRef}
            key="pending"
            id="pending"
            role="assistant"
            reasoning={pendingResponses[params.thread_id]?.reasoning ?? null}
            msg={pendingResponses[params.thread_id]?.content ?? ""}
          />
        )}
      </div>

      <div className="w-full max-w-3xl mx-auto shrink-0 pt-2">
        <div className="relative">
          <div
            className="absolute -top-2 -left-2 -right-2 bottom-0
                         bg-primary/30 sm:rounded-tl-[1.25rem] sm:rounded-tr-[1.25rem]
                         ring-1 ring-black/10 pointer-events-none
                         z-0"
            aria-hidden="true"
          />

          <div
            className="relative z-10 sm:rounded-tl-xl sm:rounded-tr-xl
                         p-2 bg-primary-foreground"
          >
            <ChatInput
              ref={chatInputRef}
              handleSubmit={onSendMessage}
              disabled={isPending}
            />
          </div>
        </div>
      </div>
    </div>
  );
}

const MessageList = memo(
  ({
    messages,
    forkChat,
  }: {
    messages: Message[];
    forkChat: (msgId: string) => void;
  }) => {
    return messages.map((msg) => (
      <MessageBubble
        forkChat={forkChat}
        key={msg.id}
        id={msg.id}
        role={msg.role}
        msg={msg.body}
        reasoning={msg.reasoning}
      />
    ));
  }
);
