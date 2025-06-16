import { memo, useRef } from "react";
import type { Route } from "./+types/shared";
import { useSharedChat } from "~/hooks/use-shared-chat";
import type { SharedChatWithMessages } from "~/domain/shared_chat";
import { MessageBubble } from "~/components/message-bubble";
import { Link } from "react-router";
import { BotMessageSquare } from "lucide-react";
import { Button } from "~/components/ui/button";

function Header() {
  return (
    <header className="sticky top-0 z-50 w-full border-b">
      <div className="container mx-auto flex h-14 max-w-3xl items-center justify-between px-4 sm:px-0">
        <Link to="/" className="flex items-center gap-2">
          <BotMessageSquare className="h-6 w-6" />
          <span className="hidden font-mono sm:inline-block">Shared Chat</span>
        </Link>

        <Button asChild size="sm">
          <Link to="/login">Login</Link>
        </Button>
      </div>
    </header>
  );
}

export default function Page({ params }: Route.ComponentProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  const { data } = useSharedChat(params.id);

  return (
    <div className="relative h-dvh w-full mx-auto flex flex-col overflow-hidden">
      <Header />
      <div
        ref={containerRef}
        className="z-10 flex-1 overflow-y-auto overflow-x-none px-4 py-4 space-y-2 custom-scrollbar"
      >
        {<MessageList sharedChatWithMessages={data} />}
      </div>
    </div>
  );
}

const MessageList = memo(
  ({
    sharedChatWithMessages,
  }: {
    sharedChatWithMessages: SharedChatWithMessages | undefined;
  }) => {
    if (!sharedChatWithMessages) {
      return;
    }

    return sharedChatWithMessages.messages.map((msg) => (
      <MessageBubble
        key={msg.id}
        id={msg.id}
        role={msg.role as any}
        msg={msg.body}
        reasoning={msg.reasoning}
      />
    ));
  }
);
