// общие системы.

use tinyecs::*;

use ::ground::components::*;
use ::flora::components::FloraClass;


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
            let mut spawn_point = entity.get_component::<SpawnPoint>();
            // вынимаем позицию
            let position = entity.get_component::<Position>();

            // проверяем наличие заданных объектов.
            while spawn_point.count > 0 {
                // создаем объект Пальма.
                println!("Создаем сущность {}", spawn_point.name.to_string());
                let entity_object = world.entity_manager.create_entity();
                // добавляем сущности компонент "Имя", имя берем из компонента "spawn_point"
                entity_object.add_component(Name { name: spawn_point.name.to_string() });
                entity_object.add_component(Position{x:position.x, y: position.y});
                entity_object.add_component(FloraClass);
                entity_object.add_component(Graphic { need_replication: true });
                entity_object.refresh();

                spawn_point.count -= 1; //?
            }
        }
        entity.remove_component::<SpawnPoint>(); // удаляем компонент "Точка спавна/spawn_point"
        entity.refresh();
    }
}

pub struct _ReplicationSystem;

impl System for _ReplicationSystem {
    // Добавляем систтему "ReplicationSystem" к сущностям содержащих компоненты "NeedReplication"
    fn aspect(&self) -> Aspect {
        aspect_all!(Graphic)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        let graphic = entity.get_component::<Graphic>();
        if graphic.need_replication {
            // рассылаем всем клиентам "updherb idHerb classHerb stateHerb x y"
        };
    }
}