//! Provides a wrapper over `snake.move_forward` method that
//! causes movement in the specified direction (which is a state).

use common::world::types::GridPos;

use crate::entity::{EntityId, EntityManager};

#[derive(Debug, Clone)]
pub enum MovementEvent {
    /// Сущность сдвинулась на одну клетку.
    EntityMoved {
        entity_id: EntityId,
        new_head: GridPos,
        removed_tail: GridPos,
    },
}

pub struct MovementSystem;

impl MovementSystem {
    /// Двигает все сущности и генерирует события об их перемещении.
    pub fn tick(entities: &mut EntityManager, events_bus: &mut Vec<MovementEvent>) {
        // Проходимся по всем змейкам, чтобы их подвинуть
        for (entity_id, snake) in entities.entities.iter_mut() {
            // 1. Запоминаем позицию хвоста ДО его удаления
            // .cloned() нужен, так как .back() возвращает ссылку, а нам нужно владение.
            // Если у змейки нет тела, то и хвоста нет.
            if let Some(old_tail_pos) = snake.body.back().cloned() {
                
                // 2. Выполняем само движение (добавляется голова, удаляется хвост)
                snake.move_forward();

                // 3. Получаем позицию новой головы
                // .unwrap() здесь безопасен, так как move_forward гарантирует наличие головы
                let new_head_pos = snake.body.front().unwrap().clone();
                
                // 4. Создаем и добавляем событие в шину
                events_bus.push(MovementEvent::EntityMoved {
                    entity_id: *entity_id,
                    new_head: new_head_pos,
                    removed_tail: old_tail_pos,
                });
            }
        }
    }
}