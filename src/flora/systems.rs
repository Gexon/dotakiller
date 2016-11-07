// тут будут описаны все системы специфичные только для растений.
use tinyecs::*;
use time::{PreciseTime, Duration};

use ::ground::components::*;
use ::flora::components::*;


pub struct _PlantReproduction;

impl System for _PlantReproduction {
    // выбираем сущности содержащие компоненты "FloraClass"
    fn aspect(&self) -> Aspect {
        aspect_all![FloraClass]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        // перебираем все сущности
        for entity in entities {
            //let mut state = entity.get_component::<FloraState>();

            // пробуем бросить семя через каждые 20 сек
            //if float_delta > 20f32 {
            let _position = entity.get_component::<Position>();
            //let target_x = стрим окончен всем пока!


            {
                // поручаем спавнеру, засумонить в наш мир пальму.
                // создаем спавнер
                //                    let mut entity_manager = dk_world.entity_manager();
                //                    let entity = entity_manager.create_entity();
                //
                //                    entity.add_component(SpawnPoint { name: "palm" });
                //                    entity.add_component(Position { x: 0f32, y: 0f32 });
                //                    entity.refresh();
                //                    println!("Пальма размножилась");
            }
            //}
            // фиксируем время
            //state.start = PreciseTime::now();
        }
    }
}

pub struct PlantGrowth;

impl System for PlantGrowth {
    // выбираем сущности содержащие компоненты "FloraState"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass)
    }

    // эта функция выполняется во время update столько раз, сколько сущностей содержащих компонент FloraState
    fn process_one(&mut self, entity: &mut Entity) {
        trace!("PLANT GROWTH");
        let mut state = entity.get_component::<FloraState>();

        // инкрементим state, тобиш его рост.
        if state.start.to(PreciseTime::now()) > Duration::seconds(10) && state.state < 10 {
            let mut graphic = entity.get_component::<Graphic>();
            state.state += 1;
            graphic.need_replication = true;
            println!("Пальма выросла немного");
            // фиксируем текущее время
            state.start = PreciseTime::now();
        }
    }
}

