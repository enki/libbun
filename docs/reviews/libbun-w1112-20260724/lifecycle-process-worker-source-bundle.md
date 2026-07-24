# Vendored Bun process-exit, VM interruption/reset, WebWorker quiescence, and ordered shutdown source bundle (correction 6)

Exact source SHA: 6066a5b85a0c6d1f6397914b8666b0fd0e5fd7eb

Exact source tree: cb964de8ab8162449fbe95959bf34d231570aa5c

The ordered source path is public or uncaught-exception-handler process exit -> Rust process owner -> main-VM global exit or worker exit -> concrete RuntimeHooks binding -> process-global worker registry termination sweep/wait -> per-worker termination checkpoint -> ordered VM unpublish, exit handlers, JSC teardown, unregister, exit dispatch, and worker-resource destruction. NodeVMModule::evaluate and NodeVMScript bind timeout/SIGINT exception clearing and VM termination-request reset. A reset is not quiescence: timeout, exception-path ambiguity, or surviving child/nested-worker state cannot authorize reuse.

Every compact excerpt names the complete owning item span selected from the exact file, plus the full-file blob/SHA-256/byte identity and an excerpt SHA-256. Small bounded files are included completely. The repository-wide discovery gate runs before this fixed closure is rendered.

## Bound source inventory

| Path | Git blob | Full-file SHA-256 | Bytes | Included source |
| --- | --- | --- | ---: | --- |
| vendor/bun/src/runtime/node/node_process.rs | cc29b9beb83fe57b806ae94ebd25cff44d407d3e | 41411bc2fd5cf382f8c0e5e117527e805d5d78a2943a4d8b1c00a4c7f4731ed3 | 25417 | complete owning items |
| vendor/bun/src/runtime/jsc_hooks.rs | 6ff3c4c76c1c100457ebcf23c64dff89f44b0617 | 1fcac2afe803982ba1073981b2457023bb128be0982f41ea34c1b4e39bbb645e | 250774 | complete owning items |
| vendor/bun/src/jsc/web_worker.rs | f39d9d806bea3c4eeb7a703ccbaaf9e8bdac886c | d5c322bfea520b0743a3c3e063f77024b7f3263200895b2bdd58f87663c5d57b | 80981 | complete owning items |
| vendor/bun/src/jsc/bindings/BunProcess.cpp | 6935f0cbbcee9bf5f5f5364560a16707b9cc527e | ec7a0a9d63d2bbd1549871c0e36b200336748a976c617ed593198bf5866a2872 | 193398 | complete owning items |
| vendor/bun/src/jsc/bindings/webcore/Worker.cpp | 9c06ac70eed091ad22cdee195990c36ed1c5b438 | 4ece2800cc6d33a5b49064cbbb2d0c0d7e1f8f3b29d7eb859c47e0fe6094a9bb | 32550 | complete owning items |
| vendor/bun/src/jsc/bindings/vm/SigintWatcher.cpp | 93f2c42bd2b53369ada78a93a33b04fce0f0fdc3 | 5d0adef2743902412b1d8f378176374dba9f03ba0e98248dff555552a2d56369 | 4424 | complete owning items |
| vendor/bun/src/jsc/bindings/NodeVM.cpp | dae4e540c9da7a8a974df664e9140162b514c7a4 | c90f9095225c82af0fc7c4c275b872d212854595ca52fd99d0c69d8dd153ba4f | 80305 | complete owning items |
| vendor/bun/src/jsc/bindings/NodeVMScript.cpp | d7cfa6157cbd91ba454848573318869ecd837f27 | 2bc8b85e761e0cafd3c2b9f18628dd78db6d34f9cff32d2934525017663e8c56 | 27536 | complete owning items |
| vendor/bun/src/jsc/bindings/NodeVMModule.cpp | bba5326eb0512ce4f6fb6e0d73b23dbb39592dee | b9e58e4bc00c8d01d98ca3991fdacfda2f59c8f99351aaca2432596528309ce2 | 24625 | complete owning items |

## vendor/bun/src/runtime/node/node_process.rs:1-64

- Full-file Git blob: cc29b9beb83fe57b806ae94ebd25cff44d407d3e
- Full-file SHA-256: 41411bc2fd5cf382f8c0e5e117527e805d5d78a2943a4d8b1c00a4c7f4731ed3
- Full-file bytes: 25417
- Excerpt line span: 1-64
- Excerpt SHA-256: 497ccda5b950081b41af4163f0a6beff2e98446416cf22cf35a70cbfe15e616e

     1  //! Process information and control APIs (`globalThis.process` / `node:process`)
     2
     3  use core::ffi::c_char;
     4
     5  use bun_core::env_var::feature_flag;
     6  use bun_core::{self, Environment, Global};
     7  use bun_jsc::zig_string::ZigString;
     8  use bun_jsc::{JSGlobalObject, JSValue, WebWorker, ZigStringJsc as _};
     9
    10  // TODO(port): move to <area>_sys — extern decls colocated for now
    11  unsafe extern "C" {
    12      safe fn Bun__Process__getArgv(global: &JSGlobalObject) -> JSValue;
    13      safe fn Bun__Process__getExecArgv(global: &JSGlobalObject) -> JSValue;
    14  }
    15
    16  // ───────────────────────────── argv0 / execPath ─────────────────────────────
    17
    18  // `&JSGlobalObject` is ABI-identical to `*const JSGlobalObject` (non-null) in
    19  // `extern "C"`; the C++ caller guarantees a live pointer, so the reference
    20  // param discharges the non-null obligation at the type level.
    21  #[unsafe(export_name = "Bun__Process__createArgv0")]
    22  pub extern "C" fn create_argv0(global_object: &JSGlobalObject) -> JSValue {
    23      let argv0 = bun_core::argv()
    24          .get(0)
    25          .map(|z| z.as_bytes())
    26          .unwrap_or(b"bun");
    27      ZigString::from_utf8(argv0).to_js(global_object)
    28  }
    29
    30  #[unsafe(export_name = "Bun__Process__getExecPath")]
    31  pub extern "C" fn get_exec_path(global_object: &JSGlobalObject) -> JSValue {
    32      let Ok(out) = bun_core::self_exe_path() else {
    33          // if for any reason we are unable to get the executable path, we just return argv[0]
    34          return create_argv0(global_object);
    35      };
    36      ZigString::from_utf8(out.as_bytes()).to_js(global_object)
    37  }
    38
    39  // ───────────────────────────── argv (C++ accessor wrappers) ─────────────────
    40
    41  pub extern "C" fn get_argv(global: &JSGlobalObject) -> JSValue {
    42      Bun__Process__getArgv(global)
    43  }
    44
    45  pub extern "C" fn get_exec_argv(global: &JSGlobalObject) -> JSValue {
    46      Bun__Process__getExecArgv(global)
    47  }
    48
    49  // ───────────────────────────── exit ─────────────────────────────
    50
    51  // TODO(@190n) this may need to be noreturn
    52  #[unsafe(export_name = "Bun__Process__exit")]
    53  pub extern "C" fn exit(global_object: &JSGlobalObject, code: u8) {
    54      let vm = global_object.bun_vm().as_mut();
    55      vm.exit_handler.exit_code = code;
    56      if let Some(worker) = vm.worker_ref() {
    57          // TODO(@190n) we may need to use requestTerminate or throwTerminationException
    58          // instead to terminate the worker sooner
    59          worker.exit();
    60      } else {
    61          vm.on_exit();
    62          vm.global_exit();
    63      }
    64  }

## vendor/bun/src/runtime/jsc_hooks.rs:1188-1199

- Full-file Git blob: 6ff3c4c76c1c100457ebcf23c64dff89f44b0617
- Full-file SHA-256: 1fcac2afe803982ba1073981b2457023bb128be0982f41ea34c1b4e39bbb645e
- Full-file bytes: 250774
- Excerpt line span: 1188-1199
- Excerpt SHA-256: 849d7eb903d51850dcd809786e6d7865eecace4e000697d2a915b9995d13dfc1

  1188  /// `bun.api.node.process.exit(global, code)` — Spec
  1189  /// `runtime/node/node_process.zig`. Main-thread is `noreturn`; in a worker it
  1190  /// returns and the caller `panic!`s.
  1191  ///
  1192  /// # Safety
  1193  /// `global` is the live VM global.
  1194  unsafe fn process_exit(global: *mut JSGlobalObject, code: u8) {
  1195      // SAFETY: per fn contract — `global` is the live VM global. The deref is
  1196      // performed once here in the hook shim so the user-facing `process::exit`
  1197      // can take a safe `&JSGlobalObject`.
  1198      crate::node::process::exit(unsafe { &*global }, code);
  1199  }

## vendor/bun/src/runtime/jsc_hooks.rs:1397-1428

- Full-file Git blob: 6ff3c4c76c1c100457ebcf23c64dff89f44b0617
- Full-file SHA-256: 1fcac2afe803982ba1073981b2457023bb128be0982f41ea34c1b4e39bbb645e
- Full-file bytes: 250774
- Excerpt line span: 1397-1428
- Excerpt SHA-256: b8d81e95e93312c4985569b38a4b43013b38f59adb461f6861bf6df67056f87d

  1397  /// The static `RuntimeHooks` instance handed to `bun_jsc`.
  1398  #[unsafe(no_mangle)]
  1399  pub static __BUN_RUNTIME_HOOKS: RuntimeHooks = RuntimeHooks {
  1400      init_runtime_state,
  1401      deinit_runtime_state,
  1402      generate_entry_point,
  1403      load_preloads,
  1404      ensure_debugger,
  1405      auto_tick,
  1406      auto_tick_active,
  1407      print_exception,
  1408      timer_insert,
  1409      timer_remove,
  1410      default_client_ssl_ctx,
  1411      ssl_ctx_cache_get_or_create,
  1412      create_node_fs,
  1413      has_blob_url,
  1414      body_mixin_get_blob,
  1415      process_exit,
  1416      handle_ipc_internal_child,
  1417      ipc_child_singleton_deinit,
  1418      console_on_before_print,
  1419      console_print_runtime_object,
  1420      load_standalone_sourcemap,
  1421      bake_per_thread_source_map,
  1422      apply_standalone_runtime_flags,
  1423      parse_worker_exec_argv_allow_addons,
  1424      cron_clear_all_teardown,
  1425      cron_clear_all_reload,
  1426      terminate_all_workers_and_wait,
  1427      retroactively_report_discovered_tests,
  1428  };

