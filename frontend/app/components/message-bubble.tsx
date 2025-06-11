import {
  ArrowPathIcon,
  ArrowTopRightOnSquareIcon,
} from "@heroicons/react/24/outline";
import { forwardRef, memo } from "react";
import Markdown from "react-markdown";
import { CodeBlock } from "./code-block";
import remarkGfm from "remark-gfm";
import { SplitIcon } from "lucide-react";
import { Button } from "./ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip";
import { useChatStore } from "~/stores/chat";
import { useMessageStore } from "~/stores/message";
import type { Chat } from "~/domain/chat";
import type { Message } from "~/domain/message";
import { nanoid } from "nanoid";
import {
  useReplicache,
  type ReplicacheType,
} from "~/contexts/ReplicacheContext";
import type { Replicache } from "replicache";

interface Props {
  id: string;
  role: "user" | "assistant";
  msg: string;
  chat_id: string;
}

interface ContentProps {
  msg: string;
}

const UserMessageContent = memo(({ msg }: ContentProps) => (
  <Markdown
    remarkPlugins={[remarkGfm]}
    components={{
      a: CitationLink,
      code({ className, children }) {
        return (
          <CodeBlock
            language="text"
            value={String(children).replace(/\n$/, "")}
          />
        );
      },
    }}
  >
    {msg}
  </Markdown>
));

const CitationLink = ({
  href,
  children,
  ...props
}: React.AnchorHTMLAttributes<HTMLAnchorElement>) => {
  const isExternalLink = href?.startsWith("http");

  if (isExternalLink) {
    return (
      <a
        href={href}
        target="_blank"
        rel="noopener noreferrer"
        className={`inline-flex items-center gap-1 underline hover:no-underline transition-colors text-primary`}
        {...props}
      >
        {children}
        <ArrowTopRightOnSquareIcon className="h-3 w-3 flex-shrink-0" />
      </a>
    );
  }

  return (
    <a href={href} className="text-primary underline" {...props}>
      {children}
    </a>
  );
};

const MarkdownCodeRenderer = memo(
  ({ node, className, children, ...props }: any) => {
    const match = /language-(\w+)/.exec(className || "");
    return match ? (
      <CodeBlock
        language={match[1]}
        value={String(children).replace(/\n$/, "")}
      />
    ) : (
      <code
        className="custom-scrollbar break-words rounded-md bg-secondary px-1.5 py-0.5 font-mono text-sm font-semibold text-secondary-foreground"
        {...props}
      >
        {children}
      </code>
    );
  }
);

const forkChat = (
  rep: Replicache<ReplicacheType>,
  break_id: string,
  chat: Chat,
  msgs: Message[]
) => {
  const new_id = nanoid();

  const idx = msgs.findIndex((m) => m.id === break_id);
  const new_msgs = msgs.slice(0, idx + 1).map((m) => ({
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
};

const AssistantMessageContent = memo(
  ({ id, chat_id, msg }: { id: string; chat_id: string; msg: string }) => {
    const chat = useChatStore((state) =>
      state.data.find((c) => c.id === chat_id)
    );
    const msgs = useMessageStore((state) => state.data);
    const rep = useReplicache();

    if (id === "pending" && msg === "") {
      return (
        <p className="flex items-center">
          <ArrowPathIcon className="mr-2 h-4 w-4 animate-spin text-primary-500" />
          Thinking...
        </p>
      );
    }

    if (msg.startsWith("Error: ")) {
      return <span className="text-red-600">{msg}</span>;
    }

    return (
      <div className="prose break-words">
        <Markdown
          remarkPlugins={[remarkGfm]}
          components={{
            a: CitationLink,
            code: MarkdownCodeRenderer,
          }}
        >
          {msg}
        </Markdown>
        {id !== "pending" && (
          <div className="mt-10">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  size="icon"
                  variant="secondary"
                  onClick={() => forkChat(rep, id, chat as Chat, msgs)}
                  disabled={!chat}
                >
                  <SplitIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>Fork chat</TooltipContent>
            </Tooltip>
          </div>
        )}
      </div>
    );
  }
);

export const MessageBubble = memo(
  forwardRef<HTMLDivElement, Props>(({ id, chat_id, role, msg }, ref) => {
    const isUser = role === "user";

    const bubbleContainerClasses = `flex flex-col max-w-3xl min-h-20 my-10 mx-auto ${
      isUser ? "items-end" : "items-start"
    }`;

    const bubbleStyles = `rounded-lg whitespace-pre-wrap p-3 ${
      isUser
        ? "bg-primary max-w-[75%] text-primary-foreground rounded-br-none"
        : "w-[100%] break-words bg-background text-foreground rounded-bl-none"
    }`;

    return (
      <div ref={ref} className={bubbleContainerClasses}>
        <div className={bubbleStyles}>
          {isUser ? (
            <UserMessageContent msg={msg} />
          ) : (
            <AssistantMessageContent id={id} chat_id={chat_id} msg={msg} />
          )}
        </div>
      </div>
    );
  })
);
