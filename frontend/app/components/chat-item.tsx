import { href, Link, useNavigate } from "react-router";
import { Loader2, Pin, PinOff, SplitIcon, Trash2 } from "lucide-react";
import { SidebarMenuButton, SidebarMenuItem } from "~/components/ui/sidebar";
import { Button } from "~/components/ui/button";
import { TooltipContent, TooltipTrigger } from "~/components/ui/tooltip";
import type { Chat } from "~/domain/chat";
import { useChatStream } from "~/contexts/ChatStreamContext";
import * as TooltipPrimitive from "@radix-ui/react-tooltip";
import { Badge } from "./ui/badge";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "./ui/alert-dialog";
import { useCreateShare } from "~/hooks/use-shared-chat";
import { toast } from "sonner";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuTrigger,
} from "./ui/context-menu";
import { useState } from "react";

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
  const navigate = useNavigate();
  const { pendingResponses } = useChatStream();
  const isPending = !!pendingResponses[item.id];
  const [isDeleteAlertOpen, setIsDeleteAlertOpen] = useState(false);

  const shareMutation = useCreateShare();

  const handlePinClick = (e: React.MouseEvent | Event) => {
    e.preventDefault();
    onPin(item.id, !item.pinned);
  };

  const handleDeleteClick = (e: React.MouseEvent) => {
    e.preventDefault();
    onDelete(item.id);
    navigate(href("/"));
  };

  const handleShare = async () => {
    shareMutation.mutate(item.id, {
      onSuccess: (snapshot) => {
        const url = `${window.location.origin}/shared/${snapshot.id}`;
        navigator.clipboard.writeText(url).catch(() => {});
        toast.success("Share link copied to clipboard");
      },
      onError: () => toast.error("Failed to create share"),
    });
  };

  // can't get the tooltip to work on sidebarmenu button, so ill just wrap it
  return (
    <AlertDialog open={isDeleteAlertOpen} onOpenChange={setIsDeleteAlertOpen}>
      <ContextMenu>
        <ContextMenuTrigger asChild>
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
                      <span className="truncate ">
                        {item.title || "New chat"}
                      </span>
                    </>
                  </Link>
                </SidebarMenuButton>
              </TooltipTrigger>
              <TooltipContent side="right">
                {item.title || "New chat"}
              </TooltipContent>
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
                        onClick={() => setIsDeleteAlertOpen(true)}
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
        </ContextMenuTrigger>
        <ContextMenuContent className="w-48">
          <ContextMenuItem onSelect={handlePinClick}>
            {item.pinned ? "Unpin" : "Pin"}
          </ContextMenuItem>

          <ContextMenuItem onSelect={handleShare}>Share</ContextMenuItem>

          <ContextMenuSeparator />

          <ContextMenuItem
            variant="destructive"
            onSelect={() => setIsDeleteAlertOpen(true)}
          >
            <Trash2 className="h-4 w-4 text-destructive" />
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>

      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            This action cannot be undone. This will permanently delete this
            chat.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={handleDeleteClick}>
            Confirm
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