## vendor/bun/src/runtime/jsc_hooks.rs:1518-1527

- Full-file Git blob: 6ff3c4c76c1c100457ebcf23c64dff89f44b0617
- Full-file SHA-256: 1fcac2afe803982ba1073981b2457023bb128be0982f41ea34c1b4e39bbb645e
- Full-file bytes: 250774
- Excerpt line span: 1518-1527
- Excerpt SHA-256: a8812f9bc58f794a2f794b5c1b3d647cb7005ef2fb1f4d11fe7e0fa30559f100

  1518  /// `webcore.WebWorker.terminateAllAndWait(timeout_ms)` — spec
  1519  /// VirtualMachine.zig:975. Forwards to the in-crate `bun_jsc::web_worker`
  1520  /// implementation; routed through `RuntimeHooks` because `virtual_machine.rs`
  1521  /// sits below `web_worker.rs` in the module DAG and the wait re-enters
  1522  /// `auto_tick` (this crate) on the worker side.
  1523  ///
  1524  /// Main-thread only; called from `global_exit` after `is_shutting_down` is set.
  1525  fn terminate_all_workers_and_wait(timeout_ms: u64) {
  1526      bun_jsc::web_worker::terminate_all_and_wait(timeout_ms);
  1527  }

## vendor/bun/src/jsc/web_worker.rs:90-203

- Full-file Git blob: f39d9d806bea3c4eeb7a703ccbaaf9e8bdac886c
- Full-file SHA-256: d5c322bfea520b0743a3c3e063f77024b7f3263200895b2bdd58f87663c5d57b
- Full-file bytes: 80981
- Excerpt line span: 90-203
- Excerpt SHA-256: 200a81d96cd7785b140f0fb466c64977b12cf62b9ae822689757f7d1865da1c4

    90      /// "Known gap" in the file header. When `parent_poll_ref` is held (the
    91      /// default), the parent's loop stays alive until the close task runs.
    92      // `BackRef` (not `&'a VirtualMachine`) because the struct is FFI-owned and
    93      // crosses threads; the backref invariant (parent outlives child via
    94      // `parent_poll_ref`) is documented above.
    95      parent: bun_ptr::BackRef<VirtualMachine>,
    96      parent_context_id: u32,
    97      execution_context_id: u32,
    98      mini: bool,
    99      eval_mode: bool,
   100      store_fd: bool,
   101      /// Borrowed from C++ `WorkerOptions` (kept alive by the owning `Worker`).
   102      // TODO(port): lifetime — borrowed from cpp_worker (BACKREF).
   103      argv_ptr: *const WTFStringImpl,
   104      argv_len: usize,
   105      exec_argv_ptr: *const WTFStringImpl,
   106      exec_argv_len: usize,
   107      inherit_exec_argv: bool,
   108      /// Heap-owned by this struct; freed in `destroy()`.
   109      unresolved_specifier: Box<[u8]>,
   110      preloads: Vec<Box<[u8]>>,
   111      /// Owned NUL-terminated bytes; Zig was `[:0]const u8`.
   112      name: bun_core::ZBox,
   113
   114      // ---- Cross-thread signalling --------------------------------------------
   115      /// Intrusive node for the process-global `LiveWorkers` list. Registered
   116      /// before the thread is spawned; removed in `shutdown()` once the worker is
   117      /// past all process-global resolver access.
   118      ///
   119      /// `Cell` because `terminate_all_and_wait` walks the list through
   120      /// `&WebWorker` while `register`/`unregister` (under `live_workers::MUTEX`)
   121      /// write these on another thread — the mutex serialises memory ops, but
   122      /// Rust's aliasing model still requires interior mutability. `*mut T` is
   123      /// `Copy`, so `Cell` (not `UnsafeCell`) suffices and every read/write is
   124      /// safe `.get()`/`.set()`.
   125      // TODO(port): intrusive doubly-linked list node — `bun_collections` has no
   126      // `IntrusiveList` yet; raw next/prev pointers used directly.
   127      live_next: Cell<*mut WebWorker>,
   128      live_prev: Cell<*mut WebWorker>,
   129
   130      /// Set by the parent (`notifyNeedTermination`) or by the worker itself
   131      /// (`exit`). The worker loop polls this between ticks.
   132      requested_terminate: AtomicBool,
   133
   134      /// The worker's `jsc.VirtualMachine`, or null before `startVM()` / after
   135      /// `shutdown()` nulls it. Lives inside `arena`. `vm_lock` must be held for
   136      /// any cross-thread read (see header comment).
   137      ///
   138      /// `Cell` because this is read through `&WebWorker` on the parent / main
   139      /// thread (`notify_need_termination`, `terminate_all_and_wait`, `exit`) and
   140      /// written on the worker thread (`start_vm`, `shutdown`) — `vm_lock`
   141      /// serialises the memory ops, but Rust's aliasing model still requires
   142      /// interior mutability for a field written while a `&WebWorker` may be
   143      /// live. `*mut T` is `Copy`, so `Cell` gives safe `.get()`/`.set()`/
   144      /// `.replace()` and no `unsafe` at the access sites.
   145      vm: Cell<*mut VirtualMachine>,
   146      vm_lock: Mutex,
   147
   148      // ---- Parent-thread only -------------------------------------------------
   149      /// Keep-alive on the parent's event loop. `Async.KeepAlive` is not
   150      /// thread-safe; it is reffed in `create()`, toggled by `setRef()` (JS
   151      /// `.ref()`/`.unref()`), and released by `releaseParentPollRef()` from the
   152      /// close task — all on the parent thread.
   153      ///
   154      /// `JsCell` because all parent-thread FFI exports take `*mut WebWorker`
   155      /// (the worker thread may concurrently hold `&WebWorker`); we mutate this
   156      /// field through a shared-provenance pointer. Parent-thread-only access
   157      /// satisfies `JsCell`'s single-owner-thread invariant (same as `arena`
   158      /// below for the worker thread).
   159      parent_poll_ref: JsCell<KeepAlive>,
   160
   161      // ---- Worker-thread only -------------------------------------------------
   162      // These are mutated only on the worker thread, but the worker-thread call
   163      // chain takes `&self` (NOT `&mut self`) because the parent / main thread
   164      // may concurrently hold `&WebWorker` (`notify_need_termination`,
   165      // `terminate_all_and_wait`); materialising `&mut WebWorker` on the worker
   166      // thread while another thread holds `&WebWorker` is aliased-&mut UB. Hence
   167      // `Cell` / `UnsafeCell` even for single-threaded data.
   168      status: Cell<Status>,
   169      // PERF(port): was MimallocArena bulk-free backing the worker VM — keep as
   170      // explicit arena rather than deleting per §Allocators non-AST rule, because
   171      // the VM's allocator IS this arena (load-bearing). Profile if it shows up on a hot path.
   172      // `JsCell` (not `Cell`) because `Arena` is non-`Copy`; worker-thread-only
   173      // so the single-owner-thread invariant `JsCell` documents is upheld.
   174      arena: JsCell<Option<bun_alloc::Arena>>,
   175      /// Heap-owned cloned env (Map + Loader) for the worker VM. In Zig both
   176      /// were `allocator.create`'d on the worker arena and bulk-freed by
   177      /// `arena.deinit()`. Rust's `Arena = bumpalo::Bump` does not run `Drop`
   178      /// (so the inner `HashTable` would leak), and `clone_with_allocator()` no
   179      /// longer routes through the arena allocator anyway — own them as `Box`es
   180      /// here instead. `start_vm()` `heap::alloc`s and stores the pointers;
   181      /// `shutdown()` step 5 `heap::take`s after `vm.destroy()` (loader
   182      /// first, then map — `Loader<'static>` borrows `*map`).
   183      worker_env_map: Cell<*mut bun_dotenv::Map>,
   184      worker_env_loader: Cell<*mut bun_dotenv::Loader<'static>>,
   185      /// Set by `exit()` so that `spin()`'s error paths don't clobber an explicit
   186      /// `process.exit(code)`. Atomic so `exit()` can take `&self` (the struct is
   187      /// observed concurrently by `terminate_all_and_wait` / parent-thread FFI;
   188      /// producing `&mut WebWorker` while another thread holds `&WebWorker` is UB).
   189      exit_called: AtomicBool,
   190  }
   191
   192  #[repr(u8)]
   193  #[derive(Copy, Clone, Eq, PartialEq, strum::IntoStaticStr)]
   194  pub enum Status {
   195      /// Thread not yet started / startVM in progress.
   196      Start,
   197      /// `spin()` has begun; entry point is loading.
   198      Starting,
   199      /// `dispatchOnline` has fired; event loop is running.
   200      Running,
   201      /// `shutdown()` has begun; no further JS will run.
   202      Terminated,
   203  }

## vendor/bun/src/jsc/web_worker.rs:233-396

