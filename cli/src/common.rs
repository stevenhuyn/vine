use clap::Args;

use ivm::{heap::Heap, IVM};
use ivy::{ast::Nets, optimize::Optimizer};

#[derive(Debug, Default, Args)]
pub struct Optimizations {
  #[arg(long)]
  no_opt: bool,
}

impl Optimizations {
  pub fn apply(self, nets: &mut Nets) {
    if !self.no_opt {
      Optimizer::default().optimize(nets)
    }
  }
}

#[derive(Debug, Args)]
pub struct RunArgs {
  #[arg(long)]
  no_stats: bool,
}

impl RunArgs {
  pub fn run(self, nets: Nets) {
    let mut globals = Vec::new();
    let globals = nets.serialize(&mut globals);
    let main = &globals[nets.get_index_of("::main").expect("missing main")];
    let heap = Heap::new();
    let mut ivm = IVM::new(&heap);
    ivm.boot(main);
    ivm.normalize();
    if !self.no_stats {
      eprintln!("{}", ivm.stats);
    }
  }
}