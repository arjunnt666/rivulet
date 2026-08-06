import { RivuletClient } from "@rivulet/js";

const client = new RivuletClient();
const doc = client.open();
console.log("todo doc", doc.id);
