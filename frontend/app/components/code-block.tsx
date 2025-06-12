import { ClipboardDocumentIcon } from "@heroicons/react/24/outline";
import { Highlight, themes } from "prism-react-renderer";
import { memo, useState } from "react";
import { Button } from "./ui/button";
import { CheckIcon, TextIcon, WrapTextIcon } from "lucide-react";
import { cn } from "~/lib/utils";
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip";
import { usePreferencesStore } from "~/stores/preferences";

interface Props {
  language: string;
  value: string;
}

export const CodeBlock = memo(({ language, value }: Props) => {
  const [copied, setCopied] = useState(false);
  const prefs = usePreferencesStore((state) => state.data);
  const setPrefs = usePreferencesStore((state) => state.setData);

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

        <div>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                onClick={() =>
                  setPrefs({ ...prefs, wrapText: !prefs.wrapText })
                }
                variant="ghost"
                size="icon"
                className="size-7 text-primary-foreground"
              >
                {prefs.wrapText ? (
                  <TextIcon className="h-4 w-4" />
                ) : (
                  <WrapTextIcon className="h-4 w-4" />
                )}
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              {prefs.wrapText
                ? "Disable text wrapping"
                : "Enable text wrapping"}
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                onClick={handleCopy}
                variant="ghost"
                size="icon"
                className="size-7 text-primary-foreground"
              >
                {copied ? (
                  <CheckIcon className="h-4 w-4" />
                ) : (
                  <ClipboardDocumentIcon className="h-4 w-4" />
                )}
              </Button>
            </TooltipTrigger>
            <TooltipContent>Copy</TooltipContent>
          </Tooltip>
        </div>
      </div>

      <Highlight theme={themes.vsLight} code={value} language={language as any}>
        {({ className, style, tokens, getLineProps, getTokenProps }) => (
          <pre
            className={cn(
              `custom-scrollbar overflow-auto m-0 rounded-b-lg p-4 ${className}`,
              prefs.wrapText && "text-wrap"
            )}
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
