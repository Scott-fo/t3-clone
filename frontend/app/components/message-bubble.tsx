import {
  ArrowPathIcon,
  ArrowTopRightOnSquareIcon,
  ClipboardDocumentIcon,
} from "@heroicons/react/24/outline";
import { forwardRef, memo, useState } from "react";
import Markdown from "react-markdown";
import { CodeBlock } from "./code-block";
import remarkGfm from "remark-gfm";
import {
  CheckIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  Loader2,
  SplitIcon,
} from "lucide-react";
import { Button } from "./ui/button";
import { TooltipContent, TooltipProvider, TooltipTrigger } from "./ui/tooltip";
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
import { usePreferencesStore } from "~/stores/preferences";
import { cn } from "~/lib/utils";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";

interface Props {
  id: string;
  role: "user" | "assistant";
  msg: string;
  reasoning?: string | null;
  chat_id: string;
}

interface ContentProps {
  msg: string;
}

const CopyButton = ({
  textToCopy,
  variant = "secondary",
  className,
}: {
  textToCopy: string;
  variant?: "secondary" | "ghost";
  className?: string;
}) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(textToCopy);
    setCopied(true);
    setTimeout(() => {
      setCopied(false);
    }, 2000);
  };

  return (
    <TooltipPrimitive.Root>
      <TooltipTrigger asChild>
        <Button
          size="icon"
          variant={variant}
          onClick={handleCopy}
          className={className}
        >
          {copied ? (
            <CheckIcon className="h-4 w-4 text-green-500" />
          ) : (
            <ClipboardDocumentIcon className="h-4 w-4" />
          )}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{copied ? "Copied!" : "Copy message"}</TooltipContent>
    </TooltipPrimitive.Root>
  );
};

const UserMessageContent = memo(({ msg }: ContentProps) => (
  <div>
    <Markdown
      remarkPlugins={[remarkGfm]}
      components={{
        a: CitationLink,
        code({ children }) {
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
  </div>
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

const ReasoningView = memo(
  ({ id, reasoning }: { id: string; reasoning: string }) => {
    const isPending = id === "pending";

    const prefs = usePreferencesStore((state) => state.data);
    const setPrefs = usePreferencesStore((state) => state.setData);

    const toggle = () => {
      setPrefs({ ...prefs, showReasoning: !prefs.showReasoning });
    };

    return (
      <div className="w-full mb-6">
        <button
          onClick={() => toggle()}
          className="flex items-center gap-1 text-sm text-muted-foreground transition-colors"
        >
          {prefs.showReasoning ? (
            <ChevronDownIcon className="h-4 w-4" />
          ) : (
            <ChevronRightIcon className="h-4 w-4" />
          )}
          {isPending ? (
            <>
              <span>Reasoning</span>
              <Loader2 className="animate-spin size-4" />
            </>
          ) : (
            <span>Reasoning</span>
          )}
        </button>
        {prefs.showReasoning && (
          <div className="mt-2 p-3 border rounded-lg bg-muted/50 text-sm">
            <Markdown>{reasoning}</Markdown>
          </div>
        )}
      </div>
    );
  }
);

const AssistantMessageContent = memo(
  ({
    id,
    chat_id,
    reasoning,
    msg,
  }: {
    id: string;
    chat_id: string;
    msg: string;
    reasoning?: string | null;
  }) => {
    const chat = useChatStore((state) =>
      state.data.find((c) => c.id === chat_id)
    );
    const msgs = useMessageStore((state) => state.data);
    const rep = useReplicache();

    if (id === "pending" && msg === "" && !reasoning) {
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

    // way too much vertical space. looks like a styling thing
    return (
      <div className="prose-sm break-words group/message">
        <Markdown
          remarkPlugins={[remarkGfm]}
          components={{
            a: CitationLink,
            code: MarkdownCodeRenderer,
            p: ({ node, ...props }) => (
              <p {...props} className="m-0 p-0 leading-snug" />
            ),
            li: ({ node, ...props }) => (
              <li {...props} className="m-0 p-0 leading-snug" />
            ),
            ol: ({ node, ...props }) => (
              <ol {...props} className="m-0 p-0 leading-snug" />
            ),
            table: ({ node, ...props }) => (
              <div className="overflow-x-auto">
                <table {...props} className="my-4 w-full border-collapse" />
              </div>
            ),
            thead: ({ node, ...props }) => <thead {...props} />,
            tbody: ({ node, ...props }) => <tbody {...props} />,
            tr: ({ node, ...props }) => (
              <tr {...props} className="border-b even:bg-muted/50" />
            ),
            th: ({ node, ...props }) => (
              <th
                {...props}
                className="p-2 text-left font-semibold border border-border"
              />
            ),
            td: ({ node, ...props }) => (
              <td {...props} className="p-2 border border-border" />
            ),
          }}
        >
          {msg}
        </Markdown>
        {id !== "pending" && (
          <div className="pt-4 flex items-center gap-x-2 opacity-0 group-hover/message:opacity-100 transition-opacity duration-200">
            <TooltipProvider delayDuration={300}>
              <TooltipPrimitive.Root>
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
              </TooltipPrimitive.Root>
              <CopyButton textToCopy={msg} />
            </TooltipProvider>
          </div>
        )}
      </div>
    );
  }
);

export const MessageBubble = memo(
  forwardRef<HTMLDivElement, Props>(
    ({ id, chat_id, role, msg, reasoning }, ref) => {
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
        <div ref={ref} className={cn(bubbleContainerClasses, "group/message")}>
          <div className={bubbleStyles}>
            {isUser ? (
              <UserMessageContent msg={msg} />
            ) : (
              <div>
                {reasoning && <ReasoningView id={id} reasoning={reasoning} />}
                <AssistantMessageContent
                  id={id}
                  chat_id={chat_id}
                  msg={msg}
                  reasoning={reasoning}
                />
              </div>
            )}
          </div>
          {isUser && (
            <div className="pt-4 flex justify-end opacity-0 transition-opacity duration-200 group-hover/message:opacity-100">
              <TooltipProvider delayDuration={300}>
                <CopyButton textToCopy={msg} variant="secondary" />
              </TooltipProvider>
            </div>
          )}
        </div>
      );
    }
  )
);
