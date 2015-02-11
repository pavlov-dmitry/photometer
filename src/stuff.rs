/// Сборка ряда "сервисов" в отвязке от запроса, для использовани их вне контекста запроса
/// например для реализации каких-либо действий по таймеру

use iron::typemap::{ TypeMap, Key };
use iron::{ BeforeMiddleware, Request, IronResult };
use std::sync::Arc;

pub struct Stuff {
    pub extensions: TypeMap
}

impl Stuff {
    fn new() -> Stuff {
        Stuff {
            extensions : TypeMap::new()
        }
    }
}

type StuffInstallablePtr = Box<StuffInstallable + Send + Sync>;

#[derive(Clone)]
pub struct StuffMiddleware {
    collection: Arc<StuffCollection>
}

pub struct StuffCollection {
    elems : Vec<StuffInstallablePtr>
}

impl StuffCollection {
    pub fn new() -> StuffCollection {
        StuffCollection {
            elems: Vec::new()
        }
    }
    pub fn add<T: StuffInstallable + Send + Sync>(&mut self, part: T ) {
        self.elems.push( Box::new( part ) as StuffInstallablePtr );
    }
}

impl StuffMiddleware {
    pub fn new( collection: StuffCollection ) -> StuffMiddleware {
        StuffMiddleware {
            collection : Arc::new( collection )
        }
    }

    pub fn new_stuff(&self) -> Stuff {
        let mut stuff = Stuff::new();
        for part in self.collection.elems.iter() {
            part.install_to( &mut stuff );
        }
        stuff
    }
}

pub trait Stuffable {
    fn stuff(&mut self) -> &mut Stuff;
}

pub trait StuffInstallable {
    fn install_to( &self, stuff: &mut Stuff );
}

impl Key for Stuff { type Value = Stuff; }

impl BeforeMiddleware for StuffMiddleware {
    fn before( &self, req: &mut Request ) -> IronResult<()> {
        req.extensions.insert::<Stuff>( self.new_stuff() );
        Ok( () )
    }
}

impl<'a> Stuffable for Request<'a> {
    fn stuff(&mut self) -> &mut Stuff {
        self.extensions.get_mut::<Stuff>().unwrap()
    }
}
