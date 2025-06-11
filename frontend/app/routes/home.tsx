import ChatInput from "~/components/chat-input";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { useCallback } from "react";
import { nanoid } from "nanoid";
import { useUserStore } from "~/stores/user";
import { useChatStream } from "~/contexts/ChatStreamContext";
import { href, useNavigate } from "react-router";

export default function Page() {
  const rep = useReplicache();
  const user = useUserStore((state) => state.data);
  const { startStream } = useChatStream();
  const navigate = useNavigate();

  const onSendMessage = useCallback(
    async (msg: string) => {
      if (!user) return;
      const now = new Date().toISOString();

      const new_chat_id = nanoid();

      rep.mutate.createChat({
        id: new_chat_id,
        user_id: user.id,
        title: "test",
        archived: false,
        created_at: now,
        updated_at: now,
        version: 1,
      });

      rep.mutate.createMessage({
        id: nanoid(),
        chat_id: new_chat_id,
        user_id: user.id,
        role: "user",
        body: msg,
        created_at: now,
        updated_at: now,
        version: 1,
      });

      startStream(new_chat_id);
      navigate(href("/chat/:thread_id", { thread_id: new_chat_id }));
    },
    [rep, user?.id]
  );

  return (
    <div className="h-full max-h-screen h-screen w-full mx-auto flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto px-4 py-4 space-y-10 custom-scrollbar"></div>
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
            <ChatInput handleSubmit={onSendMessage} disabled={false} />
          </div>
        </div>
      </div>
    </div>
  );
}
