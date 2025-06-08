import {
  type ReactNode,
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
  useCallback,
} from "react";

type SSEContextType = {
  isConnected: boolean;
  connectionState: "connecting" | "connected" | "disconnected" | "error";
  addEventListener: (type: string, handler: (data: any) => void) => void;
  removeEventListener: (type: string, handler: (data: any) => void) => void;
};

type SSEProviderProps = {
  userId: string;
  children: ReactNode;
};

const SSEContext = createContext<SSEContextType | null>(null);

export function useSSE(): SSEContextType {
  const context = useContext(SSEContext);
  if (!context) {
    throw new Error("useSSE must be used within SSEProvider");
  }
  return context;
}

export function SSEProvider({ children, userId }: SSEProviderProps) {
  const [connectionState, setConnectionState] = useState<
    "connecting" | "connected" | "disconnected" | "error"
  >("disconnected");
  const eventHandlersRef = useRef<Map<string, Set<(data: any) => void>>>(
    new Map()
  );
  const eventSourceRef = useRef<EventSource | null>(null);
  const registeredEventsRef = useRef<Set<string>>(new Set());

  const addEventListener = useCallback(
    (type: string, handler: (data: any) => void) => {
      if (!eventHandlersRef.current.has(type)) {
        eventHandlersRef.current.set(type, new Set());
      }
      eventHandlersRef.current.get(type)!.add(handler);

      if (eventSourceRef.current && !registeredEventsRef.current.has(type)) {
        eventSourceRef.current.addEventListener(type, (e: MessageEvent) => {
          handleEvent(type, e.data);
        });
        registeredEventsRef.current.add(type);
        console.log(`Registered SSE event listener for: ${type}`);
      }
    },
    []
  );

  const removeEventListener = useCallback(
    (type: string, handler: (data: any) => void) => {
      const handlers = eventHandlersRef.current.get(type);
      if (handlers) {
        handlers.delete(handler);
        if (handlers.size === 0) {
          eventHandlersRef.current.delete(type);
        }
      }
    },
    []
  );

  const handleEvent = useCallback((type: string, eventData: string) => {
    try {
      const data = eventData ? JSON.parse(eventData) : null;

      const handlers = eventHandlersRef.current.get(type);
      if (handlers) {
        handlers.forEach((handler) => handler(data));
      }
    } catch (error) {
      console.error(`Failed to parse SSE ${type} data:`, error);
    }
  }, []);

  useEffect(() => {
    console.log("Connecting to SSE");
    setConnectionState("connecting");

    const es = new EventSource("/api/sse");
    eventSourceRef.current = es;

    es.onopen = () => {
      console.log("SSE connection opened");
      setConnectionState("connected");
    };

    es.onerror = (err) => {
      console.error("SSE error", err);
      setConnectionState("error");
    };

    eventHandlersRef.current.forEach((_, type) => {
      if (!registeredEventsRef.current.has(type)) {
        es.addEventListener(type, (e: MessageEvent) => {
          handleEvent(type, e.data);
        });
        registeredEventsRef.current.add(type);
        console.log(`Registered SSE event listener for: ${type}`);
      }
    });

    return () => {
      console.log("Closing SSE connection");
      setConnectionState("disconnected");
      eventSourceRef.current = null;
      registeredEventsRef.current.clear();
      es.close();
    };
  }, [handleEvent, userId]);

  return (
    <SSEContext.Provider
      value={{
        isConnected: connectionState === "connected",
        connectionState,
        addEventListener,
        removeEventListener,
      }}
    >
      {children}
    </SSEContext.Provider>
  );
}
