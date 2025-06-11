import { ClipboardDocumentIcon } from "@heroicons/react/24/outline";
import { type FC, memo, useState } from "react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneLight } from "react-syntax-highlighter/dist/cjs/styles/prism";

interface Props {
  language: string;
  value: string;
}

const CustomPre = (props: any) => {
  return <pre {...props} className="custom-scrollbar" />;
};

export const CodeBlock: FC<Props> = memo(({ language, value }) => {
  const [isCopied, setIsCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(value);
    setIsCopied(true);
    setTimeout(() => {
      setIsCopied(false);
    }, 2000);
  };

  return (
    <div className="relative my-2 rounded-lg bg-muted font-sans text-sm">
      <div className="flex items-center justify-between rounded-t-lg bg-primary/80 px-4 py-1.5">
        <span className="text-xs text-primary-foreground">{language}</span>
        <button
          className="flex items-center gap-1.5 text-xs text-primary-foreground hover:font-semibold"
          onClick={handleCopy}
        >
          {isCopied ? (
            "Copied!"
          ) : (
            <>
              <ClipboardDocumentIcon className="h-4 w-4" />
              Copy code
            </>
          )}
        </button>
      </div>
      <SyntaxHighlighter
        language={language}
        style={oneLight}
        PreTag={CustomPre}
        customStyle={{
          margin: 0,
          padding: "1rem",
          borderBottomLeftRadius: "0.5rem",
          borderBottomRightRadius: "0.5rem",
        }}
        codeTagProps={{
          style: {
            fontFamily: "inherit",
          },
        }}
      >
        {value}
      </SyntaxHighlighter>
    </div>
  );
});
