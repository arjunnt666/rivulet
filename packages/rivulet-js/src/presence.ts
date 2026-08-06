export interface PresenceCursor {
  actorId: string;
  path?: string;
  anchor?: number;
  meta?: Record<string, unknown>;
}

export class PresenceRoom {
  private peers = new Map<string, PresenceCursor>();

  update(cursor: PresenceCursor) { this.peers.set(cursor.actorId, cursor); }
  leave(actorId: string) { this.peers.delete(actorId); }
  list(): PresenceCursor[] { return [...this.peers.values()]; }
}