- Full-file Git blob: f39d9d806bea3c4eeb7a703ccbaaf9e8bdac886c
- Full-file SHA-256: d5c322bfea520b0743a3c3e063f77024b7f3263200895b2bdd58f87663c5d57b
- Full-file bytes: 80981
- Excerpt line span: 233-396
- Excerpt SHA-256: 1b14cf8141c4af69c055f3a2b4e5323e2ffb3cf18433468fdf0ec6164b19be17

   233  /// Process-global registry of worker threads that have been spawned and
   234  /// have not yet reached the point in `shutdown()` where they are past all
   235  /// process-global resolver access (BSSMap singletons like `dir_cache`).
   236  /// `globalExit()` uses this to terminate and wait for workers before
   237  /// `transpiler.deinit()` frees those singletons.
   238  ///
   239  /// Lock ordering: `LiveWorkers.mutex` → `worker.vm_lock` (never the reverse).
   240  mod live_workers {
   241      use super::*;
   242
   243      // PORT NOTE: `Mutex::new()` is the prevailing const-init spelling across
   244      // un-gated jsc modules (ConsoleObject.rs, bundler/ThreadPool.rs); the
   245      // `bun_threading` crate provides it.
   246      pub(super) static MUTEX: Mutex = Mutex::new();
   247      // TODO(port): std.DoublyLinkedList — intrusive, nodes are `WebWorker.live_{next,prev}`
   248      // PORTING.md §Global mutable state: list head, every read/write is under
   249      // `MUTEX` above. `AtomicCell` so the slot itself is `Sync` with safe
   250      // load/store (the mutex still provides the actual happens-before for the
   251      // intrusive list walk; Zig: plain `var head: ?*WebWorker`).
   252      pub(super) static HEAD: bun_core::AtomicCell<*mut WebWorker> =
   253          bun_core::AtomicCell::new(core::ptr::null_mut());
   254      /// Number of workers registered in `list`. Separate atomic so
   255      /// `terminateAllAndWait` can futex-wait on it without the mutex.
   256      pub(super) static OUTSTANDING: AtomicU32 = AtomicU32::new(0);
   257
   258      pub(super) fn register(worker: *mut WebWorker) {
   259          MUTEX.lock();
   260          let head = HEAD.load();
   261          // SAFETY: MUTEX held; `worker` is a valid heap allocation owned by C++.
   262          unsafe {
   263              (*worker).live_prev.set(core::ptr::null_mut());
   264              (*worker).live_next.set(head);
   265              if !head.is_null() {
   266                  (*head).live_prev.set(worker);
   267              }
   268          }
   269          HEAD.store(worker);
   270          // fetch_add and wake MUST happen under MUTEX (matching the Zig
   271          // `defer mutex.unlock()` ordering) so that `terminate_all_and_wait`
   272          // can never observe the worker in the list while OUTSTANDING is still
   273          // at its pre-increment value — otherwise it could sweep B, see
   274          // OUTSTANDING==0 (A's unregister already ran, B's add hasn't), and
   275          // return early while B is still starting.
   276          OUTSTANDING.fetch_add(1, Ordering::Release);
   277          // Wake terminateAllAndWait so it re-sweeps and catches this worker
   278          // (it may have been created by another worker mid-sweep). No-op if
   279          // nothing is waiting.
   280          Futex::wake(&OUTSTANDING, 1);
   281          MUTEX.unlock();
   282      }
   283
   284      // `*const WebWorker` (not `*mut`): called from `shutdown(&self)` while
   285      // other threads may hold `&WebWorker`, so the caller only has shared-ref
   286      // provenance. All writes here go through `Cell` fields
   287      // (`live_next`/`live_prev`), which is sound via shared provenance.
   288      pub(super) fn unregister(worker: *const WebWorker) {
   289          MUTEX.lock();
   290          // SAFETY: MUTEX held; node was registered in `register`.
   291          unsafe {
   292              let prev = (*worker).live_prev.get();
   293              let next = (*worker).live_next.get();
   294              if !prev.is_null() {
   295                  (*prev).live_next.set(next);
   296              } else {
   297                  HEAD.store(next);
   298              }
   299              if !next.is_null() {
   300                  (*next).live_prev.set(prev);
   301              }
   302              (*worker).live_prev.set(core::ptr::null_mut());
   303              (*worker).live_next.set(core::ptr::null_mut());
   304          }
   305          MUTEX.unlock();
   306          // Wake any waiter in terminateAllAndWait when we hit zero. Waking
   307          // unconditionally is fine (spurious wakeups just re-check the
   308          // counter) and avoids a compare-before-wake race.
   309          OUTSTANDING.fetch_sub(1, Ordering::Release);
   310          Futex::wake(&OUTSTANDING, 1);
   311      }
   312  }
   313
   314  /// Request termination of every live worker and block until each has reached
   315  /// `shutdown()` (past all process-global resolver access), or `timeout_ms`
   316  /// elapses. Called from `VirtualMachine.globalExit()` on the main thread
   317  /// before `transpiler.deinit()` frees the process-global BSSMap singletons —
   318  /// without this, a detached worker still in `startVM()`/`spin()` would UAF on
   319  /// `dir_cache` / `dirname_store` etc.
   320  ///
   321  /// This is the `Environment::stop_sub_worker_contexts()` equivalent for the
   322  /// main thread; nested workers (a worker's own sub-workers at the worker's
   323  /// exit) remain the documented gap.
   324  ///
   325  /// Termination is cooperative: `requested_terminate` is polled at
   326  /// checkpoints throughout `startVM()` and `spin()`, and for a running VM
   327  /// `notifyNeedTermination()` raises a TerminationException at the next JSC
   328  /// safepoint. We do NOT use `thread_suspend`/`SuspendThread` — a worker
   329  /// frozen mid-mimalloc-alloc or holding the `dir_cache` mutex would
   330  /// deadlock/corrupt the very cleanup we're trying to make safe.
   331  pub fn terminate_all_and_wait(timeout_ms: u64) {
   332      if live_workers::OUTSTANDING.load(Ordering::Acquire) == 0 {
   333          return;
   334      }
   335
   336      // Futex-wait on the counter so we sleep rather than burn a core. Each
   337      // unregister() wakes us; we re-check and re-wait until zero or deadline.
   338      // We re-sweep the list on EVERY iteration: a worker A that was mid-
   339      // `WebWorker__create` for a nested worker B when we first swept will
   340      // register B after we release the mutex, and B's `requested_terminate`
   341      // was never set. Sweeping is O(outstanding) and `requested_terminate`
   342      // is a swap, so re-sweeping already-terminated entries is cheap.
   343      let timer = std::time::Instant::now();
   344      let deadline_ns: u64 = timeout_ms * 1_000_000; // std.time.ns_per_ms
   345      loop {
   346          live_workers::MUTEX.lock();
   347          // MUTEX held while walking the intrusive list; HEAD load is safe.
   348          let mut it = live_workers::HEAD.load();
   349          while let Some(nn) = NonNull::new(it) {
   350              // Worker valid while registered (removed only in shutdown());
   351              // MUTEX held — `ParentRef` invariant (pointee outlives borrow) holds.
   352              let w = bun_ptr::ParentRef::from(nn);
   353              // live_workers::MUTEX held; list links written only under it.
   354              it = w.live_next.get();
   355              if w.requested_terminate.swap(true, Ordering::Release) {
   356                  continue;
   357              }
   358              w.vm_lock.lock();
   359              // vm_lock held; `vm` is published/unpublished under vm_lock.
   360              let vm_ptr = w.vm_ptr();
   361              if !vm_ptr.is_null() {
   362                  // SAFETY: vm_ptr published under vm_lock and non-null here.
   363                  // jsc_vm is a valid JSC::VM*; notify_need_termination is
   364                  // documented thread-safe (VMTraps). Cast through the real
   365                  // opaque `crate::VM` (the `crate::VM` stub is layout-only).
   366                  // We deliberately do NOT bind `&VirtualMachine` — the worker
   367                  // thread may hold a live mutable view of the VM; raw-pointer
   368                  // field/method access keeps any autoref scoped to the access.
   369                  unsafe { (*(*vm_ptr).jsc_vm.cast_const()).notify_need_termination() };
   370                  // SAFETY: event_loop() returns the live `*mut EventLoop` self-ptr.
   371                  unsafe { (*(*vm_ptr).event_loop()).wakeup() };
   372              }
   373              w.vm_lock.unlock();
   374          }
   375          live_workers::MUTEX.unlock();
   376
   377          let n = live_workers::OUTSTANDING.load(Ordering::Acquire);
   378          if n == 0 {
   379              return;
   380          }
   381          let elapsed = u64::try_from(timer.elapsed().as_nanos()).unwrap_or(u64::MAX);
   382          if elapsed >= deadline_ns {
   383              log!("terminateAllAndWait: timed out with {} outstanding", n);
   384              return;
   385          }
   386          let _ = Futex::wait(&live_workers::OUTSTANDING, n, Some(deadline_ns - elapsed));
   387      }
   388  }
   389
   390  #[unsafe(no_mangle)]
   391  pub extern "C" fn WebWorker__getParentWorker(vm: &VirtualMachine) -> *mut c_void {
   392      vm.worker_ref()
   393          .map(|w| w.cpp_worker)
   394          .unwrap_or(core::ptr::null_mut())
   395  }
   396

## vendor/bun/src/jsc/web_worker.rs:540-710

