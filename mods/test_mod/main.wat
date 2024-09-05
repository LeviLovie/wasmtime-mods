(module
 (type $0 (func (param i32)))
 (type $1 (func))
 (import "host" "host_func" (func $assembly/index/host_func (param i32)))
 (memory $0 0)
 (export "hello" (func $assembly/index/hello))
 (export "memory" (memory $0))
 (func $assembly/index/hello
  i32.const 3
  call $assembly/index/host_func
 )
)
