import { Send } from "lucide-react";
import { useRef, useState } from "react";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "~/components/ui/tooltip";

import { AutosizeTextarea } from "~/components/ui/autosize-text-area";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";

interface Props {
  handleSubmit: (text: string) => Promise<void>;
  disabled: boolean;
  chatId: string;
}

function ChatInput({ disabled, chatId, handleSubmit }: Props) {
  const [messageValue, setMessageValue] = useState("");

  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await submitMessage();
  };

  const submitMessage = async (): Promise<void> => {
    if (!messageValue.trim()) return;
    await handleSubmit(messageValue.trim());
    setMessageValue("");
  };

  return (
    <div>
      <form onSubmit={onSubmit} className="relative">
        <div className="flex w-full items-start overflow-hidden rounded-lg bg-primary-background p-2 custom-scrollbar">
          <AutosizeTextarea
            ref={textareaRef}
            id="chat-input"
            placeholder={"Write a message…"}
            value={messageValue}
            onChange={(e) => setMessageValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                submitMessage();
              }
            }}
            className={cn(
              "custom-scrollbar flex-1 resize-none border-0 bg-transparent shadow-none",
              "max-h-48 p-1",
              "overflow-y-auto",
              "focus:outline-none focus:ring-0 focus-visible:ring-0",
              "placeholder:text-muted-foreground",
              "disabled:cursor-not-allowed disabled:opacity-50"
            )}
            rows={2}
          />

          <div className={cn("flex items-center space-x-1 pl-1", "shrink-0")}>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  type="submit"
                  variant="default"
                  size="icon"
                  disabled={!messageValue?.trim() || disabled}
                  className="h-8 w-8 shrink-0 md:h-9 md:w-9"
                  aria-label="Send message"
                >
                  <Send className="h-4 w-4 md:h-5 md:w-5" />
                </Button>
              </TooltipTrigger>
              <TooltipContent side="top">Send message</TooltipContent>
            </Tooltip>
          </div>
        </div>
      </form>
    </div>
  );
}

export default ChatInput;
