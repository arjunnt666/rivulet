import { useEffect, useState, useSyncExternalStore } from "react";
import type { RivuletDoc } from "@rivulet/js";

export function useDocVersion(doc: RivuletDoc): number {
  return useSyncExternalStore(
    (onStoreChange) => {
      const t = setInterval(onStoreChange, 500);
      return () => clearInterval(t);
    },
    () => doc.getOps().length
  );
}

export function usePresenceList(listFn: () => unknown[]) {
  const [list, setList] = useState(listFn);
  useEffect(() => {
    const t = setInterval(() => setList(listFn()), 300);
    return () => clearInterval(t);
  }, [listFn]);
  return list;
}
