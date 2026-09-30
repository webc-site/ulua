#[macro_export]
macro_rules! PROP {
  ($emitter:expr, $node:expr, $prop:ident) => {
    $emitter.write(stringify!($prop), &$node.$prop);
  };
}

pub use PROP;
