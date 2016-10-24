// общие системы.

use tinyecs::*;

use ::ground::components::*;


pub struct SpawnSystem;

impl System for SpawnSystem {
    // Добавляем систтему "SpawnSystem" к сущностям содержащих компоненты "SpawnPoint", "Position"
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnPoint, Position)
    }

    // обработчик, вызывается при update
    fn process_w(&mut self, entity: &mut Entity, world: &mut WorldHandle) {
        {
            // берем компонент "Точка спавна/spawn_point"
            let mut spawn_point = entity.get_component::<SpawnPoint>();

            // проверяем наличие заданных спавнов.
            while spawn_point.count > 0 {
                //if spawn_point.count > 0 {
                println!("создаем сущность 'Спавнер'");
                let spawned = world.entity_manager.create_entity();
                // добавляем сущности компонент "Имя", имя берем из компонента "spawn_point"
                spawned.add_component(Name { name: spawn_point.name.to_string() });
                spawned.refresh();

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