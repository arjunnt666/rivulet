import type { RivuletDoc, VersionVector, Op } from "./index";

export interface SyncTransport {
  send(msg: unknown): Promise<void>;
  onMessage(cb: (msg: unknown) => void): void;
}

export async function pullMissing(
  doc: RivuletDoc,
  remoteVV: VersionVector,
  fetchOps: (have: VersionVector) => Promise<Op[]>
) {
  const missing = await fetchOps(doc.getVV());
  for (const op of missing) doc.apply(op);
  void remoteVV;
}