- Full-file Git blob: f39d9d806bea3c4eeb7a703ccbaaf9e8bdac886c
- Full-file SHA-256: d5c322bfea520b0743a3c3e063f77024b7f3263200895b2bdd58f87663c5d57b
- Full-file bytes: 80981
- Excerpt line span: 540-710
- Excerpt SHA-256: c60f7edecc41ee689ca8802d9ac75fd4998f4942fffc94644e0755a48ebb9799

   540              exec_argv_ptr,
   541              exec_argv_len,
   542              inherit_exec_argv,
   543              unresolved_specifier: spec_slice.slice().to_vec().into_boxed_slice(),
   544              preloads,
   545              name: if name_str.is_empty() {
   546                  bun_core::ZBox::default()
   547              } else {
   548                  name_str.to_owned_slice_z()
   549              },
   550              live_next: Cell::new(core::ptr::null_mut()),
   551              live_prev: Cell::new(core::ptr::null_mut()),
   552              requested_terminate: AtomicBool::new(false),
   553              vm: Cell::new(core::ptr::null_mut()),
   554              vm_lock: Mutex::new(),
   555              parent_poll_ref: JsCell::new(KeepAlive::init()),
   556              status: Cell::new(Status::Start),
   557              arena: JsCell::new(None),
   558              worker_env_map: Cell::new(core::ptr::null_mut()),
   559              worker_env_loader: Cell::new(core::ptr::null_mut()),
   560              exit_called: AtomicBool::new(false),
   561          }));
   562          // `worker` is non-null (just heap-allocated). Wrap once for the safe
   563          // shared reborrows below; the raw `worker` is still used for
   564          // `register`/`destroy`/the FFI return value.
   565          let worker_ref =
   566              bun_ptr::ParentRef::from(NonNull::new(worker).expect("heap::into_raw is non-null"));
   567
   568          // Keep the parent's event loop alive until the close task releases this.
   569          // If the user passed `{ ref: false }` we skip — they've opted out of the
   570          // worker keeping the process alive. Exception: a nested worker (parent is
   571          // itself a worker, not joined on exit) must hold the parent-loop keepalive
   572          // regardless, because the child holds a non-owning `BackRef` to the parent VM.
   573          if !default_unref || parent_ref.worker_ref().is_some() {
   574              // `worker` is a fresh heap allocation; not yet shared.
   575              // `bun_io::js_vm_ctx()` resolves to this (parent) thread's loop.
   576              worker_ref.with_parent_poll_ref(|p| p.ref_(bun_io::js_vm_ctx()));
   577          }
   578
   579          // Register BEFORE spawning so terminateAllAndWait() can never miss a
   580          // worker whose thread is already running.
   581          live_workers::register(worker);
   582
   583          // PORT NOTE: Zig's `std.Thread.spawn(.{ .stack_size }, threadMain, .{worker})`.
   584          // `std::thread` is permitted (only `std::{fs,net,process}` are banned);
   585          // bun_threading has no generic spawn helper.
   586          struct SendPtr(*mut WebWorker);
   587          // SAFETY: `WebWorker` is heap-allocated and the worker thread is the
   588          // sole writer to its worker-thread-only fields; cross-thread fields are
   589          // atomic/locked. The pointer is moved into the new thread exactly once.
   590          unsafe impl Send for SendPtr {}
   591          let send = SendPtr(worker);
   592          let spawn = std::thread::Builder::new()
   593              .stack_size(bun_threading::thread_pool::DEFAULT_THREAD_STACK_SIZE as usize)
   594              .spawn(move || {
   595                  let send = send;
   596                  // SAFETY: `send.0` is a valid heap `WebWorker` owned by C++;
   597                  // `&WebWorker` (not `&mut`) — see worker-thread `&self` note.
   598                  unsafe { (*send.0).thread_main() };
   599              });
   600          match spawn {
   601              Ok(handle) => {
   602                  // Detach: see "Known gap" in the file header.
   603                  drop(handle);
   604                  worker
   605              }
   606              Err(_) => {
   607                  live_workers::unregister(worker);
   608                  // `worker` not yet shared (spawn failed); parent thread.
   609                  worker_ref.with_parent_poll_ref(|p| p.unref(bun_io::js_vm_ctx()));
   610                  Self::destroy(worker);
   611                  *error_message = BunString::static_(b"Failed to spawn worker thread");
   612                  core::ptr::null_mut()
   613              }
   614          }
   615      }
   616
   617      /// Free the struct and its owned strings. Called from
   618      /// `WebCore::Worker::~Worker()` (or from `create()` on spawn failure). The
   619      /// allocator is mimalloc (thread-safe), so the caller's thread doesn't
   620      /// matter.
   621      #[unsafe(export_name = "WebWorker__destroy")]
   622      pub extern "C" fn destroy(this: *mut WebWorker) {
   623          // SAFETY: this was heap-allocated in create(); C++ owns it and calls
   624          // destroy exactly once.
   625          let this = unsafe { bun_core::heap::take(this) };
   626          log!("[{}] destroy", this.execution_context_id);
   627          // unresolved_specifier / preloads / name freed by Drop.
   628          drop(this);
   629      }
   630
   631      // =========================================================================
   632      // Parent-thread API (called from C++ via JS)
   633      // =========================================================================
   634
   635      /// worker.ref()/.unref() from JS. The struct is guaranteed alive: it's
   636      /// freed by `~Worker`, which can't run while JSWorker (the caller) holds
   637      /// its `Ref<Worker>`. `Worker::setKeepAlive()` gates out calls after
   638      /// terminate() or the close task, so this can unconditionally toggle.
   639      ///
   640      /// Takes `*mut` (not `&mut`) because the worker thread concurrently
   641      /// dereferences this struct; materialising `&mut WebWorker` here would be
   642      /// aliased-&mut UB.
   643      #[unsafe(export_name = "WebWorker__setRef")]
   644      pub extern "C" fn set_ref(this: *mut WebWorker, value: bool) {
   645          // `this` is a valid heap allocation owned by C++ `WebCore::Worker`
   646          // (alive while JSWorker holds its Ref) — `ParentRef` invariant holds.
   647          // `bun_io::js_vm_ctx()` resolves to this (parent) thread's loop, which
   648          // IS `this.parent`'s loop.
   649          let this = bun_ptr::ParentRef::from(NonNull::new(this).expect("WebWorker FFI ptr"));
   650          // A nested worker (parent is itself a worker) must keep the parent-loop
   651          // keepalive even on `.unref()`: the child holds a non-owning `BackRef` to
   652          // the parent VM and worker parents aren't joined on exit.
   653          let parent_is_worker = this.parent.get().worker_ref().is_some();
   654          this.with_parent_poll_ref(|poll| {
   655              if value {
   656                  poll.ref_(bun_io::js_vm_ctx());
   657              } else if !parent_is_worker {
   658                  poll.unref(bun_io::js_vm_ctx());
   659              }
   660          });
   661      }
   662
   663      /// worker.terminate() from JS. Sets `requested_terminate`, interrupts
   664      /// running JS in the worker (TerminationException at the next safepoint),
   665      /// and wakes the worker loop so it observes the flag. `parent_poll_ref`
   666      /// stays held until the close task runs so that `await worker.terminate()`
   667      /// keeps the parent alive until 'close' fires.
   668      ///
   669      /// Takes `*mut` (not `&mut`) because the worker thread concurrently
   670      /// dereferences this struct (polling `requested_terminate`, holding
   671      /// `vm_lock`, reading `vm`); materialising `&mut WebWorker` on the parent
   672      /// thread while the worker holds any reference is aliased-&mut UB.
   673      #[unsafe(export_name = "WebWorker__notifyNeedTermination")]
   674      pub extern "C" fn notify_need_termination(this: *mut WebWorker) {
   675          // `this` is a valid heap allocation owned by C++ `WebCore::Worker`
   676          // (alive while JSWorker holds its Ref) — `ParentRef` invariant holds.
   677          // Only atomic / lock-guarded fields are touched cross-thread; never
   678          // `&mut WebWorker`.
   679          let this = bun_ptr::ParentRef::from(NonNull::new(this).expect("WebWorker FFI ptr"));
   680          if this.set_requested_terminate() {
   681              return;
   682          }
   683          log!("[{}] notifyNeedTermination", this.execution_context_id);
   684
   685          // vm_lock serialises against shutdown() nulling `vm` and freeing the
   686          // arena it lives in.
   687          this.vm_lock.lock();
   688          // vm_lock held; `vm` is published/unpublished under vm_lock.
   689          let vm_ptr = this.vm_ptr();
   690          if !vm_ptr.is_null() {
   691              // SAFETY: vm_ptr published under vm_lock and non-null here.
   692              // jsc_vm is a valid JSC::VM*; notify_need_termination is
   693              // documented thread-safe (VMTraps). Cast through the real opaque
   694              // `crate::VM` (the `crate::VM` stub is layout-only). No
   695              // `&VirtualMachine` binding — see `terminate_all_and_wait`.
   696              unsafe { (*(*vm_ptr).jsc_vm.cast_const()).notify_need_termination() };
   697              // SAFETY: event_loop() returns the live `*mut EventLoop` self-ptr.
   698              unsafe { (*(*vm_ptr).event_loop()).wakeup() };
   699          }
   700          this.vm_lock.unlock();
   701      }
   702
   703      /// Release the keep-alive on the parent's event loop. Called on the parent
   704      /// thread from the close task posted by `dispatchExit`.
   705      ///
   706      /// Takes `*mut` for consistency with the other parent-thread FFI exports
   707      /// (the worker thread has exited by the time this runs, so `&mut` would be
   708      /// sound here, but matching signatures avoids surprises).
   709      #[unsafe(export_name = "WebWorker__releaseParentPollRef")]
   710      pub extern "C" fn release_parent_poll_ref(this: *mut WebWorker) {

## vendor/bun/src/jsc/web_worker.rs:990-1346

- Full-file Git blob: f39d9d806bea3c4eeb7a703ccbaaf9e8bdac886c
- Full-file SHA-256: d5c322bfea520b0743a3c3e063f77024b7f3263200895b2bdd58f87663c5d57b
- Full-file bytes: 80981
- Excerpt line span: 990-1346
- Excerpt SHA-256: ebffc655077eb915286682923de66767629a551fcbef890530db9439086d49be

   990          // spin() will observe the flag and shutdown() under the API lock.
   991          if self.has_requested_terminate() {
   992              return Ok(vm);
   993          }
   994
   995          // SAFETY: see post-publish note above.
   996          unsafe {
   997              if (*vm).transpiler.configure_defines().is_err() {
   998                  // Fall through to spin() → shutdown() for full teardown under
   999                  // the API lock (flushLogs runs JS). Set terminate so spin()
  1000                  // bails immediately; vm.log carries the error for flushLogs.
  1001                  (*vm).exit_handler.exit_code = 1;
  1002                  let _ = self.set_requested_terminate();
  1003                  return Ok(vm);
  1004              }
  1005
  1006              (*vm).load_extra_env_and_source_code_printer();
  1007          }
  1008          Ok(vm)
  1009      }
  1010
  1011      /// Phase 2: load the entry point, dispatch 'online', run the event loop.
  1012      /// Runs inside `holdAPILock`. Always ends by calling `shutdown()`.
  1013      ///
  1014      /// PORT NOTE: Zig's `spin` is `noreturn` (every path ends in `shutdown`
  1015      /// → `bun.exitThread`). The Rust port returns `()` so the thread can
  1016      /// unwind-free fall out of the `extern "C"` trampoline — see `shutdown`.
  1017      fn spin(&self) {
  1018          log!("[{}] spin start", self.execution_context_id);
  1019
  1020          // vm published in start_vm; non-null past this point. Do NOT bind a
  1021          // long-lived `&mut VirtualMachine`: while the event loop runs, the
  1022          // parent / main thread may dereference the same pointer under
  1023          // `vm_lock` (`notify_need_termination`, `terminate_all_and_wait`).
  1024          // Those cross-thread paths only form raw-ptr field reads (never
  1025          // `&mut VirtualMachine`), so holding `&VirtualMachine` here is sound;
  1026          // mutation goes through `vm.as_mut()` which forms a fresh short-lived
  1027          // `&mut` per call (the `JsCell` escape hatch — provenance from the
  1028          // thread-local `*mut`).
  1029          let vm_ptr: *mut VirtualMachine = self.vm_ptr();
  1030          // vm published in `start_vm` under `vm_lock`; non-null and live for the
  1031          // worker thread's duration. This IS the worker thread's per-thread VM
  1032          // (set by `VirtualMachine::init` → `VMHolder`), so the safe
  1033          // thread-local accessor returns the same allocation.
  1034          debug_assert!(core::ptr::eq(vm_ptr, VirtualMachine::get_mut_ptr()));
  1035          let vm: &VirtualMachine = VirtualMachine::get();
  1036          debug_assert!(self.status.get() == Status::Start);
  1037          self.set_status(Status::Starting);
  1038
  1039          // Terminated during startVM() (or startVM() short-circuited here on
  1040          // configureDefines failure) — shut down under the API lock so the
  1041          // JSC::VM built by initWorker is torn down rather than leaked.
  1042          if self.has_requested_terminate() {
  1043              self.flush_logs(vm);
  1044              return self.shutdown();
  1045          }
  1046
  1047          // `preloads` is owned by `self` (heap `WebWorker` outlives the VM).
  1048          // PORT NOTE: Zig's slice-copy assignment; here `preload: Vec<Box<[u8]>>`
  1049          // so clone the boxes (cheap, ≤handful).
  1050          vm.as_mut().preload = self.preloads.clone();
  1051
  1052          // Resolve the entry point on the worker thread (the parent only stored
  1053          // the raw specifier). The returned slice is BORROWED — every exit from
  1054          // spin() goes through shutdown() which is noreturn, so a `defer free`
  1055          // here would never run anyway.
  1056          let mut resolve_error = BunString::empty();
  1057          let vm_log = vm.log_mut().unwrap();
  1058          // SAFETY: `vm_ptr` is the live worker-thread VM; the fn takes a raw ptr
  1059          // (no `&mut`) because `vm` is already published under `vm_lock` — see
  1060          // `resolve_entry_point_specifier` Safety contract.
  1061          let path = match unsafe {
  1062              resolve_entry_point_specifier(
  1063                  vm_ptr,
  1064                  &self.unresolved_specifier,
  1065                  &mut resolve_error,
  1066                  vm_log,
  1067              )
  1068          } {
  1069              Some(p) => p,
  1070              None => {
  1071                  vm.as_mut().exit_handler.exit_code = 1;
  1072                  if vm_log.errors == 0 && !resolve_error.is_empty() {
  1073                      let err = resolve_error.to_utf8();
  1074                      // `Log::add_error` takes `impl IntoText`; pass an owned
  1075                      // `Vec<u8>` so the `Msg` owns its bytes (no lifetime tie
  1076                      // to `err`, which is dropped immediately after).
  1077                      vm_log.add_error(None, bun_ast::Loc::EMPTY, err.slice().to_vec());
  1078                  }
  1079                  resolve_error.deref();
  1080                  self.flush_logs(vm);
  1081                  return self.shutdown();
  1082              }
  1083          };
  1084          resolve_error.deref();
  1085
  1086          // Terminated while resolving — exit code 0, no error.
  1087          if self.has_requested_terminate() {
  1088              self.flush_logs(vm);
  1089              return self.shutdown();
  1090          }
  1091
  1092          // `path` borrows the resolver's process-lifetime string store, the
  1093          // standalone module graph, or `self.unresolved_specifier` — all of
  1094          // which outlive the worker VM. `vm.main` stores it as a raw BACKREF
  1095          // (see `VirtualMachine::set_main`); no lifetime extension needed.
  1096          let promise = match vm.as_mut().load_entry_point_for_web_worker(path) {
  1097              Ok(p) => p,
  1098              Err(_) => {
  1099                  // process.exit() may have run during load; don't clobber its code.
  1100                  if !self.exit_called.load(Ordering::Relaxed) {
  1101                      vm.as_mut().exit_handler.exit_code = 1;
  1102                  }
  1103                  self.flush_logs(vm);
  1104                  return self.shutdown();
  1105              }
  1106          };
  1107
  1108          // SAFETY: `promise` is a live JSC heap cell.
  1109          unsafe {
  1110              if (*promise).status() == jsc::js_promise::Status::Rejected {
  1111                  let handled = vm.as_mut().uncaught_exception(
  1112                      vm.global(),
  1113                      (*promise).result(vm.jsc_vm()),
  1114                      true,
  1115                  );
  1116                  if !handled {
  1117                      vm.as_mut().exit_handler.exit_code = 1;
  1118                      return self.shutdown();
  1119                  }
  1120              } else {
  1121                  let _ = (*promise).result(vm.jsc_vm());
  1122              }
  1123          }
  1124
  1125          self.flush_logs(vm);
  1126          log!("[{}] event loop start", self.execution_context_id);
  1127          // dispatchOnline fires the parent-side 'open' event and flips the C++
  1128          // state to Running (which routes postMessage directly instead of
  1129          // queuing). It is placed after the entry point has loaded so the parent
  1130          // observes 'online' only once the worker's top-level code has completed;
  1131          // moving it earlier would change that observable ordering.
  1132          // `cpp_worker` is the opaque C++-owned handle round-tripped via `safe fn`;
  1133          // `vm.global()` yields the live `&JSGlobalObject` published in start_vm.
  1134          WebWorker__dispatchOnline(self.cpp_worker, vm.global());
  1135          WebWorker__fireEarlyMessages(self.cpp_worker, vm.global());
  1136          self.set_status(Status::Running);
  1137
  1138          // don't run the GC if we don't actually need to
  1139          if vm.is_event_loop_alive() || vm.event_loop_mut().tick_concurrent_with_count() > 0 {
  1140              vm.global().vm().release_weak_refs();
  1141              // PERF(port): `vm.arena.gc()` was `MimallocArena.gc()` →
  1142              // `mi_heap_collect`. `Arena = bumpalo::Bump` has no collect;
  1143              // global mimalloc handles reclamation. Profile if it shows up on a hot path.
  1144              let _ = vm.global().vm().run_gc(false);
  1145          }
  1146
  1147          // Always do a first tick so we call CppTask without delay after
  1148          // dispatchOnline.
  1149          vm.as_mut().tick();
  1150
  1151          while vm.is_event_loop_alive() {
  1152              vm.as_mut().tick();
  1153              if self.has_requested_terminate() {
  1154                  break;
  1155              }
  1156              vm.as_mut().auto_tick_active();
  1157              if self.has_requested_terminate() {
  1158                  break;
  1159              }
  1160          }
  1161
  1162          log!(
  1163              "[{}] before exit {}",
  1164              self.execution_context_id,
  1165              if self.has_requested_terminate() {
  1166                  "(terminated)"
  1167              } else {
  1168                  "(event loop dead)"
  1169              }
  1170          );
  1171
  1172          // Only emit 'beforeExit' on a natural drain, not on terminate().
  1173          if !self.has_requested_terminate() {
  1174              // TODO: is this able to allow the event loop to continue?
  1175              vm.as_mut().on_before_exit();
  1176          }
  1177
  1178          self.flush_logs(vm);
  1179          self.shutdown();
  1180      }
  1181
  1182      /// Phase 3: run exit handlers, tear down the JSC VM, post the close
  1183      /// event, free the arena, exit the thread.
  1184      ///
  1185      /// Ordering constraints (each step is a barrier for the next):
  1186      ///   1. `vm = null` under lock    — a racing notifyNeedTermination() now sees
  1187      ///                                  null and skips wakeup() instead of touching
  1188      ///                                  memory freed in step 5.
  1189      ///   2. `vm.onExit()`             — user 'exit' handlers run; needs the JSC VM.
  1190      ///   3. `teardownJSCVM()`         — collectNow + vm.deref (single — Zig
  1191      ///                                  derefs ×2 because `JSLockHolder` holds
  1192      ///                                  a `RefPtr<VM>`; the Rust API-lock path
  1193      ///                                  takes no extra ref, see `thread_main`
  1194      ///                                  PORT NOTE); can re-enter via
  1195      ///                                  finalizers, so must precede step 5.
  1196      ///   4. `dispatchExit()`          — posts close task → parent releases
  1197      ///                                  parent_poll_ref + thread-held Worker ref.
  1198      ///                                  After this `this` may be freed at any time.
  1199      ///   5. free loop/arena/pools     — no `this.*` dereferences below step 4.
  1200      ///
  1201      /// Does NOT free `this` — see ownership rule in the file header.
  1202      ///
  1203      /// PORT NOTE: Zig's `shutdown` is `noreturn` (ends in `bun.exitThread`).
  1204      /// The Rust port returns `()` and lets the thread fall out of the spawn
  1205      /// closure instead — see the note at the bottom of this fn.
  1206      fn shutdown(&self) {
  1207          jsc::mark_binding();
  1208          self.set_status(Status::Terminated);
  1209          bun_analytics::features::workers_terminated.fetch_add(1, Ordering::Relaxed);
  1210          log!("[{}] shutdown", self.execution_context_id);
  1211
  1212          // Snapshot everything we'll need after `this` may be freed (step 4).
  1213          let cpp_worker = self.cpp_worker;
  1214          // worker-thread only field; no other thread reads `arena`.
  1215          let mut arena = self.arena.replace(None);
  1216          let env_loader = self.worker_env_loader.replace(core::ptr::null_mut());
  1217          let env_map = self.worker_env_map.replace(core::ptr::null_mut());
  1218
  1219          // ---- 1. Unpublish vm ------------------------------------------------
  1220          self.vm_lock.lock();
  1221          // vm_lock held; this is the unpublish point.
  1222          let vm_ptr = self.vm.replace(core::ptr::null_mut());
  1223          self.vm_lock.unlock();
  1224          let mut loop_: Option<*mut bun_uws::Loop> = None;
  1225          if !vm_ptr.is_null() {
  1226              // SAFETY: vm_ptr was published under vm_lock; sole owner now.
  1227              loop_ = Some(unsafe { &*vm_ptr }.uws_loop());
  1228          }
  1229
  1230          // ---- 2. User exit handlers -----------------------------------------
  1231          let mut exit_code: i32 = 0;
  1232          let mut global_object: Option<*const JSGlobalObject> = None;
  1233          if !vm_ptr.is_null() {
  1234              // SAFETY: vm_ptr valid; unpublished above under vm_lock, so no
  1235              // other thread can dereference it now — `&mut` is exclusive.
  1236              let vm = unsafe { &mut *vm_ptr };
  1237              // terminate() set the JSC termination flag to interrupt running JS;
  1238              // clear it so process.on('exit') handlers can run. teardownJSCVM
  1239              // re-sets it for the JSC VM teardown.
  1240              vm.jsc_vm().clear_has_termination_request();
  1241              vm.is_shutting_down = true;
  1242              vm.on_exit();
  1243              if let Some(hooks) = runtime_hooks() {
  1244                  (hooks.cron_clear_all_teardown)(vm);
  1245              }
  1246              // Embedded socket groups must drain while JSC is still alive —
  1247              // closeAll() fires on_close → JS callbacks. RareData.deinit() runs
  1248              // after teardownJSCVM and only deinit()s (asserts empty in debug).
  1249              if let Some(rare) = vm.rare_data.as_deref_mut() {
  1250                  // PORT NOTE: reshaped for borrowck — `close_all_socket_groups`
  1251                  // wants `&VirtualMachine` while `rare` is `&mut` borrowed from
  1252                  // `vm`. Re-derive `vm` through the raw ptr (sole owner).
  1253                  rare.close_all_socket_groups(unsafe { &*vm_ptr });
  1254              }
  1255              exit_code = i32::from(vm.exit_handler.exit_code);
  1256              global_object = Some(vm.global);
  1257          }
  1258
  1259          // ---- 3. JSC VM teardown --------------------------------------------
  1260          if let Some(global) = global_object {
  1261              // `JSGlobalObject` is an opaque ZST handle; `opaque_ref` is the
  1262              // centralised non-null deref proof (JSC VM still alive here).
  1263              WebWorker__teardownJSCVM(JSGlobalObject::opaque_ref(global));
  1264          }
  1265
  1266          // JSC is down; no more resolver/module-loader access past this point.
  1267          // Unregister so the main thread's terminateAllAndWait() can proceed to
  1268          // free process-global resolver state. Must happen before dispatchExit
  1269          // because `this` may be freed once that posts.
  1270          live_workers::unregister(self);
  1271
  1272          // ---- 4. Post close task to parent ----------------------------------
  1273          // `cpp_worker` is the opaque C++-owned handle (snapshot taken above).
  1274          WebWorker__dispatchExit(cpp_worker, exit_code);
  1275          // `this` may be freed past this point.
  1276
  1277          // ---- 5. Free worker-thread resources -------------------------------
  1278          if let Some(loop_) = loop_ {
  1279              // SAFETY: loop owned by this thread's VM; no concurrent access.
  1280              unsafe { (*loop_).internal_loop_data.jsc_vm = core::ptr::null_mut() };
  1281          }
  1282          if !vm_ptr.is_null() {
  1283              // SAFETY: vm_ptr valid; sole owner.
  1284              // Must precede Loop.shutdown so uv_close isn't called twice on the
  1285              // GC timer.
  1286              unsafe { (*vm_ptr).gc_controller.deinit() };
  1287          }
  1288          #[cfg(windows)]
  1289          {
  1290              // Per-thread libuv loop teardown; closes any handles still open on
  1291              // this worker's loop and drops the thread-local pointer.
  1292              bun_sys::windows::libuv::Loop::shutdown();
  1293          }
  1294          if !vm_ptr.is_null() {
  1295              // SAFETY: vm_ptr valid; sole owner. `destroy()` is the port of
  1296              // Zig `vm.deinit()`.
  1297              unsafe { (*vm_ptr).destroy() };
  1298          }
  1299          // Reclaim the cloned env (loader borrows `*map` — drop loader first).
  1300          // In Zig both lived on the worker arena and were bulk-freed below;
  1301          // here they were `heap::alloc`'d in `start_vm()` (see field doc).
  1302          if !env_loader.is_null() {
  1303              // SAFETY: `heap::alloc`'d in `start_vm`; sole owner; the VM is
  1304              // gone so its raw `transpiler.env` borrow is dead.
  1305              drop(unsafe { bun_core::heap::take(env_loader) });
  1306          }
  1307          if !env_map.is_null() {
  1308              // SAFETY: `heap::alloc`'d in `start_vm`; sole owner.
  1309              drop(unsafe { bun_core::heap::take(env_map) });
  1310          }
  1311          bun_core::delete_all_pools_for_thread_exit();
  1312          drop(arena.take());
  1313
  1314          // PORT NOTE: Zig calls `bun.exitThread()` (`pthread_exit`) here. In
  1315          // Rust we MUST NOT — glibc's `pthread_exit` throws a `__forced_unwind`
  1316          // C++ exception to run destructors, and unwinding that across an
  1317          // `extern "C"` (`nounwind`) Rust frame on the way out to
  1318          // `std::thread`'s entry point makes Rust abort the whole process.
  1319          // Instead return normally: `shutdown()` → `spin()` → `thread_main`
  1320          // (which `forget`s the API-lock guard) → the `std::thread` spawn
  1321          // closure, which then exits the thread cleanly. No `this.*` is
  1322          // touched past `dispatchExit` above, so the `this`-may-be-freed
  1323          // contract still holds across the unwind-free return path.
  1324      }
  1325
  1326      /// process.exit() inside the worker. Worker-thread only.
  1327      ///
  1328      /// Takes `&self` (not `&mut self`) because `terminate_all_and_wait` /
  1329      /// `notify_need_termination` may concurrently hold `&WebWorker` on another
  1330      /// thread; producing `&mut` here would be aliased-&mut UB.
  1331      pub fn exit(&self) {
  1332          self.exit_called.store(true, Ordering::Relaxed);
  1333          let _ = self.set_requested_terminate();
  1334          // Stop subsequent JS at the next safepoint. `this.vm` is null during
  1335          // `vm.onExit()` (shutdown nulls it first), so a re-entrant
  1336          // process.exit() from an exit handler does not re-arm the trap.
  1337          // worker-thread only; `vm` is read here on the same thread
  1338          // that publishes/unpublishes it, so no lock is needed for the load.
  1339          let vm_ptr = self.vm_ptr();
  1340          if !vm_ptr.is_null() {
  1341              // SAFETY: vm_ptr non-null; jsc_vm is a valid JSC::VM*;
  1342              // notify_need_termination is documented thread-safe (VMTraps).
  1343              // Cast through the real opaque `crate::VM`.
  1344              unsafe { (*(*vm_ptr).jsc_vm.cast_const()).notify_need_termination() };
  1345          }
  1346      }

