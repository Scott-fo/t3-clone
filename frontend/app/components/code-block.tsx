import { ClipboardDocumentIcon } from "@heroicons/react/24/outline";
import { Highlight, themes } from "prism-react-renderer";
import { memo, useState } from "react";

interface Props {
  language: string;
  value: string;
}

export const CodeBlock = memo(({ language, value }: Props) => {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(value);
    setCopied(true);
    setTimeout(() => setCopied(false), 2_000);
  };

  return (
    <div className="relative my-2 rounded-lg bg-muted font-sans text-sm">
      <div className="flex items-center justify-between rounded-t-lg bg-primary/80 px-4 py-1.5">
        <span className="text-xs text-primary-foreground">
          {language || "text"}
        </span>

        <button
          onClick={handleCopy}
          className="flex items-center gap-1.5 text-xs text-primary-foreground hover:font-semibold"
        >
          {copied ? (
            "Copied!"
          ) : (
            <>
              <ClipboardDocumentIcon className="h-4 w-4" />
              Copy&nbsp;code
            </>
          )}
        </button>
      </div>

      <Highlight theme={themes.vsLight} code={value} language={language as any}>
        {({ className, style, tokens, getLineProps, getTokenProps }) => (
          <pre
            className={`custom-scrollbar overflow-auto m-0 rounded-b-lg p-4 ${className}`}
            style={{
              ...style,
              margin: 0,
              borderBottomLeftRadius: "0.5rem",
              borderBottomRightRadius: "0.5rem",
              background: "var(--sidebar)",
            }}
          >
            {tokens.map((line, i) => (
              <div key={i} {...getLineProps({ line })}>
                {line.map((token, key) => (
                  <span key={key} {...getTokenProps({ token })} />
                ))}
              </div>
            ))}
          </pre>
        )}
      </Highlight>
    </div>
  );
});
