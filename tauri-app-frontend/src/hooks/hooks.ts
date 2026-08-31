import { useEffect, useRef } from "react";

export function useTimeout(callback: () => void, delay: number | null): void {
  const savedCallback = useRef(callback);

  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  useEffect(() => {
    if (delay === null) return;

    const id = setTimeout(() => {
      savedCallback.current();
    }, delay);

    return () => clearTimeout(id);
  }, [delay]);
}

export function usePingListener(callback: () => void) {
  const serverUrl = "http://localhost:7878/ping/listen";
  useEffect(() => {
    const evListener = new EventSource(serverUrl)
    evListener.addEventListener("ping", callback)
  	return () => {
     evListener.close()
  	};
  }, [serverUrl]);
}