## vendor/bun/src/jsc/bindings/BunProcess.cpp:280-304

- Full-file Git blob: 6935f0cbbcee9bf5f5f5364560a16707b9cc527e
- Full-file SHA-256: ec7a0a9d63d2bbd1549871c0e36b200336748a976c617ed593198bf5866a2872
- Full-file bytes: 193398
- Excerpt line span: 280-304
- Excerpt SHA-256: aea7e87d406c1c43f195a4704f752efc25c4c203138003999271f18f293d2451

   280  static void dispatchExitInternal(JSC::JSGlobalObject* globalObject, Process* process, int exitCode)
   281  {
   282      static bool processIsExiting = false;
   283      if (processIsExiting)
   284          return;
   285      processIsExiting = true;
   286      auto& emitter = process->wrapped();
   287      auto& vm = JSC::getVM(globalObject);
   288
   289      if (vm.hasTerminationRequest() || vm.hasExceptionsAfterHandlingTraps())
   290          return;
   291
   292      auto event = Identifier::fromString(vm, "exit"_s);
   293      if (!emitter.hasEventListeners(event)) {
   294          return;
   295      }
   296      process->putDirect(vm, Identifier::fromString(vm, "_exiting"_s), jsBoolean(true), 0);
   297
   298      MarkedArgumentBuffer arguments;
   299      arguments.append(jsNumber(exitCode));
   300      emitter.emit(event, arguments);
   301  }
   302
   303  JSC_DEFINE_CUSTOM_SETTER(Process_defaultSetter, (JSC::JSGlobalObject * globalObject, JSC::EncodedJSValue thisValue, JSC::EncodedJSValue value, JSC::PropertyName propertyName))
   304  {

## vendor/bun/src/jsc/bindings/BunProcess.cpp:1205-1247

- Full-file Git blob: 6935f0cbbcee9bf5f5f5364560a16707b9cc527e
- Full-file SHA-256: ec7a0a9d63d2bbd1549871c0e36b200336748a976c617ed593198bf5866a2872
- Full-file bytes: 193398
- Excerpt line span: 1205-1247
- Excerpt SHA-256: 35aed93786ab319a4e60d60c05e7337d646c9c1275a60ef0b197135b5cb8ba1f

  1205  extern "C" int Bun__handleUncaughtException(JSC::JSGlobalObject* lexicalGlobalObject, JSC::JSValue exception, int isRejection)
  1206  {
  1207      if (!lexicalGlobalObject->inherits(Zig::GlobalObject::info()))
  1208          return false;
  1209      auto* globalObject = uncheckedDowncast<Zig::GlobalObject>(lexicalGlobalObject);
  1210      auto* process = globalObject->processObject();
  1211      auto& wrapped = process->wrapped();
  1212      auto& vm = JSC::getVM(globalObject);
  1213
  1214      MarkedArgumentBuffer args;
  1215      args.append(exception);
  1216      if (isRejection) {
  1217          args.append(jsString(vm, String("unhandledRejection"_s)));
  1218      } else {
  1219          args.append(jsString(vm, String("uncaughtException"_s)));
  1220      }
  1221
  1222      auto uncaughtExceptionMonitor = Identifier::fromString(JSC::getVM(globalObject), "uncaughtExceptionMonitor"_s);
  1223      if (wrapped.listenerCount(uncaughtExceptionMonitor) > 0) {
  1224          wrapped.emit(uncaughtExceptionMonitor, args);
  1225      }
  1226
  1227      auto uncaughtExceptionIdent = Identifier::fromString(JSC::getVM(globalObject), "uncaughtException"_s);
  1228
  1229      // if there is an uncaughtExceptionCaptureCallback, call it and consider the exception handled
  1230      auto capture = process->getUncaughtExceptionCaptureCallback();
  1231      if (!capture.isEmpty() && !capture.isUndefinedOrNull()) {
  1232          auto scope = DECLARE_TOP_EXCEPTION_SCOPE(vm);
  1233          (void)call(lexicalGlobalObject, capture, args, "uncaughtExceptionCaptureCallback"_s);
  1234          if (auto ex = scope.exception()) {
  1235              (void)scope.tryClearException();
  1236              // if an exception is thrown in the uncaughtException handler, we abort
  1237              Bun__logUnhandledException(JSValue::encode(JSValue(ex)));
  1238              Bun__Process__exit(lexicalGlobalObject, 1);
  1239          }
  1240      } else if (wrapped.listenerCount(uncaughtExceptionIdent) > 0) {
  1241          wrapped.emit(uncaughtExceptionIdent, args);
  1242      } else {
  1243          return false;
  1244      }
  1245
  1246      return true;
  1247  }

## vendor/bun/src/jsc/bindings/BunProcess.cpp:3245-3263

- Full-file Git blob: 6935f0cbbcee9bf5f5f5364560a16707b9cc527e
- Full-file SHA-256: ec7a0a9d63d2bbd1549871c0e36b200336748a976c617ed593198bf5866a2872
- Full-file bytes: 193398
- Excerpt line span: 3245-3263
- Excerpt SHA-256: 226e3c10d64db64e1bb525d129ddf48c68cfc1198299d894e7c4fc7fd10262f2

  3245  JSC_DEFINE_HOST_FUNCTION(Process_functionReallyExit, (JSGlobalObject * globalObject, CallFrame* callFrame))
  3246  {
  3247      auto& vm = JSC::getVM(globalObject);
  3248      auto throwScope = DECLARE_THROW_SCOPE(vm);
  3249      uint8_t exitCode = 0;
  3250      JSValue arg0 = callFrame->argument(0);
  3251      if (arg0.isAnyInt()) {
  3252          exitCode = static_cast<uint8_t>(arg0.toInt32(globalObject) % 256);
  3253          RETURN_IF_EXCEPTION(throwScope, {});
  3254      }
  3255
  3256      auto* zigGlobal = defaultGlobalObject(globalObject);
  3257      Bun__Process__exit(zigGlobal, exitCode);
  3258      // Main-thread Bun__Process__exit is noreturn. In a worker it returns; the
  3259      // Zig WebWorker.exit() it called requests JSC termination (guarded so it's a
  3260      // no-op when re-entered from a process.on('exit') handler).
  3261      throwScope.release();
  3262      return JSC::JSValue::encode(jsUndefined());
  3263  }

## vendor/bun/src/jsc/bindings/webcore/Worker.cpp:350-430

- Full-file Git blob: 9c06ac70eed091ad22cdee195990c36ed1c5b438
- Full-file SHA-256: 4ece2800cc6d33a5b49064cbbb2d0c0d7e1f8f3b29d7eb859c47e0fe6094a9bb
- Full-file bytes: 32550
- Excerpt line span: 350-430
- Excerpt SHA-256: 7fd71ba1889d9d52cb1b4e6387ff0611e3ff41580207f1522f6e1e8b32e8cd64

   350          m_toParent.drainScheduled.store(false, std::memory_order_relaxed);
   351          return;
   352      }
   353      bool reschedule = drainInbox(m_toParent, globalObject, context, [&](Event& event) {
   354          dispatchEvent(event);
   355      });
   356      if (reschedule) {
   357          postTaskToParent([protectedThis = Ref { *this }](ScriptExecutionContext& c) {
   358              protectedThis->drainToParent(c);
   359          });
   360      }
   361  }
   362
   363  void Worker::terminate()
   364  {
   365      if (m_terminateRequested.exchange(true))
   366          return;
   367      WebWorker__notifyNeedTermination(impl_);
   368  }
   369
   370  void Worker::setKeepAlive(bool keepAlive)
   371  {
   372      // Once terminate() has been called or the close task has started, the
   373      // worker no longer participates in the parent's liveness — the close
   374      // task is the last thing to touch parent_poll_ref.
   375      if (m_terminateRequested.load() || m_state.load() >= State::Closing)
   376          return;
   377      WebWorker__setRef(impl_, keepAlive);
   378  }
   379
   380  void Worker::dispatchEvent(Event& event)
   381  {
   382      // Suppress user-visible events once terminate() has been called or the
   383      // worker has closed. The close event itself bypasses this (dispatchExit
   384      // calls EventTargetWithInlineData::dispatchEvent directly) so that
   385      // `await worker.terminate()` still resolves.
   386      if (m_terminateRequested.load() || m_state.load() == State::Closed)
   387          return;
   388      EventTargetWithInlineData::dispatchEvent(event);
   389  }
   390
   391  bool Worker::postTaskToWorkerGlobalScope(Function<void(ScriptExecutionContext&)>&& task)
   392  {
   393      {
   394          Locker lock(m_pendingTasksMutex);
   395          switch (m_state.load()) {
   396          case State::Pending:
   397              // Worker VM not up yet; queue for fireEarlyMessages().
   398              m_pendingTasks.append(WTF::move(task));
   399              return true;
   400          case State::Running:
   401              break;
   402          case State::Closing:
   403          case State::Closed:
   404              // Worker VM is gone; drop immediately (silent no-op).
   405              // postMessage() goes through enqueueToWorker(), not here — the
   406              // only user is getHeapSnapshot().
   407              return false;
   408          }
   409      }
   410      return ScriptExecutionContext::postTaskTo(m_clientIdentifier, WTF::move(task));
   411  }
   412
   413  // ---- Worker-thread entry points ---------------------------------------------
   414
   415  void Worker::dispatchOnline(Zig::GlobalObject* workerGlobalObject)
   416  {
   417      // Pending→Running under the same lock postTaskToWorkerGlobalScope uses, so
   418      // a message post racing this transition either queues (drained below by
   419      // fireEarlyMessages) or posts directly — never both, never neither.
   420      //
   421      // This MUST happen BEFORE the open event is posted to the parent: the
   422      // parent's `online` handler may immediately call getHeapSnapshot() (or
   423      // anything else gated on isOnline() / postTaskToWorkerGlobalScope()). If
   424      // the state flip happens after the post, a fast parent thread can run the
   425      // open task while m_state is still Pending and observe
   426      // ERR_WORKER_NOT_RUNNING — flaky `await once(worker, "online");
   427      // worker.getHeapSnapshot()` in worker_threads.test.ts.
   428      {
   429          Locker lock(m_pendingTasksMutex);
   430          m_state.store(State::Running);

## vendor/bun/src/jsc/bindings/vm/SigintWatcher.cpp:105-208

- Full-file Git blob: 93f2c42bd2b53369ada78a93a33b04fce0f0fdc3
- Full-file SHA-256: 5d0adef2743902412b1d8f378176374dba9f03ba0e98248dff555552a2d56369
- Full-file bytes: 4424
- Excerpt line span: 105-208
- Excerpt SHA-256: 1915377cb15a7e606d1a858a759dfa8502ed89a2a6eba89ce4793b359c02fac3

   105  }
   106
   107  void SigintWatcher::signalReceived()
   108  {
   109      if (!m_waiting.test_and_set()) {
   110          bool success = m_semaphore.signal();
   111          ASSERT(success);
   112      }
   113  }
   114
   115  void SigintWatcher::registerGlobalObject(JSGlobalObject* globalObject)
   116  {
   117      if (globalObject == nullptr) {
   118          return;
   119      }
   120
   121      WTF::Locker lock(m_globalObjectsMutex);
   122      m_globalObjects.appendIfNotContains(globalObject);
   123  }
   124
   125  void SigintWatcher::unregisterGlobalObject(JSGlobalObject* globalObject)
   126  {
   127      if (globalObject == nullptr) {
   128          return;
   129      }
   130
   131      WTF::Locker lock(m_globalObjectsMutex);
   132
   133      auto iter = std::find(m_globalObjects.begin(), m_globalObjects.end(), globalObject);
   134      if (iter == m_globalObjects.end()) {
   135          return;
   136      }
   137
   138      std::swap(*iter, m_globalObjects.last());
   139      m_globalObjects.removeLast();
   140  }
   141
   142  void SigintWatcher::registerReceiver(SigintReceiver* module)
   143  {
   144      if (module == nullptr) {
   145          return;
   146      }
   147
   148      WTF::Locker lock(m_receiversMutex);
   149      m_receivers.appendIfNotContains(module);
   150  }
   151
   152  void SigintWatcher::unregisterReceiver(SigintReceiver* module)
   153  {
   154      WTF::Locker lock(m_receiversMutex);
   155
   156      auto iter = std::find(m_receivers.begin(), m_receivers.end(), module);
   157      if (iter == m_receivers.end()) {
   158          return;
   159      }
   160
   161      std::swap(*iter, m_receivers.last());
   162      m_receivers.removeLast();
   163  }
   164
   165  void SigintWatcher::ref()
   166  {
   167      if (m_refCount++ == 0) {
   168          install();
   169      }
   170  }
   171
   172  void SigintWatcher::deref()
   173  {
   174      ASSERT(m_refCount > 0);
   175      if (--m_refCount == 0) {
   176          uninstall();
   177      }
   178  }
   179
   180  SigintWatcher& SigintWatcher::get()
   181  {
   182      static SigintWatcher instance;
   183      return instance;
   184  }
   185
   186  bool SigintWatcher::signalAll()
   187  {
   188      {
   189          WTF::Locker lock(m_receiversMutex);
   190          for (auto* receiver : m_receivers) {
   191              receiver->setSigintReceived();
   192          }
   193      }
   194
   195      WTF::Locker lock(m_globalObjectsMutex);
   196
   197      if (m_globalObjects.isEmpty()) {
   198          return false;
   199      }
   200
   201      for (JSGlobalObject* globalObject : m_globalObjects) {
   202          globalObject->vm().notifyNeedTermination();
   203      }
   204
   205      return true;
   206  }
   207
   208  } // namespace Bun

