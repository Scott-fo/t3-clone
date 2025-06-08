import React, {
  useEffect,
  useLayoutEffect,
  useRef,
  forwardRef,
  type Ref,
} from "react";
import { Textarea } from "~/components/ui/textarea";
import { cn } from "~/lib/utils";

function useCombinedRefs<T>(...refs: Ref<T>[]): Ref<T> {
  const targetRef = useRef<T>(null);
  useEffect(() => {
    refs.forEach((ref) => {
      if (!ref) return;
      if (typeof ref === "function") {
        ref(targetRef.current);
      } else {
        (ref as React.RefObject<T | null>).current = targetRef.current;
      }
    });
  }, [refs]);
  return targetRef;
}

export const AutosizeTextarea = forwardRef<
  HTMLTextAreaElement,
  React.ComponentProps<typeof Textarea>
>(({ className, value, ...props }, forwardedRef) => {
  const internalRef = useRef<HTMLTextAreaElement>(null);
  const combinedRef = useCombinedRefs(internalRef, forwardedRef);

  const adjustHeight = () => {
    const ta = internalRef.current;
    if (!ta) return;
    ta.style.height = "auto";
    ta.style.height = `${ta.scrollHeight}px`;
  };

  useLayoutEffect(() => {
    adjustHeight();
  }, [value]);

  useEffect(() => {
    const ta = internalRef.current;
    if (!ta || typeof ResizeObserver === "undefined") return;

    const ro = new ResizeObserver(() => {
      adjustHeight();
    });
    ro.observe(ta);

    return () => {
      ro.disconnect();
    };
  }, []);

  return (
    <Textarea
      ref={combinedRef}
      value={value}
      className={cn("resize-none overflow-hidden", className)}
      {...props}
    />
  );
});

AutosizeTextarea.displayName = "AutosizeTextarea";
