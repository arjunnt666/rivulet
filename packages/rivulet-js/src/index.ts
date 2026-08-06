export type ActorId = string;
export type DocId = string;

export interface VersionVector { [actorId: string]: number }

export interface Op {
  id: { actor: ActorId; counter: number };
  payload: unknown;
}

export class RivuletDoc {
  readonly id: DocId;
  private vv: VersionVector = {};
  private ops: Op[] = [];

  constructor(id?: DocId) {
    this.id = id ?? crypto.randomUUID();
  }

  apply(op: Op) {
    this.ops.push(op);
    const a = op.id.actor;
    this.vv[a] = Math.max(this.vv[a] ?? 0, op.id.counter);
  }

  getOps(): readonly Op[] { return this.ops; }
  getVV(): VersionVector { return { ...this.vv }; }
}

export class RivuletClient {
  private docs = new Map<DocId, RivuletDoc>();

  open(id?: DocId): RivuletDoc {
    const doc = new RivuletDoc(id);
    this.docs.set(doc.id, doc);
    return doc;
  }

  get(id: DocId): RivuletDoc | undefined {
    return this.docs.get(id);
  }
}
