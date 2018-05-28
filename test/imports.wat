(module
  (type $type0 (func (param i32) (result i32)))
  (type $type1 (func (param i32 i32) (result i32)))
  (type $type2 (func))
  (import $import0 "env" "printf"  (param i32 i32) (result i32))
  (table 0 anyfunc)
  (memory 1)
  (export "memory" memory)
  (export "_Z2hiv" $func1)
  (func $func1
    i32.const 16
    i32.const 0
    call $import0
    drop
  )
  (data (i32.const 16)
    "Hello world\00"
  )
)