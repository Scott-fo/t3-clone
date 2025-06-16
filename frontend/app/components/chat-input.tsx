import {
  BrainIcon,
  ChevronsUpDown,
  Send,
  SignalHigh,
  SignalLow,
  SignalMedium,
} from "lucide-react";
import { forwardRef, useRef, useState } from "react";
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

import { AutosizeTextarea } from "~/components/ui/autosize-text-area";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";
import { useActiveModelStore } from "~/stores/active-model";
import { useReplicache } from "~/contexts/ReplicacheContext";
import { nanoid } from "nanoid";
import { supportedModels, type Reasoning } from "~/domain/ai";
import { useSidebar } from "./ui/sidebar";

interface Props {
  handleSubmit: (text: string) => Promise<void>;
  disabled: boolean;
}

const reasoningLevelIcon = {
  low: SignalLow,
  medium: SignalMedium,
  high: SignalHigh,
};

const ChatInput = forwardRef<HTMLTextAreaElement, Props>(
  ({ disabled, handleSubmit }, ref) => {
    const rep = useReplicache();
    const { isMobile } = useSidebar();
    const activeModel = useActiveModelStore((state) => state.data);

    const [messageValue, setMessageValue] = useState("");
    const [popoverOpen, setPopoverOpen] = useState(false);
    const [reasoningPopoverOpen, setReasoningPopoverOpen] = useState(false);

    const onSubmit = async (e: React.FormEvent) => {
      e.preventDefault();
      await submitMessage();
    };

    const submitMessage = async (): Promise<void> => {
      if (!messageValue.trim()) return;
      await handleSubmit(messageValue.trim());
      setMessageValue("");
    };

    const provider = activeModel
      ? supportedModels.find((sm) => sm.provider === activeModel.provider)
      : null;

    const activeModelDetails = provider
      ? provider.models.find((m) => m.model === activeModel!.model)
      : null;

    const ActiveLogo = provider?.logo;

    const ActiveReasoningIcon = activeModel?.reasoning
      ? reasoningLevelIcon[activeModel.reasoning]
      : BrainIcon;

    return (
      <div>
        <form onSubmit={onSubmit} className="relative">
          <div className="flex w-full items-start overflow-hidden rounded-lg bg-primary-background p-2 custom-scrollbar">
            <AutosizeTextarea
              ref={ref}
              id="chat-input"
              placeholder={"Write a message…"}
              value={messageValue}
              onChange={(e) => setMessageValue(e.target.value)}
              onKeyDown={(e) => {
                if (
                  e.key === "Enter" &&
                  !e.shiftKey &&
                  !isMobile &&
                  !disabled
                ) {
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
          <div className="mt-2 flex flex-wrap items-center">
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
              <PopoverContent
                className="w-[250px] p-0"
                side="top"
                align="start"
              >
                {supportedModels.flatMap(({ provider, logo: Logo, models }) =>
                  models.map(({ model, display, reasoning }) => (
                    <Button
                      key={model}
                      variant="ghost"
                      className="w-full justify-start font-normal"
                      onClick={() => {
                        if (activeModel) {
                          const update = {
                            id: activeModel.id,
                            model,
                            provider,
                            reasoning: null as Reasoning | null,
                            updated_at: new Date().toISOString(),
                          };

                          if (reasoning) {
                            update.reasoning = "high";
                          }

                          rep.mutate.updateActiveModel(update);
                        } else {
                          const create = {
                            id: nanoid(),
                            model,
                            provider,
                            reasoning: null as Reasoning | null,
                            created_at: new Date().toISOString(),
                            updated_at: new Date().toISOString(),
                          };

                          if (reasoning) {
                            create.reasoning = "high";
                          }

                          rep.mutate.createActiveModel(create);
                        }
                        setPopoverOpen(false);
                      }}
                    >
                      <Logo />
                      {display}
                      {reasoning && <BrainIcon className="size-3 ml-auto" />}
                    </Button>
                  ))
                )}
              </PopoverContent>
            </Popover>
            {activeModel?.reasoning && (
              <Popover
                open={reasoningPopoverOpen}
                onOpenChange={setReasoningPopoverOpen}
              >
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    role="combobox"
                    aria-expanded={reasoningPopoverOpen}
                    className="w-fit justify-between text-xs capitalize"
                  >
                    <div className="flex items-center">
                      <ActiveReasoningIcon className="mr-2 h-4 w-4" />
                      {activeModel?.reasoning}
                    </div>
                    <ChevronsUpDown className="ml-2 h-3 w-3 shrink-0 opacity-50" />
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0" side="top" align="start">
                  {["high", "medium", "low"].map((effort) => {
                    const Icon =
                      reasoningLevelIcon[
                        effort as keyof typeof reasoningLevelIcon
                      ];
                    return (
                      <Button
                        key={effort}
                        variant="ghost"
                        className="w-full justify-start font-normal capitalize"
                        onClick={() => {
                          rep.mutate.updateActiveModel({
                            id: activeModel.id,
                            provider: activeModel.provider,
                            model: activeModel.model,
                            reasoning: effort as Reasoning,
                            updated_at: new Date().toISOString(),
                          });
                          setReasoningPopoverOpen(false);
                        }}
                      >
                        <Icon className="mr-2 h-4 w-4" />
                        {effort}
                      </Button>
                    );
                  })}
                </PopoverContent>
              </Popover>
            )}
          </div>
        </form>
      </div>
    );
  }
);

export default ChatInput;
