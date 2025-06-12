import { href, useNavigate } from "react-router";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "~/components/ui/command";
import type { Chat } from "~/domain/chat";

interface Props {
  open: boolean;
  setOpen: (b: boolean) => void;
  chats: Chat[];
}

export function CommandMenu({ open, setOpen, chats }: Props) {
  const navigate = useNavigate();

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>
        <CommandGroup heading="Chats">
          {chats.map((c) => (
            <CommandItem
              onSelect={() => {
                navigate(href("/chat/:thread_id", { thread_id: c.id }));
                setOpen(false);
              }}
            >
              {c.title}
            </CommandItem>
          ))}
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  );
}
