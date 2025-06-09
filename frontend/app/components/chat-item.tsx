import { href, Link } from "react-router";
import { Pin, PinOff, Trash2 } from "lucide-react";
import { SidebarMenuButton, SidebarMenuItem } from "~/components/ui/sidebar";
import { Button } from "~/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "~/components/ui/tooltip";
import type { Chat } from "~/domain/chat";

interface ChatItemProps {
  item: Chat;
  isActive: boolean;
  onPin: (id: string, pinned: boolean) => void;
  onDelete: (id: string) => void;
}

export function ChatItem({ item, isActive, onPin, onDelete }: ChatItemProps) {
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

  return (
    <SidebarMenuItem className="group/chat relative">
      <SidebarMenuButton isActive={isActive} asChild>
        <Link to={href("/chat/:thread_id", { thread_id: item.id })}>
          <span className="truncate pr-10">{item.title || "New chat"}</span>
        </Link>
      </SidebarMenuButton>

      <div
        className="
          absolute right-0 top-0 h-full
          flex items-center justify-end
          bg-gradient-to-l from-sidebar-accent via-sidebar-accent/80 to-transparent
          pl-10 opacity-0 transition-opacity duration-200
          group-hover/chat:opacity-100"
      >
        <TooltipProvider delayDuration={300}>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
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
              {item.pinned ? "Unpin chat" : "Pin chat"}
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7"
                onClick={handleDeleteClick}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="top">Delete chat</TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>
    </SidebarMenuItem>
  );
}
