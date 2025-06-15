import ChatInput from "~/components/chat-input";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { useCallback } from "react";
import { nanoid } from "nanoid";
import { useUserStore } from "~/stores/user";
import { useChatStream } from "~/contexts/ChatStreamContext";
import { href, useNavigate } from "react-router";
import { SidebarTrigger, useSidebar } from "~/components/ui/sidebar";
import { cn } from "~/lib/utils";

export default function Page() {
  const rep = useReplicache();
  const user = useUserStore((state) => state.data);
  const { startStream } = useChatStream();
  const navigate = useNavigate();
  const sidebar = useSidebar();

  const onSendMessage = useCallback(
    async (msg: string) => {
      if (!user) return;
      const now = new Date().toISOString();

      const new_chat_id = nanoid();

      await rep.mutate.createChat({
        id: new_chat_id,
        title: "test",
        forked: false,
        archived: false,
        created_at: now,
        updated_at: now,
      });

      await rep.mutate.createMessage({
        id: nanoid(),
        chat_id: new_chat_id,
        role: "user",
        body: msg,
        created_at: now,
        updated_at: now,
      });

      startStream(new_chat_id);
      navigate(href("/chat/:thread_id", { thread_id: new_chat_id }));
    },
    [rep, user?.id]
  );

  return (
    <div className="h-full max-h-screen h-screen w-full mx-auto flex flex-col overflow-hidden">
      <SidebarTrigger
        variant="secondary"
        className={cn(
          `z-50 absolute opacity-100 top-3 left-3 transition-opacity duration-200`,
          sidebar.open && sidebar.isMobile && "opacity-0"
        )}
      />
      <div className="flex-1 overflow-y-auto px-4 py-4 space-y-10 custom-scrollbar"></div>
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
            <ChatInput handleSubmit={onSendMessage} disabled={false} />
          </div>
        </div>
      </div>
    </div>
  );
}