## vendor/bun/src/jsc/bindings/NodeVM.cpp:847-870

- Full-file Git blob: dae4e540c9da7a8a974df664e9140162b514c7a4
- Full-file SHA-256: c90f9095225c82af0fc7c4c275b872d212854595ca52fd99d0c69d8dd153ba4f
- Full-file bytes: 80305
- Excerpt line span: 847-870
- Excerpt SHA-256: 64dd32b01d39b2d2839f8a0e17f90442b4450e7e0296d35f158b6e839b9ea102

   847  void NodeVMGlobalObject::destroy(JSCell* cell)
   848  {
   849      static_cast<NodeVMGlobalObject*>(cell)->~NodeVMGlobalObject();
   850  }
   851
   852  NodeVMGlobalObject::~NodeVMGlobalObject()
   853  {
   854      SigintWatcher::get().unregisterGlobalObject(this);
   855  }
   856
   857  void NodeVMGlobalObject::setContextifiedObject(JSC::JSObject* contextifiedObject)
   858  {
   859      m_sandbox.set(vm(), this, contextifiedObject);
   860  }
   861
   862  void NodeVMGlobalObject::clearContextifiedObject()
   863  {
   864      m_sandbox.clear();
   865  }
   866
   867  void NodeVMGlobalObject::sigintReceived()
   868  {
   869      vm().notifyNeedTermination();
   870  }

