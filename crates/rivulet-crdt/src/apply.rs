use crate::types::*;
use rivulet_core::op::{Op, OpPayload};

pub fn apply_op(map: &mut LwwMap, text: &mut RgaText, list: &mut OrList, counter: &mut PnCounter, op: &Op) {
    match &op.payload {
        OpPayload::MapSet { key, value } => map.set(key, value.clone(), op.id.dot),
        OpPayload::MapDelete { key } => map.delete(key, op.id.dot),
        OpPayload::TextInsert { pos, content } => text.insert(pos.clone(), content, op.id.dot),
        OpPayload::TextDelete { start, end } => text.delete_range(start, end),
        OpPayload::ListInsert { index_hint, item_id, value } => {
            list.insert(item_id.clone(), value.clone(), op.id.dot, *index_hint as usize)
        }
        OpPayload::ListRemove { item_id } => list.remove(item_id, op.id.dot),
        OpPayload::CounterAdd { delta } => counter.add(op.actor, *delta),
        OpPayload::Extension { .. } => {}
    }
}
