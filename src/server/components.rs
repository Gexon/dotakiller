// тут будут расписаны все компоненты специфичные только для серверов.

use tinyecs::*;




// компонент сервера репликации
pub struct ReplicationServerClass;

impl Component for ReplicationServerClass {}