## vendor/bun/src/jsc/bindings/NodeVMScript.cpp:278-300

- Full-file Git blob: d7cfa6157cbd91ba454848573318869ecd837f27
- Full-file SHA-256: 2bc8b85e761e0cafd3c2b9f18628dd78db6d34f9cff32d2934525017663e8c56
- Full-file bytes: 27536
- Excerpt line span: 278-300
- Excerpt SHA-256: d56672c588052cc656ebaba8810c496dc8b01122c0428d87d75063ac1d446be2

   278  void NodeVMScript::destroy(JSCell* cell)
   279  {
   280      static_cast<NodeVMScript*>(cell)->NodeVMScript::~NodeVMScript();
   281  }
   282
   283  static bool checkForTermination(JSC::VM& vm, JSC::JSGlobalObject* globalObject, JSC::ThrowScope& scope, NodeVMScript* script, std::optional<double> timeout)
   284  {
   285      if (vm.hasTerminationRequest()) {
   286          vm.drainMicrotasksForGlobalObject(globalObject);
   287          vm.clearHasTerminationRequest();
   288          if (script->getSigintReceived()) {
   289              script->setSigintReceived(false);
   290              throwError(globalObject, scope, ErrorCode::ERR_SCRIPT_EXECUTION_INTERRUPTED, "Script execution was interrupted by `SIGINT`"_s);
   291          } else if (timeout) {
   292              throwError(globalObject, scope, ErrorCode::ERR_SCRIPT_EXECUTION_TIMEOUT, makeString("Script execution timed out after "_s, *timeout, "ms"_s));
   293          } else {
   294              RELEASE_ASSERT_NOT_REACHED_WITH_MESSAGE("vm.Script terminated due neither to SIGINT nor to timeout");
   295          }
   296          return true;
   297      }
   298
   299      return false;
   300  }

