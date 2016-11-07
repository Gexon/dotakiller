// общие системы.

use tinyecs::*;
use time::PreciseTime;

use ::ground::components::*;
use ::flora::components::*;

pub struct SpawnSystem;
// Система создает объекты в мире.
impl System for SpawnSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint", "Position"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnPoint, Position)
    }

    // обработчик, вызывается при update
    fn process_w(&mut self, entity: &mut Entity, world: &mut WorldHandle) {
        {
            // берем компонент "Точка спавна/spawn_point"
            let spawn_point = entity.get_component::<SpawnPoint>();
            // вынимаем позицию
            let position = entity.get_component::<Position>();

            // проверяем наличие заданных объектов.
            // создаем объект Пальма.
            println!("Создаем сущность {}", spawn_point.name.to_string());
            let entity_object = world.entity_manager.create_entity();
            entity_object.add_component(Name { name: spawn_point.name.to_string() });
            entity_object.add_component(Position { x: position.x, y: position.y });
            entity_object.add_component(FloraClass);
            entity_object.add_component(Graphic { need_replication: true });
            entity_object.add_component(FloraState { state: 1, start: PreciseTime::now() });
            entity_object.refresh();
        }
        entity.remove_component::<SpawnPoint>(); // удаляем компонент "Точка спавна/spawn_point"
        entity.refresh();
    }
}


