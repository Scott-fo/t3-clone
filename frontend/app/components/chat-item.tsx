import { href, Link } from "react-router";
import { Loader2, Pin, PinOff, SplitIcon, Trash2 } from "lucide-react";
import { SidebarMenuButton, SidebarMenuItem } from "~/components/ui/sidebar";
import { Button } from "~/components/ui/button";
import { TooltipContent, TooltipTrigger } from "~/components/ui/tooltip";
import type { Chat } from "~/domain/chat";
import { useChatStream } from "~/contexts/ChatStreamContext";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { Badge } from "./ui/badge";

interface ChatItemProps {
  item: Chat;
  isActive: boolean;
  pinIndex?: number;
  onPin: (id: string, pinned: boolean) => void;
  onDelete: (id: string) => void;
}

export function ChatItem({
  item,
  isActive,
  onPin,
  pinIndex,
  onDelete,
}: ChatItemProps) {
  const { pendingResponses } = useChatStream();
  const isPending = !!pendingResponses[item.id];

  const handlePinClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    e.preventDefault();
    onPin(item.id, !item.pinned);
  };

  const handleDeleteClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    e.preventDefault();
    onDelete(item.id);
  };

  // can't get the tooltip to work on sidebarmenu button, so ill just wrap it
  return (
    <SidebarMenuItem className="group/chat relative">
      <TooltipPrimitive.Root>
        <TooltipTrigger asChild>
          <SidebarMenuButton isActive={isActive} asChild>
            <Link to={href("/chat/:thread_id", { thread_id: item.id })}>
              <>
                {pinIndex && (
                  <Badge className="w-6" variant="default">
                    {pinIndex}
                  </Badge>
                )}
                {item.forked && <SplitIcon />}
                <span className="truncate ">{item.title || "New chat"}</span>
              </>
            </Link>
          </SidebarMenuButton>
        </TooltipTrigger>
        <TooltipContent side="right">{item.title || "New chat"}</TooltipContent>
      </TooltipPrimitive.Root>

      <div
        className={`
          absolute right-0 top-0 h-full w-1/2
          pl-2 flex items-center justify-end
          bg-gradient-to-l from-sidebar-accent via-sidebar-accent/90 to-sidebar-accent/20
          transition-opacity duration-200
          ${
            isPending
              ? "opacity-100"
              : "opacity-0 group-hover/chat:opacity-100 pointer-events-none"
          }
        `}
      >
        {isPending ? (
          <Loader2 className="h-4 w-4 mr-2 text-primary animate-spin" />
        ) : (
          <>
            <TooltipPrimitive.Root>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-7 w-7 hover:bg-primary/60 hover:text-white pointer-events-auto"
                  onClick={handlePinClick}
                >
                  {item.pinned ? (
                    <PinOff className="h-4 w-4" />
                  ) : (
                    <Pin className="h-4 w-4" />
                  )}
                </Button>
              </TooltipTrigger>
              <TooltipContent side="top">
                {item.pinned ? "Remove from hotbar" : "Add to hotbar"}
              </TooltipContent>
            </TooltipPrimitive.Root>
            <TooltipPrimitive.Root>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-7 w-7 hover:bg-destructive/80 hover:text-white pointer-events-auto"
                  onClick={handleDeleteClick}
                >
                  <Trash2 className="h-4 w-4" />
                </Button>
              </TooltipTrigger>
              <TooltipContent
                side="top"
                className="bg-destructive/80 text-white fill-destructive/80"
              >
                Delete chat
              </TooltipContent>
            </TooltipPrimitive.Root>
          </>
        )}
      </div>
    </SidebarMenuItem>
  );
}
