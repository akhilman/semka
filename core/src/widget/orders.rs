use crate::path::Path;
use futures::future::{BoxFuture, Future, FutureExt};
use std::any::Any;
use std::collections::{BTreeSet, VecDeque};

pub struct WidgetOrders {
    pub(crate) orders: VecDeque<WidgetCmd>,
}

impl WidgetOrders {
    pub fn new() -> Self {
        WidgetOrders {
            orders: VecDeque::new(),
        }
    }
    pub fn fetch_bytes(mut self, path: Path) -> Self {
        self.orders.push_back(WidgetCmd::FetchBytes(path));
        self
    }
    pub fn fetch_json(mut self, path: Path) -> Self {
        self.orders.push_back(WidgetCmd::FetchJson(path));
        self
    }
    pub fn fetch_text(mut self, path: Path) -> Self {
        self.orders.push_back(WidgetCmd::FetchText(path));
        self
    }
    pub fn perform_cmd<O: Any + Send>(
        mut self,
        cmd: impl Future<Output = O> + 'static + Send,
    ) -> Self {
        self.orders.push_front(WidgetCmd::PerformCmd(Box::pin(
            cmd.map(|out: O| Box::new(out) as Box<dyn Any>),
        )));
        self
    }
    pub fn update_deps(mut self, deps: BTreeSet<Path>) -> Self {
        self.orders.push_front(WidgetCmd::UpdateDependencies(deps));
        self
    }
    pub fn skip(mut self) -> Self {
        self.orders.push_front(WidgetCmd::Skip);
        self
    }
}

pub enum WidgetCmd {
    FetchBytes(Path),
    FetchJson(Path),
    FetchText(Path),
    PerformCmd(BoxFuture<'static, Box<dyn Any>>),
    UpdateDependencies(BTreeSet<Path>),
    Skip,
}
