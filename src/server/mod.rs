// тут будем контакт контачить

mod server;


// новый сервак
struct ServerSystem {
    server_data: Server
}

impl ServerSystem {
    fn new() -> ServerSystem {
        let server = Server {
            sock: sock,

            // Даем нашему серверу токен с номером большим чем может поместиться в нашей плите 'Slab'.
            // Политы 'Slab' используються только для внутреннего смещения.
            token: Token(10_000_000),

            // SERVER is Token(1), после запуска такого сервака
            // мы сможет подключить не больше 128 клиентов.
            conns: Slab::with_capacity(128),

            // Список событий что сервер должен обработать.
            events: Events::with_capacity(1024)
        };
        server.register(poll).expect("I WANT HANDLE ERROR");
        ServerSystem {
            server: {
                server_data: server
            }
        }
    }
}

impl System for ServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Syncable, Position, Wtf]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let cnt = try!(poll.poll(&mut self.server_data.events, None));

        let mut i = 0;

        trace!("обработка событий... cnt={}; len={}", cnt, self.server_data.events.len());
        //println!("обработка событий... cnt={}; len={}", cnt, self.server_data.events.len());

        // Перебираем уведомления.
        // Каждое из этих событий дает token для регистрации
        // (который обычно представляет собой, handle события),
        // а также информацию о том, какие события происходили (чтение, запись, сигнал, и т. д.)
        while i < cnt {
            //println!("run while");
            // TODO this would be nice if it would turn a Result type. trying to convert this
            // into a io::Result runs into a problem because .ok_or() expects std::Result and
            // not io::Result
            let event = self.server_data.events.get(i).expect("Ошибка получения события");

            trace!("event={:?}; idx={:?}", event, i);
            //println!("event={:?}; idx={:?}", event, i);
            self.server_data.ready(poll, event.token(), event.kind());

            i += 1;
        }

        self.tick(poll);
    }
}