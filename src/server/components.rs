// тут будут расписаны все компоненты специфичные только для серверов.

use tinyecs::*;




// компонент сервера репликации
pub struct _ReplicationServerClass;

impl Component for _ReplicationServerClass {}

// компонент монстра-сервера
pub struct MonsterServerClass;

impl Component for MonsterServerClass {}