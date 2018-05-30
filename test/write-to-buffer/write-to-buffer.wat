(module
 (type $FUNCSIG$iiii (func (param i32 i32 i32) (result i32)))
 (type $FUNCSIG$i (func (result i32)))
 (type $FUNCSIG$ii (func (param i32) (result i32)))
 (type $FUNCSIG$vi (func (param i32)))
 (type $FUNCSIG$viiii (func (param i32 i32 i32 i32)))
 (type $FUNCSIG$viii (func (param i32 i32 i32)))
 (type $FUNCSIG$v (func))
 (import "env" "_ZNKSt3__120__vector_base_commonILb1EE20__throw_length_errorEv" (func $_ZNKSt3__120__vector_base_commonILb1EE20__throw_length_errorEv (param i32)))
 (import "env" "_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6__initEPKcjj" (func $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6__initEPKcjj (param i32 i32 i32 i32)))
 (import "env" "_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6appendEPKcj" (func $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6appendEPKcj (param i32 i32 i32) (result i32)))
 (import "env" "_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev" (func $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev (param i32) (result i32)))
 (import "env" "_ZNSt9bad_allocC1Ev" (func $_ZNSt9bad_allocC1Ev (param i32) (result i32)))
 (import "env" "_ZdlPv" (func $_ZdlPv (param i32)))
 (import "env" "_Znwj" (func $_Znwj (param i32) (result i32)))
 (import "env" "__cxa_allocate_exception" (func $__cxa_allocate_exception (param i32) (result i32)))
 (import "env" "__cxa_atexit" (func $__cxa_atexit (param i32 i32 i32) (result i32)))
 (import "env" "__cxa_throw" (func $__cxa_throw (param i32 i32 i32)))
 (import "env" "strlen" (func $strlen (param i32) (result i32)))
 (import "env" "__dso_handle" (global $__dso_handle i32))
 (import "env" "_ZTISt9bad_alloc" (global $_ZTISt9bad_alloc i32))
 (import "env" "_ZNSt9bad_allocD1Ev" (func $_ZNSt9bad_allocD1Ev (param i32) (result i32)))
 (table 3 3 anyfunc)
 (elem (i32.const 0) $__wasm_nullptr $__importThunk__ZNSt9bad_allocD1Ev $__cxx_global_array_dtor)
 (memory $0 1)
 (data (i32.const 32) "entering function \00")
 (data (i32.const 64) "exiting function \00")
 (export "memory" (memory $0))
 (export "_Z12entered_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE" (func $_Z12entered_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE))
 (export "_Z11exited_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE" (func $_Z11exited_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE))
 (export "_GLOBAL__sub_I_2ddb24e332320669b9c107d93a98a092.cpp" (func $_GLOBAL__sub_I_2ddb24e332320669b9c107d93a98a092.cpp))
 (func $__cxx_global_array_dtor (; 12 ;) (type $FUNCSIG$vi) (param $0 i32)
  (drop
   (call $_ZNSt3__113__vector_baseINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEED2Ev
    (i32.const 12)
   )
  )
 )
 (func $_ZNSt3__113__vector_baseINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEED2Ev (; 13 ;) (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (block $label$0
   (br_if $label$0
    (i32.eqz
     (tee_local $3
      (i32.load
       (get_local $0)
      )
     )
    )
   )
   (block $label$1
    (br_if $label$1
     (i32.eq
      (tee_local $2
       (i32.load offset=4
        (get_local $0)
       )
      )
      (get_local $3)
     )
    )
    (set_local $1
     (i32.add
      (get_local $0)
      (i32.const 4)
     )
    )
    (loop $label$2
     (i32.store
      (get_local $1)
      (tee_local $2
       (i32.add
        (get_local $2)
        (i32.const -12)
       )
      )
     )
     (drop
      (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev
       (get_local $2)
      )
     )
     (br_if $label$2
      (i32.ne
       (tee_local $2
        (i32.load
         (get_local $1)
        )
       )
       (get_local $3)
      )
     )
    )
    (set_local $3
     (i32.load
      (get_local $0)
     )
    )
   )
   (call $_ZdlPv
    (get_local $3)
   )
  )
  (get_local $0)
 )
 (func $_Z12entered_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE (; 14 ;) (param $0 i32)
  (local $1 i32)
  (i32.store offset=4
   (i32.const 0)
   (tee_local $1
    (i32.sub
     (i32.load offset=4
      (i32.const 0)
     )
     (i32.const 16)
    )
   )
  )
  (call $_ZNSt3__1plIcNS_11char_traitsIcEENS_9allocatorIcEEEENS_12basic_stringIT_T0_T1_EEPKS6_RKS9_
   (get_local $1)
   (i32.const 32)
   (get_local $0)
  )
  (block $label$0
   (block $label$1
    (br_if $label$1
     (i32.ge_u
      (tee_local $0
       (i32.load offset=16
        (i32.const 0)
       )
      )
      (i32.load offset=20
       (i32.const 0)
      )
     )
    )
    (i64.store align=4
     (get_local $0)
     (i64.load
      (get_local $1)
     )
    )
    (i32.store
     (i32.add
      (get_local $0)
      (i32.const 8)
     )
     (i32.load
      (tee_local $0
       (i32.add
        (get_local $1)
        (i32.const 8)
       )
      )
     )
    )
    (i32.store
     (get_local $0)
     (i32.const 0)
    )
    (i32.store offset=4
     (get_local $1)
     (i32.const 0)
    )
    (i32.store
     (get_local $1)
     (i32.const 0)
    )
    (i32.store offset=16
     (i32.const 0)
     (i32.add
      (i32.load offset=16
       (i32.const 0)
      )
      (i32.const 12)
     )
    )
    (br $label$0)
   )
   (call $_ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE21__push_back_slow_pathIS6_EEvOT_
    (i32.const 12)
    (get_local $1)
   )
  )
  (drop
   (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev
    (get_local $1)
   )
  )
  (i32.store offset=4
   (i32.const 0)
   (i32.add
    (get_local $1)
    (i32.const 16)
   )
  )
 )
 (func $_ZNSt3__1plIcNS_11char_traitsIcEENS_9allocatorIcEEEENS_12basic_stringIT_T0_T1_EEPKS6_RKS9_ (; 15 ;) (param $0 i32) (param $1 i32) (param $2 i32)
  (local $3 i32)
  (local $4 i32)
  (i64.store align=4
   (get_local $0)
   (i64.const 0)
  )
  (i32.store
   (i32.add
    (get_local $0)
    (i32.const 8)
   )
   (i32.const 0)
  )
  (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6__initEPKcjj
   (get_local $0)
   (get_local $1)
   (tee_local $3
    (call $strlen
     (get_local $1)
    )
   )
   (i32.add
    (get_local $3)
    (tee_local $4
     (select
      (i32.load offset=4
       (get_local $2)
      )
      (i32.shr_u
       (tee_local $4
        (i32.load8_u
         (get_local $2)
        )
       )
       (i32.const 1)
      )
      (i32.and
       (get_local $4)
       (i32.const 1)
      )
     )
    )
   )
  )
  (drop
   (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE6appendEPKcj
    (get_local $0)
    (select
     (i32.load offset=8
      (get_local $2)
     )
     (i32.add
      (get_local $2)
      (i32.const 1)
     )
     (i32.and
      (i32.load8_u
       (get_local $2)
      )
      (i32.const 1)
     )
    )
    (get_local $4)
   )
  )
 )
 (func $_Z11exited_funcNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEE (; 16 ;) (param $0 i32)
  (local $1 i32)
  (i32.store offset=4
   (i32.const 0)
   (tee_local $1
    (i32.sub
     (i32.load offset=4
      (i32.const 0)
     )
     (i32.const 16)
    )
   )
  )
  (call $_ZNSt3__1plIcNS_11char_traitsIcEENS_9allocatorIcEEEENS_12basic_stringIT_T0_T1_EEPKS6_RKS9_
   (get_local $1)
   (i32.const 64)
   (get_local $0)
  )
  (block $label$0
   (block $label$1
    (br_if $label$1
     (i32.ge_u
      (tee_local $0
       (i32.load offset=16
        (i32.const 0)
       )
      )
      (i32.load offset=20
       (i32.const 0)
      )
     )
    )
    (i64.store align=4
     (get_local $0)
     (i64.load
      (get_local $1)
     )
    )
    (i32.store
     (i32.add
      (get_local $0)
      (i32.const 8)
     )
     (i32.load
      (tee_local $0
       (i32.add
        (get_local $1)
        (i32.const 8)
       )
      )
     )
    )
    (i32.store
     (get_local $0)
     (i32.const 0)
    )
    (i32.store offset=4
     (get_local $1)
     (i32.const 0)
    )
    (i32.store
     (get_local $1)
     (i32.const 0)
    )
    (i32.store offset=16
     (i32.const 0)
     (i32.add
      (i32.load offset=16
       (i32.const 0)
      )
      (i32.const 12)
     )
    )
    (br $label$0)
   )
   (call $_ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE21__push_back_slow_pathIS6_EEvOT_
    (i32.const 12)
    (get_local $1)
   )
  )
  (drop
   (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev
    (get_local $1)
   )
  )
  (i32.store offset=4
   (i32.const 0)
   (i32.add
    (get_local $1)
    (i32.const 16)
   )
  )
 )
 (func $_ZNSt3__16vectorINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEENS4_IS6_EEE21__push_back_slow_pathIS6_EEvOT_ (; 17 ;) (param $0 i32) (param $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (i32.store offset=4
   (i32.const 0)
   (tee_local $8
    (i32.sub
     (i32.load offset=4
      (i32.const 0)
     )
     (i32.const 32)
    )
   )
  )
  (block $label$0
   (br_if $label$0
    (i32.lt_u
     (tee_local $5
      (i32.add
       (i32.div_s
        (i32.sub
         (i32.load offset=4
          (get_local $0)
         )
         (tee_local $3
          (i32.load
           (get_local $0)
          )
         )
        )
        (i32.const 12)
       )
       (i32.const 1)
      )
     )
     (i32.const 357913942)
    )
   )
   (call $_ZNKSt3__120__vector_base_commonILb1EE20__throw_length_errorEv
    (get_local $0)
   )
   (set_local $3
    (i32.load
     (get_local $0)
    )
   )
  )
  (set_local $4
   (i32.add
    (get_local $0)
    (i32.const 8)
   )
  )
  (set_local $6
   (i32.const 357913941)
  )
  (block $label$1
   (br_if $label$1
    (i32.gt_u
     (tee_local $7
      (i32.div_s
       (i32.sub
        (i32.load offset=8
         (get_local $0)
        )
        (get_local $3)
       )
       (i32.const 12)
      )
     )
     (i32.const 178956969)
    )
   )
   (set_local $6
    (select
     (get_local $5)
     (tee_local $6
      (i32.shl
       (get_local $7)
       (i32.const 1)
      )
     )
     (i32.lt_u
      (get_local $6)
      (get_local $5)
     )
    )
   )
  )
  (i64.store align=4
   (tee_local $3
    (i32.load offset=8
     (tee_local $2
      (call $_ZNSt3__114__split_bufferINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS4_IS6_EEEC2EjjS8_
       (i32.add
        (get_local $8)
        (i32.const 8)
       )
       (get_local $6)
       (i32.div_s
        (i32.sub
         (i32.load
          (tee_local $5
           (i32.add
            (get_local $0)
            (i32.const 4)
           )
          )
         )
         (get_local $3)
        )
        (i32.const 12)
       )
       (get_local $4)
      )
     )
    )
   )
   (i64.load align=4
    (get_local $1)
   )
  )
  (i32.store
   (i32.add
    (get_local $3)
    (i32.const 8)
   )
   (i32.load
    (tee_local $3
     (i32.add
      (get_local $1)
      (i32.const 8)
     )
    )
   )
  )
  (i32.store
   (get_local $3)
   (i32.const 0)
  )
  (i32.store
   (get_local $1)
   (i32.const 0)
  )
  (i32.store
   (i32.add
    (get_local $1)
    (i32.const 4)
   )
   (i32.const 0)
  )
  (i32.store offset=8
   (get_local $2)
   (tee_local $3
    (i32.add
     (i32.load offset=8
      (get_local $2)
     )
     (i32.const 12)
    )
   )
  )
  (block $label$2
   (block $label$3
    (br_if $label$3
     (i32.eq
      (tee_local $1
       (i32.load
        (get_local $5)
       )
      )
      (tee_local $7
       (i32.load
        (get_local $0)
       )
      )
     )
    )
    (set_local $6
     (i32.load
      (tee_local $4
       (i32.add
        (get_local $2)
        (i32.const 4)
       )
      )
     )
    )
    (loop $label$4
     (i32.store
      (i32.add
       (get_local $6)
       (i32.const -4)
      )
      (i32.load
       (tee_local $5
        (i32.add
         (get_local $1)
         (i32.const -4)
        )
       )
      )
     )
     (i64.store align=4
      (i32.add
       (get_local $6)
       (i32.const -12)
      )
      (i64.load align=4
       (tee_local $3
        (i32.add
         (get_local $1)
         (i32.const -12)
        )
       )
      )
     )
     (i32.store
      (i32.add
       (get_local $1)
       (i32.const -8)
      )
      (i32.const 0)
     )
     (i32.store
      (get_local $5)
      (i32.const 0)
     )
     (i32.store
      (get_local $3)
      (i32.const 0)
     )
     (i32.store
      (get_local $4)
      (tee_local $6
       (i32.add
        (i32.load
         (get_local $4)
        )
        (i32.const -12)
       )
      )
     )
     (set_local $1
      (get_local $3)
     )
     (br_if $label$4
      (i32.ne
       (get_local $7)
       (get_local $3)
      )
     )
    )
    (set_local $3
     (i32.load
      (i32.add
       (get_local $2)
       (i32.const 8)
      )
     )
    )
    (set_local $1
     (i32.load
      (i32.add
       (get_local $0)
       (i32.const 4)
      )
     )
    )
    (set_local $7
     (i32.load
      (get_local $0)
     )
    )
    (br $label$2)
   )
   (set_local $6
    (i32.load
     (i32.add
      (get_local $2)
      (i32.const 4)
     )
    )
   )
  )
  (i32.store
   (get_local $0)
   (get_local $6)
  )
  (i32.store
   (i32.add
    (get_local $2)
    (i32.const 4)
   )
   (get_local $7)
  )
  (i32.store
   (i32.add
    (get_local $0)
    (i32.const 4)
   )
   (get_local $3)
  )
  (i32.store
   (i32.add
    (get_local $2)
    (i32.const 8)
   )
   (get_local $1)
  )
  (set_local $3
   (i32.load
    (tee_local $1
     (i32.add
      (get_local $0)
      (i32.const 8)
     )
    )
   )
  )
  (i32.store
   (get_local $1)
   (i32.load offset=12
    (get_local $2)
   )
  )
  (i32.store offset=12
   (get_local $2)
   (get_local $3)
  )
  (i32.store
   (get_local $2)
   (get_local $7)
  )
  (drop
   (call $_ZNSt3__114__split_bufferINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS4_IS6_EEED2Ev
    (get_local $2)
   )
  )
  (i32.store offset=4
   (i32.const 0)
   (i32.add
    (get_local $8)
    (i32.const 32)
   )
  )
 )
 (func $_ZNSt3__114__split_bufferINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS4_IS6_EEEC2EjjS8_ (; 18 ;) (param $0 i32) (param $1 i32) (param $2 i32) (param $3 i32) (result i32)
  (local $4 i32)
  (set_local $4
   (i32.const 0)
  )
  (i32.store offset=12
   (get_local $0)
   (i32.const 0)
  )
  (i32.store
   (i32.add
    (get_local $0)
    (i32.const 16)
   )
   (get_local $3)
  )
  (block $label$0
   (block $label$1
    (br_if $label$1
     (i32.eqz
      (get_local $1)
     )
    )
    (br_if $label$0
     (i32.ge_u
      (get_local $1)
      (i32.const 357913942)
     )
    )
    (set_local $4
     (call $_Znwj
      (i32.mul
       (get_local $1)
       (i32.const 12)
      )
     )
    )
   )
   (i32.store
    (get_local $0)
    (get_local $4)
   )
   (i32.store offset=8
    (get_local $0)
    (tee_local $3
     (i32.add
      (get_local $4)
      (i32.mul
       (get_local $2)
       (i32.const 12)
      )
     )
    )
   )
   (i32.store offset=4
    (get_local $0)
    (get_local $3)
   )
   (i32.store offset=12
    (get_local $0)
    (i32.add
     (get_local $4)
     (i32.mul
      (get_local $1)
      (i32.const 12)
     )
    )
   )
   (return
    (get_local $0)
   )
  )
  (drop
   (call $_ZNSt9bad_allocC1Ev
    (tee_local $0
     (call $__cxa_allocate_exception
      (i32.const 4)
     )
    )
   )
  )
  (call $__cxa_throw
   (get_local $0)
   (get_global $_ZTISt9bad_alloc)
   (i32.const 1)
  )
  (unreachable)
 )
 (func $_ZNSt3__114__split_bufferINS_12basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEERNS4_IS6_EEED2Ev (; 19 ;) (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (block $label$0
   (br_if $label$0
    (i32.eq
     (tee_local $3
      (i32.load offset=8
       (get_local $0)
      )
     )
     (tee_local $1
      (i32.load offset=4
       (get_local $0)
      )
     )
    )
   )
   (set_local $2
    (i32.add
     (get_local $0)
     (i32.const 8)
    )
   )
   (loop $label$1
    (i32.store
     (get_local $2)
     (tee_local $3
      (i32.add
       (get_local $3)
       (i32.const -12)
      )
     )
    )
    (drop
     (call $_ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEED1Ev
      (get_local $3)
     )
    )
    (br_if $label$1
     (i32.ne
      (tee_local $3
       (i32.load
        (get_local $2)
       )
      )
      (get_local $1)
     )
    )
   )
  )
  (block $label$2
   (br_if $label$2
    (i32.eqz
     (tee_local $3
      (i32.load
       (get_local $0)
      )
     )
    )
   )
   (call $_ZdlPv
    (get_local $3)
   )
  )
  (get_local $0)
 )
 (func $_GLOBAL__sub_I_2ddb24e332320669b9c107d93a98a092.cpp (; 20 ;)
  (i64.store offset=12 align=4
   (i32.const 0)
   (i64.const 0)
  )
  (i32.store offset=20
   (i32.const 0)
   (i32.const 0)
  )
  (drop
   (call $__cxa_atexit
    (i32.const 2)
    (i32.const 0)
    (get_global $__dso_handle)
   )
  )
 )
 (func $__wasm_nullptr (; 21 ;) (type $FUNCSIG$v)
  (unreachable)
 )
 (func $__importThunk__ZNSt9bad_allocD1Ev (; 22 ;) (type $FUNCSIG$ii) (param $0 i32) (result i32)
  (call $_ZNSt9bad_allocD1Ev
   (get_local $0)
  )
 )
)
