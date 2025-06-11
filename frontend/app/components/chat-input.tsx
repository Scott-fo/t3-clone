import { ChevronsUpDown, Send } from "lucide-react";
import { useRef, useState } from "react";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "~/components/ui/popover";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "~/components/ui/tooltip";
import OpenAI from "~/logos/openai-black.svg?react";

import { AutosizeTextarea } from "~/components/ui/autosize-text-area";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { useActiveModelStore } from "~/stores/active-model";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { nanoid } from "nanoid";

interface Props {
  handleSubmit: (text: string) => Promise<void>;
  disabled: boolean;
}

// move this elsewhere and sync with server.
// do it with replicache, just add it to each cvr, then can have this synced
// from server
const modelList = [
  {
    provider: "openai",
    model: "o4-mini",
    display: "o4 mini",
    logo: OpenAI,
  },
  {
    provider: "openai",
    model: "gpt-4.1",
    display: "GPT 4.1",
    logo: OpenAI,
  },
  {
    provider: "openai",
    model: "gpt-4.1-mini",
    display: "GPT 4.1 mini",
    logo: OpenAI,
  },
];

function ChatInput({ disabled, handleSubmit }: Props) {
  const rep = useReplicache();
  const activeModel = useActiveModelStore((state) => state.data);

  const [messageValue, setMessageValue] = useState("");
  const [popoverOpen, setPopoverOpen] = useState(false);

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

  const activeModelDetails = activeModel
    ? modelList.find((m) => m.model === activeModel.model)
    : null;

  const ActiveLogo = activeModelDetails?.logo;

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
                  disabled={!messageValue?.trim() || disabled || !activeModel}
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
        <Popover open={popoverOpen} onOpenChange={setPopoverOpen}>
          <PopoverTrigger asChild>
            <Button
              variant="ghost"
              role="combobox"
              aria-expanded={popoverOpen}
              className="w-fit justify-between text-xs"
            >
              {ActiveLogo && <ActiveLogo className=" h-4 w-4" />}
              {activeModelDetails
                ? activeModelDetails.display
                : "Select a model"}
              <ChevronsUpDown className="ml-2 h-3 w-3 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[250px] p-0" side="top" align="start">
            {modelList.map(({ display, model, provider, logo: Logo }) => (
              <Button
                key={model}
                variant="ghost"
                className="w-full justify-start font-normal"
                onClick={() => {
                  if (activeModel) {
                    rep.mutate.updateActiveModel({
                      id: activeModel.id,
                      model,
                      provider,
                      updated_at: new Date().toISOString(),
                    });
                  } else {
                    rep.mutate.createActiveModel({
                      id: nanoid(),
                      model,
                      provider,
                      reasoning: null,
                      created_at: new Date().toISOString(),
                      updated_at: new Date().toISOString(),
                    });
                  }
                  setPopoverOpen(false);
                }}
              >
                <Logo />
                {display}
              </Button>
            ))}
          </PopoverContent>
        </Popover>
      </form>
    </div>
  );
}

export default ChatInput;
