import {
  ArrowPathIcon,
  ArrowTopRightOnSquareIcon,
} from "@heroicons/react/24/outline";
import { forwardRef } from "react";
import Markdown from "react-markdown";
import { CodeBlock } from "./code-block";
import remarkGfm from "remark-gfm";

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

interface Props {
  id: string;
  role: "user" | "assistant";
  msg: string;
}

export const MessageBubble = forwardRef<HTMLDivElement, Props>(
  ({ id, role, msg }, ref) => {
    const isUser = role === "user";

    return (
      <div
        ref={ref}
        className={`flex flex-col max-w-3xl min-h-20 my-10 mx-auto ${
          isUser ? "items-end" : "items-start"
        }`}
      >
        <div
          className={`rounded-lg whitespace-pre-wrap p-3 ${
            isUser
              ? "bg-primary max-w-[75%] text-primary-foreground rounded-br-none"
              : "bg-background text-foreground rounded-bl-none"
          }`}
        >
          {!isUser && msg === "" && id === "pending" ? (
            <p className="flex items-center">
              <ArrowPathIcon className="mx-auto mr-2 h-4 w-4 animate-spin text-primary-500" />
              Thinking...
            </p>
          ) : !isUser && msg.startsWith("Error: ") ? (
            <span className="text-red-600">{msg}</span>
          ) : (
            <div className="prose break-words">
              <Markdown
                remarkPlugins={[remarkGfm]}
                components={{
                  a: CitationLink,
                  code({ node, className, children, ...props }) {
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
                  },
                }}
              >
                {msg}
              </Markdown>
            </div>
          )}
        </div>
      </div>
    );
  }
);