## vendor/bun/src/jsc/bindings/NodeVMModule.cpp:52-151

- Full-file Git blob: bba5326eb0512ce4f6fb6e0d73b23dbb39592dee
- Full-file SHA-256: b9e58e4bc00c8d01d98ca3991fdacfda2f59c8f99351aaca2432596528309ce2
- Full-file bytes: 24625
- Excerpt line span: 52-151
- Excerpt SHA-256: 4bd9e8ff0268b4110a337daf7b9c1f3adfbcb95302d087d4be7bdbf311b3361a

    52  JSValue NodeVMModule::evaluate(JSGlobalObject* globalObject, uint32_t timeout, bool breakOnSigint)
    53  {
    54      VM& vm = globalObject->vm();
    55      auto scope = DECLARE_THROW_SCOPE(vm);
    56
    57      if (m_status != Status::Linked && m_status != Status::Evaluated && m_status != Status::Errored) {
    58          throwError(globalObject, scope, ErrorCode::ERR_VM_MODULE_STATUS, "Module must be linked, evaluated or errored before evaluating"_s);
    59          return {};
    60      }
    61
    62      if (m_status == Status::Evaluated) {
    63          return m_evaluationResult.get();
    64      }
    65
    66      auto* sourceTextThis = dynamicDowncast<NodeVMSourceTextModule>(this);
    67      auto* syntheticThis = dynamicDowncast<NodeVMSyntheticModule>(this);
    68
    69  #define VM_RETURN_IF_EXCEPTION(scope__, value__)                                                \
    70      do {                                                                                        \
    71          if (JSC::Exception* exception = scope__.exception()) {                                  \
    72              status(Status::Errored);                                                            \
    73              if (sourceTextThis) sourceTextThis->m_evaluationException.set(vm, this, exception); \
    74              return value__;                                                                     \
    75          }                                                                                       \
    76      } while (false);
    77
    78      AbstractModuleRecord* record {};
    79      if (sourceTextThis) {
    80          record = sourceTextThis->moduleRecord(globalObject);
    81          VM_RETURN_IF_EXCEPTION(scope, {});
    82      } else if (syntheticThis) {
    83          record = syntheticThis->moduleRecord(globalObject);
    84          VM_RETURN_IF_EXCEPTION(scope, {});
    85      } else {
    86          RELEASE_ASSERT_NOT_REACHED_WITH_MESSAGE("Invalid module type");
    87      }
    88
    89      JSValue result {};
    90
    91      NodeVMGlobalObject* nodeVmGlobalObject = NodeVM::getGlobalObjectFromContext(globalObject, m_context.get(), false);
    92      VM_RETURN_IF_EXCEPTION(scope, {});
    93      if (nodeVmGlobalObject) globalObject = nodeVmGlobalObject;
    94
    95      auto run = [&] {
    96          if (sourceTextThis) {
    97              status(Status::Evaluating);
    98              evaluateDependencies(globalObject, record, timeout, breakOnSigint);
    99              RETURN_IF_EXCEPTION(scope, );
   100              sourceTextThis->initializeImportMeta(globalObject);
   101              RETURN_IF_EXCEPTION(scope, );
   102          } else if (syntheticThis) {
   103              syntheticThis->evaluate(globalObject);
   104              RETURN_IF_EXCEPTION(scope, );
   105          }
   106          result = record->evaluate(globalObject, jsUndefined(), jsNumber(static_cast<int32_t>(JSGenerator::ResumeMode::NormalMode)));
   107          RETURN_IF_EXCEPTION(scope, );
   108      };
   109
   110      setSigintReceived(false);
   111
   112      std::optional<double> oldLimit, newLimit;
   113
   114      if (timeout != 0) {
   115          setupWatchdog(vm, timeout, &oldLimit.emplace(), &newLimit.emplace());
   116      }
   117
   118      if (breakOnSigint) {
   119          auto holder = SigintWatcher::hold(nodeVmGlobalObject, this);
   120          run();
   121      } else {
   122          run();
   123      }
   124
   125      if (timeout != 0) {
   126          vm.watchdog()->setTimeLimit(WTF::Seconds::fromMilliseconds(*oldLimit));
   127      }
   128
   129      if (vm.hasPendingTerminationException()) {
   130          vm.drainMicrotasksForGlobalObject(nodeVmGlobalObject);
   131          DECLARE_TOP_EXCEPTION_SCOPE(vm).clearException();
   132          vm.clearHasTerminationRequest();
   133          if (getSigintReceived()) {
   134              setSigintReceived(false);
   135              throwError(globalObject, scope, ErrorCode::ERR_SCRIPT_EXECUTION_INTERRUPTED, "Script execution was interrupted by `SIGINT`"_s);
   136          } else if (timeout != 0) {
   137              throwError(globalObject, scope, ErrorCode::ERR_SCRIPT_EXECUTION_TIMEOUT, makeString("Script execution timed out after "_s, timeout, "ms"_s));
   138          } else {
   139              RELEASE_ASSERT_NOT_REACHED_WITH_MESSAGE("vm.SourceTextModule evaluation terminated due neither to SIGINT nor to timeout");
   140          }
   141      } else {
   142          setSigintReceived(false);
   143      }
   144
   145      VM_RETURN_IF_EXCEPTION(scope, {});
   146
   147      status(Status::Evaluated);
   148      m_evaluationResult.set(vm, this, result);
   149      return result;
   150  #undef VM_RETURN_IF_EXCEPTION
   151  }
