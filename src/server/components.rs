// тут будут расписаны все компоненты специфичные только для серверов.

use tinyecs::*;




// компонент сервера репликации
pub struct ReplicationServerClass;

impl Component for ReplicationServerClass {}

// компонент монстра-сервера
pub struct MonsterServerClass;

impl Component for MonsterServerClass {}