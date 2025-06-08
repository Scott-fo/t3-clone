import {
  ArrowPathIcon,
  ArrowTopRightOnSquareIcon,
} from "@heroicons/react/24/outline";
import Markdown from "react-markdown";

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

export const MessageBubble = ({
  id,
  role,
  msg,
}: {
  id: string;
  role: "user" | "assistant";
  msg: string;
}) => {
  const isUser = role === "user";

  return (
    <div
      className={`flex flex-col max-w-3xl my-10 mx-auto ${
        isUser ? "items-end" : "items-start"
      }`}
    >
      <div
        className={`max-w-[75%] rounded-lg whitespace-pre-wrap p-3 ${
          isUser
            ? "bg-primary text-primary-foreground rounded-br-none"
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
          <div className="break-words">
            <Markdown
              components={{
                a: CitationLink,
                code({ className, children, ...props }) {
                  return (
                    <code
                      className={`break-words whitespace-pre-wrap ${className}`}
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
};
