import {
  ArrowPathIcon,
  ArrowTopRightOnSquareIcon,
} from "@heroicons/react/24/outline";
import { forwardRef } from "react";
import Markdown from "react-markdown";
import { CodeBlock } from "./code-block";
import remarkGfm from "remark-gfm";

interface Props {
  id: string;
  role: "user" | "assistant";
  msg: string;
}

interface ContentProps {
  msg: string;
}

const UserMessageContent = ({ msg }: ContentProps) => (
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
);

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

const MarkdownCodeRenderer = ({ node, className, children, ...props }: any) => {
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
};

const AssistantMessageContent = ({ id, msg }: { id: string; msg: string }) => {
  if (id === "pending" && msg === "") {
    return (
      <p className="flex items-center">
        <ArrowPathIcon className="mx-auto mr-2 h-4 w-4 animate-spin text-primary-500" />
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
    </div>
  );
};

export const MessageBubble = forwardRef<HTMLDivElement, Props>(
  ({ id, role, msg }, ref) => {
    const isUser = role === "user";

    const bubbleContainerClasses = `flex flex-col max-w-3xl min-h-20 my-10 mx-auto ${
      isUser ? "items-end" : "items-start"
    }`;

    const bubbleStyles = `rounded-lg whitespace-pre-wrap p-3 ${
      isUser
        ? "bg-primary max-w-[75%] text-primary-foreground rounded-br-none"
        : "bg-background text-foreground rounded-bl-none"
    }`;

    return (
      <div ref={ref} className={bubbleContainerClasses}>
        <div className={bubbleStyles}>
          {isUser ? (
            <UserMessageContent msg={msg} />
          ) : (
            <AssistantMessageContent id={id} msg={msg} />
          )}
        </div>
      </div>
    );
  }
);
