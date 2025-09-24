//! Provides processing of physical interactions in the game.

// --- Ваши импорты ---
use std::collections::HashSet;
use common::{entities::snake::Snake, world::world::World}; // World все еще нужен для chunk_at
use crate::{
    entity::{EntityId, EntityManager},
    systems::presence::PresenceSystem,
};
use rand::Rng; // Для случайного выбора при столкновении лбами

/// Событие, генерируемое физической системой.
/// Сообщает о том, какая сущность должна быть уничтожена.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PhysicsEvent {
    EntityDied(EntityId),
}

pub struct PhysicsSystem;

impl PhysicsSystem {
    /// Выполняет проверку столкновений, определяет, какие сущности должны умереть,
    /// генерирует события смерти и обновляет PresenceSystem.
    pub fn tick(
        presence_system: &mut PresenceSystem,
        entities: &EntityManager,
        world: &World, // world нужен для вызова world.chunk_at()
        events_bus: &mut Vec<PhysicsEvent>,
    ) {
        // Используем HashSet, чтобы избежать дублирования событий смерти для одной и той же сущности
        let mut entities_to_remove: HashSet<EntityId> = HashSet::new();

        for entities_in_chunk in presence_system.presence_map.values() {
            // Проход для определения, кто должен умереть
            for i in 0..entities_in_chunk.len() {
                let entity_a_id = entities_in_chunk[i];

                // Если сущность уже помечена на удаление, пропускаем ее
                if entities_to_remove.contains(&entity_a_id) {
                    continue;
                }

                if let Some(snake_a) = entities.get(&entity_a_id) {
                    // 1. Проверка на поедание самого себя
                    if Self::check_self_collision(snake_a) {
                        entities_to_remove.insert(entity_a_id);
                        continue; // Переходим к следующей сущности
                    }

                    // 2. Проверка столкновений с другими
                    for j in (i + 1)..entities_in_chunk.len() {
                        let entity_b_id = entities_in_chunk[j];
                        if entities_to_remove.contains(&entity_b_id) {
                            continue;
                        }

                        if let Some(snake_b) = entities.get(&entity_b_id) {
                            for dead_id in Self::get_dead_entities_on_collision(
                                entity_a_id,
                                snake_a,
                                entity_b_id,
                                snake_b,
                            ) {
                                entities_to_remove.insert(dead_id);
                            }
                        }
                    }
                }
            }
        }

        // --- Финальная обработка ---
        // Генерируем события и чистим PresenceSystem
        for entity_id in entities_to_remove {
            // 1. Добавляем событие в шину для дальнейшей обработки (например, в EntityManager)
            events_bus.push(PhysicsEvent::EntityDied(entity_id));

            // 2. Удаляем сущность из PresenceSystem (как и было запрошено)
            if let Some(snake_to_remove) = entities.get(&entity_id) {
                for body_part in snake_to_remove.body.iter() {
                    let chunk_id = world.chunk_at(body_part); // body_pos должен быть GridPos
                    presence_system.remove_entity_from_chunk(chunk_id, &entity_id);
                }
            }
        }
    }

    /// Проверяет, столкнулась ли голова змейки с её телом.
    fn check_self_collision(snake: &Snake) -> bool {
        if let Some(head) = snake.body.front() {
            return snake.body.iter().skip(1).any(|part| part == head);
        }
        false
    }

    /// Определяет, какие сущности умирают при столкновении, согласно правилам.
    fn get_dead_entities_on_collision(
        id_a: EntityId,
        snake_a: &Snake,
        id_b: EntityId,
        snake_b: &Snake,
    ) -> Vec<EntityId> {
        let mut dead_ids = Vec::new();
        let head_a = snake_a.body.front().unwrap();
        let head_b = snake_b.body.front().unwrap();

        // Правило 1: Столкновение лбами
        if head_a == head_b {
            // Удаляется случайная змейка
            if rand::thread_rng().gen_bool(0.5) {
                dead_ids.push(id_a);
            } else {
                dead_ids.push(id_b);
            }
            return dead_ids;
        }

        // Правило 2: Голова А врезалась в тело Б
        if snake_b.body.iter().any(|part| part == head_a) {
            dead_ids.push(id_a);
        }

        // Правило 2: Голова Б врезалась в тело А
        if snake_a.body.iter().any(|part| part == head_b) {
            dead_ids.push(id_b);
        }

        dead_ids
    }
}